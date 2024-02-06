
use log::debug;
use tokio::time::{self, MissedTickBehavior};
use tokio::time::Duration as TokioDuration;
use chrono::{Duration, DurationRound};
use chrono::{DateTime, Local};
use crate::common::HistoricalDataTransit;
use crate::HISTORY;
use crate::stat::CpuStat;
use crate::meminfo::MemInfo;
use crate::blockdevice::BlockDeviceInfo;
use crate::loadavg::LoadavgInfo;
use crate::pressure::PressureInfo;
use crate::net_dev::NetworkDeviceInfo;
use crate::vmstat::VmStatInfo;
use std::env::current_dir;
use std::path::Path;
use std::fs::write;
use crate::ARGS;

pub async fn archiver() {
    let mut interval = time::interval(TokioDuration::from_secs(60));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut archive_time = Local::now().duration_trunc(Duration::minutes(ARGS.archiver_interval)).unwrap()+Duration::minutes(ARGS.archiver_interval);
    debug!("begin: time: archive_time: {:?}, current_time: {:?}", archive_time, Local::now());
    loop {
        interval.tick().await;

        debug!("archiver tick");
        if Local::now() > archive_time {
            archive(archive_time);
            archive_time += Duration::minutes(ARGS.archiver_interval);
            debug!("new archive_time: {:?}", archive_time);
        };
    }
}
pub fn archive(high_time: DateTime<Local>) { let mut transition = HistoricalDataTransit::default();
    let low_time = high_time-Duration::minutes(ARGS.archiver_interval);
    debug!("archive times: low: {:?}, high {:?}, interval: {:?}", low_time, high_time, ARGS.archiver_interval);

    transition.cpu = HISTORY.cpu.read().unwrap().iter().filter(|cpustat| cpustat.timestamp > low_time && cpustat.timestamp <= high_time).cloned().collect::<Vec<CpuStat>>();
    transition.memory = HISTORY.memory.read().unwrap().iter().filter(|memory| memory.timestamp > low_time && memory.timestamp <= high_time).cloned().collect::<Vec<MemInfo>>();
    transition.blockdevices = HISTORY.blockdevices.read().unwrap().iter().filter(|blockdevices| blockdevices.timestamp > low_time && blockdevices.timestamp <= high_time).cloned().collect::<Vec<BlockDeviceInfo>>();
    transition.networkdevices = HISTORY.networkdevices.read().unwrap().iter().filter(|networkdevices| networkdevices.timestamp > low_time && networkdevices.timestamp <= high_time).cloned().collect::<Vec<NetworkDeviceInfo>>();
    transition.loadavg = HISTORY.loadavg.read().unwrap().iter().filter(|loadavg| loadavg.timestamp > low_time && loadavg.timestamp <= high_time).cloned().collect::<Vec<LoadavgInfo>>();
    transition.pressure = HISTORY.pressure.read().unwrap().iter().filter(|pressure| pressure.timestamp > low_time && pressure.timestamp <= high_time).cloned().collect::<Vec<PressureInfo>>();
    transition.vmstat = HISTORY.vmstat.read().unwrap().iter().filter(|vmstat| vmstat.timestamp > low_time && vmstat.timestamp <= high_time).cloned().collect::<Vec<VmStatInfo>>();

    let current_directory = current_dir().unwrap();
    let filename = current_directory.join(format!("procstat_{}-{}-{}T{}-{}.json", low_time.format("%Y"), low_time.format("%m"), low_time.format("%d"), low_time.format("%H"), low_time.format("%M")));
    debug!("filename: {:?}", filename.to_str());
    write(filename, serde_json::to_string(&transition).unwrap()).unwrap();
}

pub async fn reader(filenames: String) {
    filenames.split(',').for_each(|file| {
        if Path::new(&file).try_exists().is_ok() {
            let transition: HistoricalDataTransit = serde_json::from_str(&std::fs::read_to_string(file).unwrap()).unwrap_or_else(|e| panic!("{}", e));
            transition.cpu.iter().for_each(|row| { HISTORY.cpu.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.memory.iter().for_each(|row| { HISTORY.memory.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.blockdevices.iter().for_each(|row| { HISTORY.blockdevices.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.networkdevices.iter().for_each(|row| { HISTORY.networkdevices.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.loadavg.iter().for_each(|row| { HISTORY.loadavg.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.pressure.iter().for_each(|row| { HISTORY.pressure.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            transition.vmstat.iter().for_each(|row| { HISTORY.vmstat.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
            println!("File: {} loaded", &file)
        } else {
            println!("Warning! File: {} cannot be found!", file);
        };
    });
    println!("All files loaded.");

    // this sets up an endless loop
    let mut interval = time::interval(std::time::Duration::from_secs(ARGS.interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
    };
}

