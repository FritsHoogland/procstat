#![allow(unused_assignments)]
use std::collections::HashMap;
use log::debug;
use chrono::{DateTime, Local};
use proc_sys_parser::loadavg::ProcLoadavg;
use crate::processor::{ProcData, Statistic, single_statistic_u64, single_statistic_f64};
use crate::{add_list_of_f64_data_to_statistics, add_list_of_u64_data_to_statistics};
use serde::{Serialize, Deserialize};
use crate::HISTORY;
use anyhow::Result;

use super::ProcessorError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LoadavgInfo {
    pub timestamp: DateTime<Local>,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
    pub current_runnable: f64,
    pub total: f64,
    pub last_pid: f64,
}

pub async fn read_loadavg_proc_data() -> Result<ProcLoadavg> {
    let proc_loadavg = proc_sys_parser::loadavg::read()?;
    debug!("{:?}", proc_loadavg);
    Ok(proc_loadavg)
}

pub async fn process_loadavg_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>) -> Result<()> {
    add_list_of_f64_data_to_statistics!(loadavg, "", proc_data.timestamp, proc_data, loadavg, statistics, load_1, load_5, load_15);
    add_list_of_u64_data_to_statistics!(loadavg, "", proc_data.timestamp, proc_data, loadavg, statistics, current_runnable, total, last_pid);

    Ok(())
}

pub async fn add_loadavg_to_history(statistics: &HashMap<(String, String, String), Statistic>) -> Result<()> {
    if !statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_1".to_string() })?.updated_value { return Ok(()) };
    let timestamp = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_1".to_string() })?.last_timestamp;
    let load_1 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_1".to_string() })?.last_value;
    let load_5 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_5".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_5".to_string() })?.last_value;
    let load_15 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_15".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_15".to_string() })?.last_value;
    let current_runnable = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "current_runnable".to_string() })?.last_value;
    let total = statistics.get(&("loadavg".to_string(), "".to_string(), "total".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "total".to_string() })?.last_value;
    let last_pid = statistics.get(&("loadavg".to_string(), "".to_string(), "last_pid".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "last_pid".to_string() })?.last_value;
    HISTORY.loadavg.write().unwrap().push_back( LoadavgInfo {
        timestamp,
        load_1,
        load_5,
        load_15,
        current_runnable,
        total,
        last_pid,
    });

    Ok(())
}

pub async fn print_loadavg(
    statistics: &HashMap<(String, String, String), Statistic>, 
    output: &str, print_header: bool
) -> Result<()> {

    if print_header {
        match output {
            "sar-q-LOAD" => {
                println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "",
                         "runq-sz",
                         "plist-sz",
                         "ldavg-1",
                         "ldavg-5",
                         "ldavg-15",
                         "blocked",
                );
            }
            &_ => todo!(),
        }
    }

    if !statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "current_runnable".to_string() })?.updated_value { return Ok(()) };
    let timestamp = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "current_runnable".to_string() })?.last_timestamp;
    let current_runnable = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "current_runnable".to_string() })?.last_value;
    let total = statistics.get(&("loadavg".to_string(), "".to_string(), "total".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "total".to_string() })?.last_value;
    let load_1 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_1".to_string() })?.last_value;
    let load_5 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_5".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_5".to_string() })?.last_value;
    let load_15 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_15".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "load_15".to_string() })?.last_value;
    let blocked = statistics.get(&("stat".to_string(), "".to_string(), "processes_blocked".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "loadavg".to_string(), key2: "".to_string(), key3: "processes_blocked".to_string() })?.last_value;

    match output {
        "sar-q-LOAD" => {
            println!("{:10} {:7}    {:10.0} {:10.0} {:10.2} {:10.2} {:10.2} {:10.0}",
                     timestamp.format("%H:%M:%S"),
                     "",
                     current_runnable,
                     total,
                     load_1,
                     load_5,
                     load_15,
                     blocked,
            );
        }
        &_ => todo!(),
    }

    Ok(())
}
