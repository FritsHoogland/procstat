#![allow(unused_assignments)]

use crate::processor::{
    single_statistic_f64, single_statistic_option_f64, single_statistic_option_u64,
    single_statistic_u64, ProcData, Statistic,
};
use crate::Data;
use crate::ARGS;
use crate::DATA;
use anyhow::Result;
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::pressure::ProcPressure;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ProcessorError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PressureInfo {
    pub timestamp: DateTime<Local>,
    pub cpu_some_avg10: f64,
    pub cpu_some_avg60: f64,
    pub cpu_some_avg300: f64,
    pub cpu_some_total: f64,
    pub cpu_full_avg10: f64,
    pub cpu_full_avg60: f64,
    pub cpu_full_avg300: f64,
    pub cpu_full_total: f64,
    pub io_some_avg10: f64,
    pub io_some_avg60: f64,
    pub io_some_avg300: f64,
    pub io_some_total: f64,
    pub io_full_avg10: f64,
    pub io_full_avg60: f64,
    pub io_full_avg300: f64,
    pub io_full_total: f64,
    pub memory_some_avg10: f64,
    pub memory_some_avg60: f64,
    pub memory_some_avg300: f64,
    pub memory_some_total: f64,
    pub memory_full_avg10: f64,
    pub memory_full_avg60: f64,
    pub memory_full_avg300: f64,
    pub memory_full_total: f64,
}

pub async fn read_pressure_proc_data() -> Result<ProcPressure> {
    let proc_pressure = proc_sys_parser::pressure::read()?;
    debug!("{:?}", proc_pressure);
    Ok(proc_pressure)
}

pub async fn process_pressure_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if proc_data.pressure.psi.as_ref().is_none() {
        return Ok(());
    };
    single_statistic_f64(
        "pressure",
        "",
        "cpu_some_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg10,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "cpu_some_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg60,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "cpu_some_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg300,
        statistics,
    )
    .await;
    single_statistic_u64(
        "pressure",
        "",
        "cpu_some_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_some_total,
        statistics,
    )
    .await;
    single_statistic_option_f64(
        "pressure",
        "",
        "cpu_full_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg10,
        statistics,
    )
    .await;
    single_statistic_option_f64(
        "pressure",
        "",
        "cpu_full_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg60,
        statistics,
    )
    .await;
    single_statistic_option_f64(
        "pressure",
        "",
        "cpu_full_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg300,
        statistics,
    )
    .await;
    single_statistic_option_u64(
        "pressure",
        "",
        "cpu_full_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().cpu_full_total,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_some_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_some_avg10,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_some_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_some_avg60,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_some_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_some_avg300,
        statistics,
    )
    .await;
    single_statistic_u64(
        "pressure",
        "",
        "io_some_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_some_total,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_full_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_full_avg10,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_full_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_full_avg60,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "io_full_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_full_avg300,
        statistics,
    )
    .await;
    single_statistic_u64(
        "pressure",
        "",
        "io_full_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().io_full_total,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_some_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_some_avg10,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_some_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_some_avg60,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_some_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_some_avg300,
        statistics,
    )
    .await;
    single_statistic_u64(
        "pressure",
        "",
        "memory_some_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_some_total,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_full_avg10",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_full_avg10,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_full_avg60",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_full_avg60,
        statistics,
    )
    .await;
    single_statistic_f64(
        "pressure",
        "",
        "memory_full_avg300",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_full_avg300,
        statistics,
    )
    .await;
    single_statistic_u64(
        "pressure",
        "",
        "memory_full_total",
        proc_data.timestamp,
        proc_data.pressure.psi.as_ref().unwrap().memory_full_total,
        statistics,
    )
    .await;

    Ok(())
}

pub async fn add_pressure_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if !statistics.contains_key(&(
        "pressure".to_string(),
        "".to_string(),
        "cpu_some_avg10".to_string(),
    )) {
        return Ok(());
    };
    if !statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg10".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg10".to_string(),
        })?
        .last_timestamp;
    let cpu_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg10".to_string(),
        })?
        .last_value;
    let cpu_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg60".to_string(),
        })?
        .last_value;
    let cpu_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg300".to_string(),
        })?
        .last_value;
    let cpu_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_total".to_string(),
        })?
        .per_second_value;
    let cpu_full_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_full_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_full_avg10".to_string(),
        })?
        .last_value;
    let cpu_full_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_full_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_full_avg60".to_string(),
        })?
        .last_value;
    let cpu_full_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_full_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_full_avg300".to_string(),
        })?
        .last_value;
    let cpu_full_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_full_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_full_total".to_string(),
        })?
        .per_second_value;
    let io_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg10".to_string(),
        })?
        .last_value;
    let io_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg60".to_string(),
        })?
        .last_value;
    let io_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg300".to_string(),
        })?
        .last_value;
    let io_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_total".to_string(),
        })?
        .per_second_value;
    let io_full_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg10".to_string(),
        })?
        .last_value;
    let io_full_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg60".to_string(),
        })?
        .last_value;
    let io_full_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg300".to_string(),
        })?
        .last_value;
    let io_full_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_total".to_string(),
        })?
        .per_second_value;
    let memory_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg10".to_string(),
        })?
        .last_value;
    let memory_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg60".to_string(),
        })?
        .last_value;
    let memory_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg300".to_string(),
        })?
        .last_value;
    let memory_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_total".to_string(),
        })?
        .per_second_value;
    let memory_full_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg10".to_string(),
        })?
        .last_value;
    let memory_full_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg60".to_string(),
        })?
        .last_value;
    let memory_full_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg300".to_string(),
        })?
        .last_value;
    let memory_full_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_total".to_string(),
        })?
        .per_second_value;
    Data::push_pressure(PressureInfo {
        timestamp,
        cpu_some_avg10,
        cpu_some_avg60,
        cpu_some_avg300,
        cpu_some_total,
        cpu_full_avg10,
        cpu_full_avg60,
        cpu_full_avg300,
        cpu_full_total,
        io_some_avg10,
        io_some_avg60,
        io_some_avg300,
        io_some_total,
        io_full_avg10,
        io_full_avg60,
        io_full_avg300,
        io_full_total,
        memory_some_avg10,
        memory_some_avg60,
        memory_some_avg300,
        memory_some_total,
        memory_full_avg10,
        memory_full_avg60,
        memory_full_avg300,
        memory_full_total,
    })
    .await;
    /*
        DATA.pressure.write().unwrap().push_back(PressureInfo {
            timestamp,
            cpu_some_avg10,
            cpu_some_avg60,
            cpu_some_avg300,
            cpu_some_total,
            cpu_full_avg10,
            cpu_full_avg60,
            cpu_full_avg300,
            cpu_full_total,
            io_some_avg10,
            io_some_avg60,
            io_some_avg300,
            io_some_total,
            io_full_avg10,
            io_full_avg60,
            io_full_avg300,
            io_full_total,
            memory_some_avg10,
            memory_some_avg60,
            memory_some_avg300,
            memory_some_total,
            memory_full_avg10,
            memory_full_avg60,
            memory_full_avg300,
            memory_full_total,
        });
    */

    Ok(())
}

