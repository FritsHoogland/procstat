
use log::debug;
use tokio::time::{self, MissedTickBehavior};
use tokio::time::Duration;
use chrono::{DateTime, Local, TimeZone};
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
use std::fs::write;
use std::path::Path;

pub async fn archiver() {
    let mut interval = time::interval(Duration::from_secs(60));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut archive_time = Local::now();
    #[allow(unused_assignments)]
    let mut current_time = Local::now();

    loop {
        interval.tick().await;
        current_time = Local::now();

        debug!("archiver tick");
        if current_time.format("%H").to_string() != archive_time.format("%H").to_string() {
            archive(archive_time);
            archive_time = current_time;
        };
    }
}
pub fn archive(archive_time: DateTime<Local>) {
    let mut transition = HistoricalDataTransit::default();
    let low_timestamp: DateTime<Local> = Local
        .with_ymd_and_hms(
            archive_time.format("%Y").to_string().parse::<i32>().unwrap(), 
            archive_time.format("%m").to_string().parse::<u32>().unwrap(),  
            archive_time.format("%d").to_string().parse::<u32>().unwrap(), 
            archive_time.format("%H").to_string().parse::<u32>().unwrap(), 
            00,
            00)
        .unwrap();
    let high_timestamp: DateTime<Local> = low_timestamp.clone()+Duration::from_secs(3600);
    println!("low: {:?}, high: {:?}", low_timestamp, high_timestamp);

    transition.cpu = HISTORY.cpu.read().unwrap().iter().filter(|cpustat| cpustat.timestamp > low_timestamp && cpustat.timestamp <= high_timestamp).cloned().collect::<Vec<CpuStat>>();
    transition.memory = HISTORY.memory.read().unwrap().iter().filter(|memory| memory.timestamp > low_timestamp && memory.timestamp <= high_timestamp).cloned().collect::<Vec<MemInfo>>();
    transition.blockdevices = HISTORY.blockdevices.read().unwrap().iter().filter(|blockdevices| blockdevices.timestamp > low_timestamp && blockdevices.timestamp <= high_timestamp).cloned().collect::<Vec<BlockDeviceInfo>>();
    transition.networkdevices = HISTORY.networkdevices.read().unwrap().iter().filter(|networkdevices| networkdevices.timestamp > low_timestamp && networkdevices.timestamp <= high_timestamp).cloned().collect::<Vec<NetworkDeviceInfo>>();
    transition.loadavg = HISTORY.loadavg.read().unwrap().iter().filter(|loadavg| loadavg.timestamp > low_timestamp && loadavg.timestamp <= high_timestamp).cloned().collect::<Vec<LoadavgInfo>>();
    transition.pressure = HISTORY.pressure.read().unwrap().iter().filter(|pressure| pressure.timestamp > low_timestamp && pressure.timestamp <= high_timestamp).cloned().collect::<Vec<PressureInfo>>();
    transition.vmstat = HISTORY.vmstat.read().unwrap().iter().filter(|vmstat| vmstat.timestamp > low_timestamp && vmstat.timestamp <= high_timestamp).cloned().collect::<Vec<VmStatInfo>>();

    let current_directory = current_dir().unwrap();
    let mut filename = current_directory.join(format!("{}-{}-{}T{}", archive_time.format("%Y"), archive_time.format("%m"), archive_time.format("%d"), archive_time.format("%H")));
    if Path::new(&filename).try_exists().is_ok() {
        filename = current_directory.join(format!("{}-{}-{}T{}-{}", archive_time.format("%Y"), archive_time.format("%m"), archive_time.format("%d"), archive_time.format("%H"), archive_time.format("%M")));
        if Path::new(&filename).try_exists().is_ok() {
            filename = current_directory.join(format!("{}-{}-{}T{}-{}-{}", archive_time.format("%Y"), archive_time.format("%m"), archive_time.format("%d"), archive_time.format("%H"), archive_time.format("%M"), archive_time.format("%S")));
            if Path::new(&filename).try_exists().is_ok() {
                panic!("File: {:?} already exists. Too quick archive retry.", filename);
            };
        };
    };
    write(filename, serde_json::to_string(&transition).unwrap()).unwrap();
}

pub fn reader(filenames: String) {
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
}
