use anyhow::{Context, Result};
use chrono::{DateTime, Duration, DurationRound, Local};
use log::debug;
use std::env::current_dir;
use std::fs::write;
use std::path::Path;
use tokio::time::{self, Duration as TokioDuration, MissedTickBehavior};

use crate::processor::blockdevice::BlockDeviceInfo;
use crate::processor::loadavg::LoadavgInfo;
use crate::processor::meminfo::MemInfo;
use crate::processor::net_dev::NetworkDeviceInfo;
use crate::processor::pressure::PressureInfo;
use crate::processor::stat::CpuStat;
use crate::processor::vmstat::VmStatInfo;
use crate::processor::xfs::XfsInfo;
use crate::processor::HistoricalDataTransit;
use crate::{ARGS, HISTORY};

pub async fn archiver() -> Result<()> {
    // regardless of the archiver_interval set, the archiver will tick once per minute.
    let mut interval = time::interval(TokioDuration::from_secs(60));
    // by default, if the archiver thread missed a tick because it couldn't run/get on cpu,
    // it will still perform the ticks leading to a tick avalance. MissedTickBehavior::Skip
    // prevents this.
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    // archive_time is the archiver "high_time".
    //
    // the magic is in the duration_on_trunc function, which sets the time truncated by full
    // "units" of the duration, so if set to 10 minutes and the time is 12:22:33, it will be
    // trunced to 12:20:00 (in the past).
    //
    // once trunced, the interval is added, setting it to a time in the future, hence "high time".
    let mut high_time = Local::now().duration_trunc(Duration::minutes(ARGS.archiver_interval))?
        + Duration::minutes(ARGS.archiver_interval);

    debug!(
        "begin: current_time: {:?}, high_time: {:?}",
        Local::now(),
        high_time
    );

    loop {
        interval.tick().await;

        debug!("archiver tick");
        if Local::now() > high_time {
            match archive(high_time, true) {
                Ok(_) => {}
                Err(error) => {
                    // if the archiver returns an error, return to the caller of the archiver
                    // function.
                    // an error is encountered if the archiver can't write, which beats the purpose
                    // of the archiver, so we return to the caller of the archiver function with an
                    // error.
                    return Err(error);
                }
            };
            // now that the archiver has performed its work,
            // increase the high_time with the set interval.
            high_time += Duration::minutes(ARGS.archiver_interval);
            debug!("new high_time: {:?}", high_time);
        };
    }
}
pub fn archive(high_time: DateTime<Local>, interval_completed: bool) -> Result<()> {
    let mut transition = HistoricalDataTransit::default();
    // this function gets the "end" time of the data to archive,
    // so subtracting the interval will result in the begin time for the archive.
    //let low_time = high_time.duration_trunc(Duration::minutes(ARGS.archiver_interval))?
    //-Duration::minutes(ARGS.archiver_interval);
    let low_time = if interval_completed {
        (high_time - Duration::minutes(ARGS.archiver_interval))
            .duration_trunc(Duration::minutes(ARGS.archiver_interval))?
    } else {
        high_time.duration_trunc(Duration::minutes(ARGS.archiver_interval))?
    };

    debug!(
        "archive times: low: {:?}, high {:?}, interval: {:?} minutes.",
        low_time, high_time, ARGS.archiver_interval
    );

    transition.cpu = HISTORY
        .cpu
        .read()
        .unwrap()
        .iter()
        .filter(|cpustat| cpustat.timestamp > low_time && cpustat.timestamp <= high_time)
        .cloned()
        .collect::<Vec<CpuStat>>();
    transition.memory = HISTORY
        .memory
        .read()
        .unwrap()
        .iter()
        .filter(|memory| memory.timestamp > low_time && memory.timestamp <= high_time)
        .cloned()
        .collect::<Vec<MemInfo>>();
    transition.blockdevices = HISTORY
        .blockdevices
        .read()
        .unwrap()
        .iter()
        .filter(|blockdevices| {
            blockdevices.timestamp > low_time && blockdevices.timestamp <= high_time
        })
        .cloned()
        .collect::<Vec<BlockDeviceInfo>>();
    transition.networkdevices = HISTORY
        .networkdevices
        .read()
        .unwrap()
        .iter()
        .filter(|networkdevices| {
            networkdevices.timestamp > low_time && networkdevices.timestamp <= high_time
        })
        .cloned()
        .collect::<Vec<NetworkDeviceInfo>>();
    transition.loadavg = HISTORY
        .loadavg
        .read()
        .unwrap()
        .iter()
        .filter(|loadavg| loadavg.timestamp > low_time && loadavg.timestamp <= high_time)
        .cloned()
        .collect::<Vec<LoadavgInfo>>();
    transition.pressure = HISTORY
        .pressure
        .read()
        .unwrap()
        .iter()
        .filter(|pressure| pressure.timestamp > low_time && pressure.timestamp <= high_time)
        .cloned()
        .collect::<Vec<PressureInfo>>();
    transition.vmstat = HISTORY
        .vmstat
        .read()
        .unwrap()
        .iter()
        .filter(|vmstat| vmstat.timestamp > low_time && vmstat.timestamp <= high_time)
        .cloned()
        .collect::<Vec<VmStatInfo>>();
    transition.xfs = HISTORY
        .xfs
        .read()
        .unwrap()
        .iter()
        .filter(|xfs| xfs.timestamp > low_time && xfs.timestamp <= high_time)
        .cloned()
        .collect::<Vec<XfsInfo>>();

    let current_directory = current_dir()?;
    let filename = current_directory.join(format!(
        "procstat_{}-{}-{}T{}-{}.json",
        high_time.format("%Y"),
        high_time.format("%m"),
        high_time.format("%d"),
        high_time.format("%H"),
        high_time.format("%M")
    ));
    debug!("filename: {:?}", &filename.to_str());
    // the most likely place to fail is the write.
    write(filename.clone(), serde_json::to_string(&transition)?).with_context(|| {
        format!(
            "Error writing {} to {}.",
            filename.to_string_lossy(),
            current_directory.to_string_lossy()
        )
    })?;

    Ok(())
}

pub async fn reader(filenames: String) {
    filenames.split(',').for_each(|file| {
        if Path::new(&file).exists() {
            let transition: HistoricalDataTransit =
                serde_json::from_str(&std::fs::read_to_string(file).unwrap())
                    .unwrap_or_else(|e| panic!("{}", e));
            transition.cpu.iter().for_each(|row| {
                HISTORY
                    .cpu
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.memory.iter().for_each(|row| {
                HISTORY
                    .memory
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.blockdevices.iter().for_each(|row| {
                HISTORY
                    .blockdevices
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.networkdevices.iter().for_each(|row| {
                HISTORY
                    .networkdevices
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.loadavg.iter().for_each(|row| {
                HISTORY
                    .loadavg
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.pressure.iter().for_each(|row| {
                HISTORY
                    .pressure
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.vmstat.iter().for_each(|row| {
                HISTORY
                    .vmstat
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            transition.xfs.iter().for_each(|row| {
                HISTORY
                    .xfs
                    .write()
                    .unwrap()
                    .push_back(row.clone())
                    .unwrap_or_default();
            });
            println!("✔ {}", &file);
        } else {
            println!("✘ {}", file);
        };
    });
    println!("All files loaded.");

    // this sets up an endless loop that ticks with the set interval.
    let mut interval = time::interval(std::time::Duration::from_secs(ARGS.interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
    }
}
