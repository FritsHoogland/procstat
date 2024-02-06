#![allow(unused_assignments)]
use std::collections::HashMap;
use log::debug;
use chrono::{DateTime, Local};
use proc_sys_parser::loadavg::ProcLoadavg;
use crate::processor::{ProcData, Statistic, single_statistic_u64, single_statistic_f64};
use crate::{add_list_of_f64_data_to_statistics, add_list_of_u64_data_to_statistics};
use serde::{Serialize, Deserialize};
use crate::HISTORY;

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

pub async fn read_loadavg_proc_data() -> ProcLoadavg {
    let proc_loadavg = proc_sys_parser::loadavg::read();
    debug!("{:?}", proc_loadavg);
    proc_loadavg
}

pub async fn process_loadavg_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    add_list_of_f64_data_to_statistics!(loadavg, "", proc_data.timestamp, proc_data, loadavg, statistics, load_1, load_5, load_15);
    add_list_of_u64_data_to_statistics!(loadavg, "", proc_data.timestamp, proc_data, loadavg, statistics, current_runnable, total, last_pid);
}

pub async fn add_loadavg_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().last_timestamp;
    let load_1 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().last_value;
    let load_5 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_5".to_string())).unwrap().last_value;
    let load_15 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_15".to_string())).unwrap().last_value;
    let current_runnable = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string())).unwrap().last_value;
    let total = statistics.get(&("loadavg".to_string(), "".to_string(), "total".to_string())).unwrap().last_value;
    let last_pid = statistics.get(&("loadavg".to_string(), "".to_string(), "last_pid".to_string())).unwrap().last_value;
    HISTORY.loadavg.write().unwrap().push_back( LoadavgInfo {
        timestamp,
        load_1,
        load_5,
        load_15,
        current_runnable,
        total,
        last_pid,
    });
}

pub async fn print_loadavg(statistics: &HashMap<(String, String, String), Statistic>, output: &str, print_header: bool) {
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
    if !statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string())).unwrap().last_timestamp;
    let current_runnable = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string())).unwrap().last_value;
    let total = statistics.get(&("loadavg".to_string(), "".to_string(), "total".to_string())).unwrap().last_value;
    let load_1 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().last_value;
    let load_5 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_5".to_string())).unwrap().last_value;
    let load_15 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_15".to_string())).unwrap().last_value;
    let blocked = statistics.get(&("stat".to_string(), "".to_string(), "processes_blocked".to_string())).unwrap().last_value;
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
}
