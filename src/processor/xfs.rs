#![allow(unused_assignments)]
use crate::add_list_of_option_u64_data_to_statistics;
use crate::processor::{single_statistic_option_u64, ProcData, Statistic};
use crate::Data;
use crate::ARGS;
use crate::DATA;
use anyhow::Result;
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::fs_xfs_stat::ProcFsXfsStat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ProcessorError;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct XfsInfo {
    pub timestamp: DateTime<Local>,
    pub xs_write_calls: f64,
    pub xs_read_calls: f64,
    pub xs_write_bytes: f64,
    pub xs_read_bytes: f64,
}

pub async fn read_xfs_proc_data() -> ProcFsXfsStat {
    let proc_xfs_stats = proc_sys_parser::fs_xfs_stat::read();
    debug!("{:?}", proc_xfs_stats);
    proc_xfs_stats
}

pub async fn process_xfs_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    add_list_of_option_u64_data_to_statistics!(
        xfs,
        "",
        proc_data.timestamp,
        proc_data,
        xfs,
        statistics,
        xs_write_calls,
        xs_read_calls,
        xs_write_bytes,
        xs_read_bytes
    );

    Ok(())
}

pub async fn add_xfs_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if !statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_write_calls".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_write_calls".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_write_calls".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_write_calls".to_string(),
        })?
        .last_timestamp;
    let xs_write_calls = statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_write_calls".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_write_calls".to_string(),
        })?
        .per_second_value;
    let xs_read_calls = statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_read_calls".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_read_calls".to_string(),
        })?
        .per_second_value;
    let xs_write_bytes = statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_write_bytes".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_write_bytes".to_string(),
        })?
        .per_second_value;
    let xs_read_bytes = statistics
        .get(&(
            "xfs".to_string(),
            "".to_string(),
            "xs_read_bytes".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "xfs".to_string(),
            key2: "".to_string(),
            key3: "xs_read_bytes".to_string(),
        })?
        .per_second_value;

    Data::push_xfs(XfsInfo {
        timestamp,
        xs_write_calls,
        xs_read_calls,
        xs_write_bytes,
        xs_read_bytes,
    })
    .await;

    /*
        DATA.xfs.write().unwrap().push_back(XfsInfo {
            timestamp,
            xs_write_calls,
            xs_read_calls,
            xs_write_bytes,
            xs_read_bytes,
        });
    */

    Ok(())
}

impl Data {
    pub async fn push_xfs(xfs: XfsInfo) {
        while DATA.xfs.read().unwrap().len() >= ARGS.history {
            DATA.xfs.write().unwrap().pop_front();
        }
        DATA.xfs.write().unwrap().push_back(xfs);
    }
}