impl Data {
    pub async fn push_pressure(pressure: PressureInfo) {
        while DATA.pressure.read().unwrap().len() >= ARGS.history {
            DATA.pressure.write().unwrap().pop_front();
        }
        DATA.pressure.write().unwrap().push_back(pressure);
    }
}

pub async fn print_psi(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool,
) -> Result<()> {
    if print_header {
        match output {
            "sar-q-CPU" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp", "", "%scpu-10", "%scpu-60", "%scpu-300", "%scpu",
                );
            }
            "sar-q-IO" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp",
                    "",
                    "%sio-10",
                    "%sio-60",
                    "%sio-300",
                    "%sio",
                    "%fio-10",
                    "%fio-60",
                    "%fio-300",
                    "%fio",
                );
            }
            "sar-q-MEM" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp",
                    "",
                    "%smem-10",
                    "%smem-60",
                    "%smem-300",
                    "%smem",
                    "%fmem-10",
                    "%fmem-60",
                    "%fmem-300",
                    "%fmem",
                );
            }
            &_ => todo! {},
        }
    }
    if !statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .unwrap_or(&Statistic::default())
        .updated_value
    {
        return Ok(());
    };

    let timestamp = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg10".to_string(),
        })?
        .last_timestamp;
    let cpu_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg10".to_string(),
        })?
        .last_value;
    let cpu_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg60".to_string(),
        })?
        .last_value;
    let cpu_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_avg300".to_string(),
        })?
        .last_value;
    let cpu_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "cpu_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "cpu_some_total".to_string(),
        })?
        .per_second_value;
    // these are currently not used, but are added to the kernel source
    //let cpu_full_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg10".to_string())).unwrap().last_value;
    //let cpu_full_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg60".to_string())).unwrap().last_value;
    //let cpu_full_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg300".to_string())).unwrap().last_value;
    //let cpu_full_total = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_total".to_string())).unwrap().per_second_value;
    let mem_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg10".to_string(),
        })?
        .last_value;
    let mem_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg60".to_string(),
        })?
        .last_value;
    let mem_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_avg300".to_string(),
        })?
        .last_value;
    let mem_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_some_total".to_string(),
        })?
        .per_second_value;
    let mem_full_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_full_avg10".to_string(),
        })?
        .last_value;
    let mem_full_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_full_avg60".to_string(),
        })?
        .last_value;
    let mem_full_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_full_avg300".to_string(),
        })?
        .last_value;
    let mem_full_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "memory_full_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "memory_full_total".to_string(),
        })?
        .per_second_value;
    let io_some_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg10".to_string(),
        })?
        .last_value;
    let io_some_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg60".to_string(),
        })?
        .last_value;
    let io_some_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_avg300".to_string(),
        })?
        .last_value;
    let io_some_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_some_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_some_total".to_string(),
        })?
        .per_second_value;
    let io_full_avg10 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg10".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg10".to_string(),
        })?
        .last_value;
    let io_full_avg60 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg60".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg60".to_string(),
        })?
        .last_value;
    let io_full_avg300 = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_avg300".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_avg300".to_string(),
        })?
        .last_value;
    let io_full_total = statistics
        .get(&(
            "pressure".to_string(),
            "".to_string(),
            "io_full_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "pressure".to_string(),
            key2: "".to_string(),
            key3: "io_full_total".to_string(),
        })?
        .per_second_value;
    match output {
        "sar-q-CPU" => {
            println!(
                "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "",
                cpu_some_avg10,
                cpu_some_avg60,
                cpu_some_avg300,
                cpu_some_total / 10_000_f64,
            );
        }
        "sar-q-IO" => {
            println!(
                "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "",
                io_some_avg10,
                io_some_avg60,
                io_some_avg300,
                io_some_total / 10_000_f64,
                io_full_avg10,
                io_full_avg60,
                io_full_avg300,
                io_full_total / 10_000_f64,
            );
        }
        "sar-q-MEM" => {
            println!(
                "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "",
                mem_some_avg10,
                mem_some_avg60,
                mem_some_avg300,
                mem_some_total / 10_000_f64,
                mem_full_avg10,
                mem_full_avg60,
                mem_full_avg300,
                mem_full_total / 10_000_f64,
            );
        }
        &_ => todo! {},
    }

    Ok(())
}
