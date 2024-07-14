pub mod blockdevice;
pub mod loadavg;
pub mod meminfo;
pub mod net_dev;
pub mod pressure;
pub mod schedstat;
pub mod stat;
pub mod vmstat;
pub mod xfs;

use crate::processor::blockdevice::{
    add_blockdevices_to_history, process_blockdevice_data, read_blockdevice_sys_data,
    BlockDeviceInfo,
};
use crate::processor::loadavg::{
    add_loadavg_to_history, process_loadavg_data, read_loadavg_proc_data, LoadavgInfo,
};
use crate::processor::meminfo::{
    add_memory_to_history, process_meminfo_data, read_meminfo_proc_data, MemInfo,
};
use crate::processor::net_dev::{
    add_networkdevices_to_history, process_net_dev_data, read_netdev_proc_data, NetworkDeviceInfo,
};
use crate::processor::pressure::{
    add_pressure_to_history, process_pressure_data, read_pressure_proc_data, PressureInfo,
};
use crate::processor::schedstat::{process_schedstat_data, read_schedstat_proc_data};
use crate::processor::vmstat::{
    add_vmstat_to_history, process_vmstat_data, read_vmstat_proc_data, VmStatInfo,
};
use crate::processor::xfs::{add_xfs_to_history, process_xfs_data, read_xfs_proc_data, XfsInfo};
use crate::ARGS;
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use stat::CpuStat;
use stat::{add_cpu_total_to_history, process_stat_data, read_stat_proc_data};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
enum ProcessorError {
    #[error("Unable to find key in hashmap: {hashmap}; keys: {key1}, {key2}, {key3}.")]
    UnableToFindKeyInHashMap {
        hashmap: String,
        key1: String,
        key2: String,
        key3: String,
    },
}

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
    pub xfs: proc_sys_parser::fs_xfs_stat::ProcFsXfsStat,
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
pub struct Data {
    pub cpu: RwLock<VecDeque<CpuStat>>,
    pub memory: RwLock<VecDeque<MemInfo>>,
    pub blockdevices: RwLock<VecDeque<BlockDeviceInfo>>,
    pub networkdevices: RwLock<VecDeque<NetworkDeviceInfo>>,
    pub loadavg: RwLock<VecDeque<LoadavgInfo>>,
    pub pressure: RwLock<VecDeque<PressureInfo>>,
    pub vmstat: RwLock<VecDeque<VmStatInfo>>,
    pub xfs: RwLock<VecDeque<XfsInfo>>,
}

impl Data {
    pub fn new(history: usize) -> Data {
        Data {
            cpu: RwLock::new(VecDeque::with_capacity(history)),
            memory: RwLock::new(VecDeque::with_capacity(history)),
            blockdevices: RwLock::new(VecDeque::with_capacity(history)),
            networkdevices: RwLock::new(VecDeque::with_capacity(history)),
            loadavg: RwLock::new(VecDeque::with_capacity(history)),
            pressure: RwLock::new(VecDeque::with_capacity(history)),
            vmstat: RwLock::new(VecDeque::with_capacity(history)),
            xfs: RwLock::new(VecDeque::with_capacity(history)),
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
    pub xfs: Vec<XfsInfo>,
}

pub async fn read_proc_data_and_process(
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    let timestamp = Local::now();
    let proc_stat = read_stat_proc_data()
        .await
        .with_context(|| "Proc stat reader")?;
    let proc_schedstat = read_schedstat_proc_data()
        .await
        .with_context(|| "Proc schedstat reader")?;
    let proc_meminfo = read_meminfo_proc_data()
        .await
        .with_context(|| "Proc meminfo reader")?;
    let sys_block_devices = read_blockdevice_sys_data()
        .await
        .with_context(|| "Sys block reader")?;
    let proc_netdev = read_netdev_proc_data()
        .await
        .with_context(|| "Proc netdev reader")?;
    let proc_loadavg = read_loadavg_proc_data()
        .await
        .with_context(|| "Proc loadavg reader")?;
    let proc_pressure = read_pressure_proc_data()
        .await
        .with_context(|| "Proc pressure reader")?;
    let proc_vmstat = read_vmstat_proc_data()
        .await
        .with_context(|| "proc vmstat reader")?;
    let proc_xfs = read_xfs_proc_data().await;
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
        xfs: proc_xfs,
    };
    process_data(proc_data, statistics)
        .await
        .with_context(|| "Process data")?;
    if ARGS.webserver || ARGS.archiver {
        add_to_history(statistics)
            .await
            .with_context(|| "Add to history")?;
    }
    Ok(())
}

pub async fn process_data(
    proc_data: ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    process_stat_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc stat processor")?;
    process_schedstat_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc schedstat processor")?;
    process_meminfo_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc meminfo processor")?;
    process_blockdevice_data(&proc_data, statistics)
        .await
        .with_context(|| "Sys block processor")?;
    process_net_dev_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc netdev processor")?;
    process_loadavg_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc loadavg processor")?;
    process_pressure_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc pressure processor")?;
    process_vmstat_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc vmstat processor")?;
    process_xfs_data(&proc_data, statistics)
        .await
        .with_context(|| "Proc xfs processor")?;

    Ok(())
}

pub async fn add_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    add_cpu_total_to_history(statistics)
        .await
        .with_context(|| "Proc stat history addition")?;
    add_memory_to_history(statistics)
        .await
        .with_context(|| "Proc meminfo history addition")?;
    add_blockdevices_to_history(statistics)
        .await
        .with_context(|| "Sys blockdevices history addition")?;
    add_networkdevices_to_history(statistics)
        .await
        .with_context(|| "Proc netdev history addition")?;
    add_loadavg_to_history(statistics)
        .await
        .with_context(|| "Proc loadavg history addition")?;
    add_pressure_to_history(statistics)
        .await
        .with_context(|| "Proc pressure history addition")?;
    add_vmstat_to_history(statistics)
        .await
        .with_context(|| "Proc vmstat history addition")?;
    add_xfs_to_history(statistics)
        .await
        .with_context(|| "Proc xfs history addition")?;
    Ok(())
}

