use crate::stat::CpuStat;
use crate::HISTORY;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use bounded_vec_deque::BoundedVecDeque;
use std::sync::RwLock;
use log::debug;
use std::env::current_dir;
use std::fs::write;

use crate::stat::{add_cpu_total_to_history, process_stat_data};
use crate::schedstat::process_schedstat_data;
use crate::meminfo::{add_memory_to_history, MemInfo, process_meminfo_data};
use crate::blockdevice::{add_blockdevices_to_history, BlockDeviceInfo, process_blockdevice_data};
use crate::loadavg::{add_loadavg_to_history, LoadavgInfo, process_loadavg_data};
use crate::pressure::{add_pressure_to_history, PressureInfo, process_pressure_data};
use crate::net_dev::{add_networkdevices_to_history, NetworkDeviceInfo, process_net_dev_data};
use crate::vmstat::{add_vmstat_to_history, VmStatInfo, process_vmstat_data};
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct ProcData {
    pub timestamp: DateTime<Local>,
    pub stat: proc_sys_parser::stat::ProcStat,
    pub schedstat: proc_sys_parser::schedstat::ProcSchedStat,
    pub meminfo: proc_sys_parser::meminfo::ProcMemInfo,
    pub blockdevices: proc_sys_parser::block::SysBlock,
    pub net_dev: proc_sys_parser::net_dev::ProcNetDev,
    pub loadavg: proc_sys_parser::loadavg::ProcLoadavg,
    pub pressure: proc_sys_parser::pressure::ProcPressure,
    pub vmstat: proc_sys_parser::vmstat::ProcVmStat,
}

#[derive(Debug, Default)]
pub struct Statistic {
    pub last_timestamp: DateTime<Local>,
    pub last_value: f64,
    pub delta_value: f64,
    pub per_second_value: f64,
    pub updated_value: bool,
}

#[derive(Debug)]
pub struct HistoricalData {
    pub cpu: RwLock<BoundedVecDeque<CpuStat>>,
    pub memory: RwLock<BoundedVecDeque<MemInfo>>,
    pub blockdevices: RwLock<BoundedVecDeque<BlockDeviceInfo>>,
    pub networkdevices: RwLock<BoundedVecDeque<NetworkDeviceInfo>>,
    pub loadavg: RwLock<BoundedVecDeque<LoadavgInfo>>,
    pub pressure: RwLock<BoundedVecDeque<PressureInfo>>,
    pub vmstat: RwLock<BoundedVecDeque<VmStatInfo>>,
}

