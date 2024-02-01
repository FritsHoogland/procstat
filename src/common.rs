use crate::stat::CpuStat;
use std::collections::HashMap;
use chrono::{DateTime, Local};
use bounded_vec_deque::BoundedVecDeque;
use std::sync::RwLock;
use crate::stat::{process_stat_data, add_cpu_total_to_history, read_stat_proc_data};
use crate::schedstat::{process_schedstat_data, read_schedstat_proc_data};
use crate::meminfo::{process_meminfo_data, read_meminfo_proc_data, add_memory_to_history, MemInfo};
use crate::blockdevice::{add_blockdevices_to_history, read_blockdevice_sys_data, BlockDeviceInfo, process_blockdevice_data};
use crate::loadavg::{process_loadavg_data, read_loadavg_proc_data, add_loadavg_to_history, LoadavgInfo};
use crate::pressure::{add_pressure_to_history, PressureInfo, process_pressure_data, read_pressure_proc_data};
use crate::net_dev::{add_networkdevices_to_history, NetworkDeviceInfo, process_net_dev_data, read_netdev_proc_data};
use crate::vmstat::{add_vmstat_to_history, VmStatInfo, process_vmstat_data, read_vmstat_proc_data};
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

pub async fn read_proc_data_and_process(statistics: &mut HashMap<(String, String, String), Statistic>, webserver: bool, archiver: bool) {
    let timestamp = Local::now();
    let proc_stat = read_stat_proc_data().await;
    let proc_schedstat = read_schedstat_proc_data().await;
    let proc_meminfo = read_meminfo_proc_data().await;
    let sys_block_devices = read_blockdevice_sys_data().await;
    let proc_netdev = read_netdev_proc_data().await;
    let proc_loadavg = read_loadavg_proc_data().await;
    let proc_pressure = read_pressure_proc_data().await;
    let proc_vmstat = read_vmstat_proc_data().await;
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
    if webserver || archiver {
        add_to_history(statistics).await;
    }
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
    add_cpu_total_to_history(statistics).await;
    add_memory_to_history(statistics).await;
    add_blockdevices_to_history(statistics).await;
    add_networkdevices_to_history(statistics).await;
    add_loadavg_to_history(statistics).await;
    add_pressure_to_history(statistics).await;
    add_vmstat_to_history(statistics).await;
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