pub async fn single_statistic_u64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: u64,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) {
    statistics
        .entry((
            category.to_string(),
            subcategory.to_string(),
            name.to_string(),
        ))
        .and_modify(|row| {
            row.delta_value = value as f64 - row.last_value;
            row.per_second_value = row.delta_value
                / (timestamp
                    .signed_duration_since(row.last_timestamp)
                    .num_milliseconds() as f64
                    / 1000_f64);
            row.last_value = value as f64;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() {
                row.per_second_value = 0_f64
            }
        })
        .or_insert(Statistic {
            last_timestamp: timestamp,
            last_value: value as f64,
            delta_value: 0.0,
            per_second_value: 0.0,
            updated_value: false,
        });
}
pub async fn single_statistic_f64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: f64,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) {
    statistics
        .entry((
            category.to_string(),
            subcategory.to_string(),
            name.to_string(),
        ))
        .and_modify(|row| {
            row.delta_value = value - row.last_value;
            row.per_second_value = row.delta_value
                / (timestamp
                    .signed_duration_since(row.last_timestamp)
                    .num_milliseconds() as f64
                    / 1000_f64);
            row.last_value = value;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() {
                row.per_second_value = 0_f64
            }
        })
        .or_insert(Statistic {
            last_timestamp: timestamp,
            last_value: value,
            delta_value: 0.0,
            per_second_value: 0.0,
            updated_value: false,
        });
}
pub async fn single_statistic_option_f64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: Option<f64>,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) {
    let value = value.unwrap_or_default();
    statistics
        .entry((
            category.to_string(),
            subcategory.to_string(),
            name.to_string(),
        ))
        .and_modify(|row| {
            row.delta_value = value - row.last_value;
            row.per_second_value = row.delta_value
                / (timestamp
                    .signed_duration_since(row.last_timestamp)
                    .num_milliseconds() as f64
                    / 1000_f64);
            row.last_value = value;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() {
                row.per_second_value = 0_f64
            }
        })
        .or_insert(Statistic {
            last_timestamp: timestamp,
            last_value: value,
            delta_value: 0.0,
            per_second_value: 0.0,
            updated_value: false,
        });
}
pub async fn single_statistic_option_u64(
    category: &str,
    subcategory: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: Option<u64>,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) {
    let value = value.unwrap_or_default();
    statistics
        .entry((
            category.to_string(),
            subcategory.to_string(),
            name.to_string(),
        ))
        .and_modify(|row| {
            row.delta_value = value as f64 - row.last_value;
            row.per_second_value = row.delta_value
                / (timestamp
                    .signed_duration_since(row.last_timestamp)
                    .num_milliseconds() as f64
                    / 1000_f64);
            row.last_value = value as f64;
            row.last_timestamp = timestamp;
            row.updated_value = true;
            if row.per_second_value.is_nan() {
                row.per_second_value = 0_f64
            }
        })
        .or_insert(Statistic {
            last_timestamp: timestamp,
            last_value: value as f64,
            delta_value: 0.0,
            per_second_value: 0.0,
            updated_value: false,
        });
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