impl HistoricalData {
    pub fn new(history: usize) -> HistoricalData {
        HistoricalData {
            cpu: RwLock::new(BoundedVecDeque::new(history)),
            memory: RwLock::new(BoundedVecDeque::new(history)),
            blockdevices: RwLock::new(BoundedVecDeque::new(history)),
            networkdevices: RwLock::new(BoundedVecDeque::new(history)),
            loadavg: RwLock::new(BoundedVecDeque::new(history)),
            pressure: RwLock::new(BoundedVecDeque::new(history)),
            vmstat: RwLock::new(BoundedVecDeque::new(history)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HistoricalDataTransit {
    pub cpu: Vec<CpuStat>,
    pub memory: Vec<MemInfo>,
    pub blockdevices: Vec<BlockDeviceInfo>,
    pub networkdevices: Vec<NetworkDeviceInfo>,
    pub loadavg: Vec<LoadavgInfo>,
    pub pressure: Vec<PressureInfo>,
    pub vmstat: Vec<VmStatInfo>,
}

pub async fn read_proc_data_and_process(statistics: &mut HashMap<(String, String, String), Statistic>) {
    let timestamp = Local::now();
    let proc_stat = proc_sys_parser::stat::read();
    debug!("Stat: {:?}", proc_stat);
    let proc_schedstat = proc_sys_parser::schedstat::read();
    debug!("Schedstat: {:?}", proc_schedstat);
    let proc_meminfo = proc_sys_parser::meminfo::read();
    debug!("Meminfo: {:?}", proc_meminfo);
    let sys_block_devices = proc_sys_parser::block::read();
    debug!("Block: {:?}", sys_block_devices);
    let proc_netdev = proc_sys_parser::net_dev::read();
    debug!("Netdev: {:?}", proc_netdev);
    let proc_loadavg = proc_sys_parser::loadavg::read();
    debug!("Loadavg: {:?}", proc_loadavg);
    let proc_pressure = proc_sys_parser::pressure::read();
    debug!("Pressure: {:?}", proc_pressure);
    let proc_vmstat = proc_sys_parser::vmstat::read();
    debug!("Vmstat: {:?}", proc_vmstat);
    let proc_data = ProcData {
        timestamp,
        stat: proc_stat,
        schedstat: proc_schedstat,
        meminfo: proc_meminfo,
        blockdevices: sys_block_devices,
        net_dev: proc_netdev,
        loadavg: proc_loadavg,
        pressure: proc_pressure,
        vmstat: proc_vmstat,
    };
    process_data(proc_data, statistics).await;
    add_to_history(statistics).await;
}

pub async fn process_data(proc_data: ProcData, statistics: &mut HashMap<(String, String, String), Statistic>) {
    process_stat_data(&proc_data, statistics).await;
    process_schedstat_data(&proc_data, statistics).await;
    process_meminfo_data(&proc_data, statistics).await;
    process_blockdevice_data(&proc_data, statistics).await;
    process_net_dev_data(&proc_data, statistics).await;
    process_loadavg_data(&proc_data, statistics).await;
    process_pressure_data(&proc_data, statistics).await;
    process_vmstat_data(&proc_data, statistics).await;
}

pub async fn add_to_history(statistics: &HashMap<(String, String, String), Statistic>) {
    // history management happens here
    add_cpu_total_to_history(statistics).await;
    add_memory_to_history(statistics).await;
    add_blockdevices_to_history(statistics).await;
    add_networkdevices_to_history(statistics).await;
    add_loadavg_to_history(statistics).await;
    add_pressure_to_history(statistics).await;
    add_vmstat_to_history(statistics).await;
}

pub fn save_history() {
    let mut transition = HistoricalDataTransit::default();
    transition.cpu = HISTORY.cpu.read().unwrap().iter().cloned().collect::<Vec<CpuStat>>();
    transition.memory = HISTORY.memory.read().unwrap().iter().cloned().collect::<Vec<MemInfo>>();
    transition.blockdevices = HISTORY.blockdevices.read().unwrap().iter().cloned().collect::<Vec<BlockDeviceInfo>>();
    transition.networkdevices = HISTORY.networkdevices.read().unwrap().iter().cloned().collect::<Vec<NetworkDeviceInfo>>();
    transition.loadavg = HISTORY.loadavg.read().unwrap().iter().cloned().collect::<Vec<LoadavgInfo>>();
    transition.pressure = HISTORY.pressure.read().unwrap().iter().cloned().collect::<Vec<PressureInfo>>();
    transition.vmstat = HISTORY.vmstat.read().unwrap().iter().cloned().collect::<Vec<VmStatInfo>>();

    let current_directory = current_dir().unwrap();
    let filename = current_directory.join("procstat.json");
    write(filename, serde_json::to_string(&transition).unwrap()).unwrap();
}

pub fn read_history() {
    let current_directory = current_dir().unwrap();
    let filename = current_directory.join("procstat.json");
    let transition: HistoricalDataTransit = serde_json::from_str(&std::fs::read_to_string(filename).unwrap()).unwrap_or_else(|e| panic!("{}", e));
    //let mut cpu= HISTORY.cpu.write().unwrap();
    //*cpu = transition.cpu.iter().cloned().collect::<Vec<CpuStat>>().into();
    //*cpu = BoundedVecDeque::from(transition.cpu.try_into().unwrap())
    transition.cpu.iter().for_each(|row| { HISTORY.cpu.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.memory.iter().for_each(|row| { HISTORY.memory.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.blockdevices.iter().for_each(|row| { HISTORY.blockdevices.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.networkdevices.iter().for_each(|row| { HISTORY.networkdevices.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.loadavg.iter().for_each(|row| { HISTORY.loadavg.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.pressure.iter().for_each(|row| { HISTORY.pressure.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    transition.vmstat.iter().for_each(|row| { HISTORY.vmstat.write().unwrap().push_back(row.clone()).unwrap_or_default(); });
    //transition.cpu.iter().for_each(|row| println!("{:?}", row));
    //println!("{:?}", HISTORY);
}

pub async fn single_statistic_u64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: u64,
    statistics: &mut HashMap<(String, String, String), Statistic>,
)
{
    statistics.entry((category.to_string(), subcategory.to_string(), name.to_string()))
        .and_modify(|row| {
            row.delta_value = value as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = value as f64;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() { row.per_second_value = 0_f64 }
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: value as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                updated_value: false,
            }
        );
}
pub async fn single_statistic_f64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: f64,
    statistics: &mut HashMap<(String, String, String), Statistic>,
)
{
    statistics.entry((category.to_string(), subcategory.to_string(), name.to_string()))
        .and_modify(|row| {
            row.delta_value = value - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = value;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() { row.per_second_value = 0_f64 }
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: value,
                delta_value: 0.0,
                per_second_value: 0.0,
                updated_value: false,
            }
        );
}
pub async fn single_statistic_option_f64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: Option<f64>,
    statistics: &mut HashMap<(String, String, String), Statistic>,
)
{
    let value = value.unwrap_or_default();
    statistics.entry((category.to_string(), subcategory.to_string(), name.to_string()))
        .and_modify(|row| {
            row.delta_value = value - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = value;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() { row.per_second_value = 0_f64 }
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: value,
                delta_value: 0.0,
                per_second_value: 0.0,
                updated_value: false,
            }
        );
}
pub async fn single_statistic_option_u64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: Option<u64>,
    statistics: &mut HashMap<(String, String, String), Statistic>,
)
{
    let value = value.unwrap_or_default();
    statistics.entry((category.to_string(), subcategory.to_string(), name.to_string()))
        .and_modify(|row| {
            row.delta_value = value as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = value as f64;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() { row.per_second_value = 0_f64 }
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: value as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                updated_value: false,
            }
        );
}

#[macro_export]
macro_rules! add_list_of_u64_data_to_statistics {
    ($category:expr, $subcategory:expr, $timestamp:expr, $proc:ident, $proc_struct:ident, $statistics:ident, $($field_name:ident),*) => {
        $(
            let subcategory = if stringify!($subcategory) == "\"\"" { "" } else  { stringify!($subcategory) };
            single_statistic_u64(stringify!($category), subcategory , stringify!($field_name), $timestamp, $proc.$proc_struct.$field_name, $statistics).await;
        )*
    };
}

#[macro_export]
macro_rules! add_list_of_option_u64_data_to_statistics {
    ($category:expr, $subcategory:expr, $timestamp:expr, $proc:ident, $proc_struct:ident, $statistics:ident, $($field_name:ident),*) => {
        $(
            let subcategory = if stringify!($subcategory) == "\"\"" { "" } else  { stringify!($subcategory) };
            single_statistic_option_u64(stringify!($category), subcategory , stringify!($field_name), $timestamp, $proc.$proc_struct.$field_name, $statistics).await;
        )*
    };
}

#[macro_export]
macro_rules! add_list_of_f64_data_to_statistics {
    ($category:expr, $subcategory:expr, $timestamp:expr, $proc:ident, $proc_struct:ident, $statistics:ident, $($field_name:ident),*) => {
        $(
            let subcategory = if stringify!($subcategory) == "\"\"" { "" } else  { stringify!($subcategory) };
            single_statistic_f64(stringify!($category), subcategory, stringify!($field_name), $timestamp, $proc.$proc_struct.$field_name, $statistics).await;
        )*
    };
}
