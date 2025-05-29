use crate::processor::{
    single_statistic_option_u64, single_statistic_u64, ProcData, ProcessorError, Statistic,
};
use crate::Data;
use crate::ARGS;
use crate::DATA;
use anyhow::Result;
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::block::{Builder, SysBlock};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BlockDeviceInfo {
    pub timestamp: DateTime<Local>,
    pub device_name: String,
    pub device_major_minor: String,
    pub removable: f64,
    pub ro: f64,
    pub reads_completed_success: f64,
    pub reads_merged: f64,
    pub reads_bytes: f64,
    pub reads_time_spent_ms: f64,
    pub writes_completed_success: f64,
    pub writes_merged: f64,
    pub writes_bytes: f64,
    pub writes_time_spent_ms: f64,
    pub ios_in_progress: f64,
    pub ios_time_spent_ms: f64,
    pub ios_weighted_time_spent_ms: f64,
    pub discards_completed_success: f64,
    pub discards_merged: f64,
    pub discards_bytes: f64,
    pub discards_time_spent_ms: f64,
    pub flush_requests_completed_success: f64,
    pub flush_requests_time_spent_ms: f64,
    pub inflight_reads: f64,
    pub inflight_writes: f64,
    pub queue_max_sectors_kb: f64,
    pub queue_max_hw_sectors_kb: f64,
    pub queue_nr_requests: f64,
    pub queue_rotational: f64,
    pub queue_dax: f64,
    pub queue_hw_sector_size: f64,
    pub queue_logical_block_size: f64,
    pub queue_nomerges: f64,
    pub queue_physical_block_size: f64,
    pub queue_read_ahead_kb: f64,
    pub queue_discard_max_hw_bytes: f64,
    pub queue_discard_max_bytes: f64,
}

pub async fn read_blockdevice_sys_data() -> Result<SysBlock> {
    //let sys_block = proc_sys_parser::block::read()?;
    let sys_block = Builder::new().regex(&ARGS.disk_filter).read()?;
    debug!("{:?}", sys_block);
    Ok(sys_block)
}

pub async fn process_blockdevice_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    for disk in &proc_data.blockdevices.block_devices {
        macro_rules! add_diskstats_data_to_statistics_u64 {
            ($($field_name:ident),*) => {
                $(
                    single_statistic_u64("blockdevice", &disk.device_name, stringify!($field_name), proc_data.timestamp, disk.$field_name, statistics).await;
                )*
            };
        }
        add_diskstats_data_to_statistics_u64!(
            stat_reads_completed_success,
            stat_reads_merged,
            stat_reads_sectors,
            stat_reads_time_spent_ms,
            stat_writes_completed_success,
            stat_writes_merged,
            stat_writes_sectors,
            stat_writes_time_spent_ms,
            stat_ios_in_progress,
            stat_ios_time_spent_ms,
            stat_ios_weighted_time_spent_ms,
            inflight_reads,
            inflight_writes,
            queue_max_hw_sectors_kb,
            queue_max_sectors_kb,
            queue_nr_requests,
            dev_block_major,
            dev_block_minor,
            removable,
            ro,
            queue_rotational,
            queue_dax,
            queue_hw_sector_size,
            queue_logical_block_size,
            queue_nomerges,
            queue_physical_block_size,
            queue_read_ahead_kb,
            queue_discard_max_bytes,
            queue_discard_max_hw_bytes
        );
        macro_rules! add_diskstats_data_to_statistics_option_u64 {
            ($($field_name:ident),*) => {
                $(
                    single_statistic_option_u64("blockdevice", &disk.device_name, stringify!($field_name), proc_data.timestamp, disk.$field_name, statistics).await;
                )*
            };
        }
        add_diskstats_data_to_statistics_option_u64!(
            stat_discards_completed_success,
            stat_discards_merged,
            stat_discards_sectors,
            stat_discards_time_spent_ms,
            stat_flush_requests_completed_success,
            stat_flush_requests_time_spent_ms
        );
    }

    Ok(())
}

pub async fn add_blockdevices_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    let disk_list: Vec<_> = statistics
        .keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    // allow systems without disks; issue https://github.com/FritsHoogland/procstat/issues/3
    if !statistics
        .get(&(
            "blockdevice".to_string(),
            disk_list[0].to_string(),
            "stat_reads_completed_success".to_string(),
        ))
        .unwrap_or(&Statistic::default())
        .updated_value
    {
        return Ok(());
    };

    // this is an array where the statistics for the 'TOTAL' device are stored by simply adding the statistics to the elements in the array.
    let mut totals = [0_f64; 20];

    let timestamp = statistics
        .get(&(
            "blockdevice".to_string(),
            disk_list[0].to_string(),
            "stat_reads_completed_success".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "blockdevice".to_string(),
            key2: disk_list[0].to_string(),
            key3: "stat_reads_completed_success".to_string(),
        })?
        .last_timestamp;

    // loop are loopback devices, sr commonly is a readonly device supposedly a cdrom
    for disk_name in disk_list
        .iter()
        .filter(|disk_name| !disk_name.starts_with("loop") & !disk_name.starts_with("sr"))
    {
        // reads
        let reads_completed_success = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_reads_completed_success".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_reads_completed_success".to_string(),
            })?
            .per_second_value;
        totals[0] += reads_completed_success;
        let reads_merged = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_reads_merged".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_reads_merged".to_string(),
            })?
            .per_second_value;
        totals[1] += reads_merged;
        let reads_bytes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_reads_sectors".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_reads_sectors".to_string(),
            })?
            .per_second_value
            * 512_f64; // convert 512 bytes sector reads to bytes
        totals[2] += reads_bytes;
        let reads_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_reads_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_reads_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[3] += reads_time_spent_ms;
        // writes
        let writes_completed_success = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_writes_completed_success".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_writes_completed_success".to_string(),
            })?
            .per_second_value;
        totals[4] += writes_completed_success;
        let writes_merged = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_writes_merged".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_writes_merged".to_string(),
            })?
            .per_second_value;
        totals[5] += writes_merged;
        let writes_bytes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_writes_sectors".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_writes_wectors".to_string(),
            })?
            .per_second_value
            * 512_f64; // convert 512 bytes sector writes to bytes
        totals[6] += writes_bytes;
        let writes_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_writes_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_writes_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[7] += writes_time_spent_ms;
        // ios
        let ios_in_progress = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_ios_in_progress".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_ios_in_progress".to_string(),
            })?
            .per_second_value;
        totals[8] += ios_in_progress;
        let ios_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_ios_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_ios_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[9] += ios_time_spent_ms;
        let ios_weighted_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_ios_weighted_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_ios_weighted_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[10] += ios_weighted_time_spent_ms;
        // discards
        let discards_completed_success = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_discards_completed_success".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_discards_completed_success".to_string(),
            })?
            .per_second_value;
        totals[11] += discards_completed_success;
        let discards_merged = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_discards_merged".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_discards_merged".to_string(),
            })?
            .per_second_value;
        totals[12] += discards_merged;
        let discards_bytes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_discards_sectors".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_discards_sectors".to_string(),
            })?
            .per_second_value
            * 512_f64; // convert 512 sectors discards to bytes
        totals[13] += discards_bytes;
        let discards_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_discards_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_discards_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[14] += discards_time_spent_ms;
        // flushes
        let flush_requests_completed_success = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_flush_requests_completed_success".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_flush_requests_completed_success".to_string(),
            })?
            .per_second_value;
        totals[15] += flush_requests_completed_success;
        let flush_requests_time_spent_ms = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_flush_requests_time_spent_ms".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_flush_requests_time_spent_ms".to_string(),
            })?
            .per_second_value;
        totals[16] += flush_requests_time_spent_ms;
        // extras
        let inflight_reads = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "inflight_reads".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "inflight_reads".to_string(),
            })?
            .last_value;
        totals[17] += inflight_reads;
        let inflight_writes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "inflight_writes".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "inflight_writes".to_string(),
            })?
            .last_value;
        totals[18] += inflight_writes;
        let queue_nr_requests = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_nr_requests".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_nr_requests".to_string(),
            })?
            .last_value;
        totals[19] += queue_nr_requests;
        let queue_max_sectors_kb = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_max_sectors_kb".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_max_sectors_kb".to_string(),
            })?
            .last_value;
        let queue_max_hw_sectors_kb = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_max_hw_sectors_kb".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_max_hw_sectors_kb".to_string(),
            })?
            .last_value;
        let dev_block_major = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "dev_block_major".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "dev_block_major".to_string(),
            })?
            .last_value;
        let dev_block_minor = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "dev_block_minor".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "dev_block_minor".to_string(),
            })?
            .last_value;
        let removable = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "removable".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "removable".to_string(),
            })?
            .last_value;
        let ro = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "ro".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "ro".to_string(),
            })?
            .last_value;
        let queue_rotational = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_rotational".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_rotational".to_string(),
            })?
            .last_value;
        let queue_dax = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_dax".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_dax".to_string(),
            })?
            .last_value;
        let queue_hw_sector_size = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_hw_sector_size".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_hw_sector_size".to_string(),
            })?
            .last_value;
        let queue_logical_block_size = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_logical_block_size".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_logical_block_size".to_string(),
            })?
            .last_value;
        let queue_nomerges = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_nomerges".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_nomerges".to_string(),
            })?
            .last_value;
        let queue_physical_block_size = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_physical_block_size".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_physical_block_size".to_string(),
            })?
            .last_value;
        let queue_read_ahead_kb = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_read_ahead_kb".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_read_ahead_kb".to_string(),
            })?
            .last_value;
        let queue_discard_max_hw_bytes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_discard_max_hw_bytes".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_discard_max_hw_bytes".to_string(),
            })?
            .last_value;
        let queue_discard_max_bytes = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "queue_discard_max_bytes".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "queue_discard_max_bytes".to_string(),
            })?
            .last_value;

        DATA.blockdevices
            .write()
            .unwrap()
            .push_back(BlockDeviceInfo {
                timestamp,
                device_name: disk_name.to_string(),
                device_major_minor: format!("{}:{}", dev_block_major, dev_block_minor),
                removable,
                ro,
                reads_completed_success,
                reads_merged,
                reads_bytes,
                reads_time_spent_ms,
                writes_completed_success,
                writes_merged,
                writes_bytes,
                writes_time_spent_ms,
                ios_in_progress,
                ios_time_spent_ms,
                ios_weighted_time_spent_ms,
                discards_completed_success,
                discards_merged,
                discards_bytes,
                discards_time_spent_ms,
                flush_requests_completed_success,
                flush_requests_time_spent_ms,
                inflight_reads,
                inflight_writes,
                queue_max_sectors_kb,
                queue_max_hw_sectors_kb,
                queue_nr_requests,
                queue_rotational,
                queue_dax,
                queue_hw_sector_size,
                queue_logical_block_size,
                queue_nomerges,
                queue_physical_block_size,
                queue_read_ahead_kb,
                queue_discard_max_hw_bytes,
                queue_discard_max_bytes,
            });
    }
    Data::push_blockdevices(BlockDeviceInfo {
        timestamp,
        device_name: "TOTAL".to_string(),
        reads_completed_success: totals[0],
        reads_merged: totals[1],
        reads_bytes: totals[2],
        reads_time_spent_ms: totals[3],
        writes_completed_success: totals[4],
        writes_merged: totals[5],
        writes_bytes: totals[6],
        writes_time_spent_ms: totals[7],
        ios_in_progress: totals[8],
        ios_time_spent_ms: totals[9],
        ios_weighted_time_spent_ms: totals[10],
        discards_completed_success: totals[11],
        discards_merged: totals[12],
        discards_bytes: totals[13],
        discards_time_spent_ms: totals[14],
        flush_requests_completed_success: totals[15],
        flush_requests_time_spent_ms: totals[16],
        inflight_reads: totals[17],
        inflight_writes: totals[18],
        queue_nr_requests: totals[19],
        ..Default::default()
    })
    .await;
    /*
        DATA.blockdevices
            .write()
            .unwrap()
            .push_back(BlockDeviceInfo {
                timestamp,
                device_name: "TOTAL".to_string(),
                reads_completed_success: totals[0],
                reads_merged: totals[1],
                reads_bytes: totals[2],
                reads_time_spent_ms: totals[3],
                writes_completed_success: totals[4],
                writes_merged: totals[5],
                writes_bytes: totals[6],
                writes_time_spent_ms: totals[7],
                ios_in_progress: totals[8],
                ios_time_spent_ms: totals[9],
                ios_weighted_time_spent_ms: totals[10],
                discards_completed_success: totals[11],
                discards_merged: totals[12],
                discards_bytes: totals[13],
                discards_time_spent_ms: totals[14],
                flush_requests_completed_success: totals[15],
                flush_requests_time_spent_ms: totals[16],
                inflight_reads: totals[17],
                inflight_writes: totals[18],
                queue_nr_requests: totals[19],
                ..Default::default()
            });
    */

    Ok(())
}

impl Data {
    pub async fn push_blockdevices(blockdevice: BlockDeviceInfo) {
        while DATA.blockdevices.read().unwrap().len() >= ARGS.history {
            DATA.blockdevices.write().unwrap().pop_front();
        }
        DATA.blockdevices.write().unwrap().push_back(blockdevice);
    }
}

pub async fn print_diskstats(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool,
) -> Result<()> {
    let disk_list: Vec<_> = statistics
        .keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    // https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/pr_stats.c#L723
    // single row output must print the header before the check if the value is updated
    if output == "sar-b" && print_header {
        println!(
            "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
            "timestamp", "", "tps", "rtps", "wtps", "dtps", "bread/s", "bwrtn/s", "bdscd/s",
        );
    }

    if !statistics
        .get(&(
            "blockdevice".to_string(),
            disk_list[0].to_string(),
            "stat_reads_completed_success".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "blockdevice".to_string(),
            key2: disk_list[0].to_string(),
            key3: "stat_reads_completed_success".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };

    match output {
        "sar-d" => {
            println!(
                "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                "timestamp", "DEV", "tps", "rMB/s", "wMB/s", "areq-sz", "aqu-sz", "await",
            );
        }
        "iostat" => {
            println!(
                "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10}",
                "timestamp", "Device", "tps", "MB_read/s", "MB_wrtn/s", "MB_read", "MB_wrtn",
            );
        }
        "iostat-x" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                     "timestamp",
                     "Device",
                     "r/s",
                     "w/s",
                     "rMB/s",
                     "wMB/s",
                     "rrqm/s",
                     "wrqm/s",
                     "%rrqm/s",
                     "%wrqm/s",
                     "r_await",
                     "w_await",
                     "aqu-sz",
                     "rareq-sz",
                     "wareq-sz",
            );
        }
        "sar-b" => {}
        "ioq" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                 "timestamp",
                 "Device",
                 "r/s",
                 "rMB/s",
                 "rmrg/s",
                 "r_await",
                 "w/s",
                 "wMB/s",
                 "wmrg/s",
                 "w_await",
                 "d/s",
                 "dMB/s",
                 "dmrg/s",
                 "d_await",
                 "q_size",
                 "q_limit",
                 "r_inflight",
                 "w_inflight",
             );
        }
        "ios" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                 "timestamp",
                 "Device",
                 "r/s",
                 "rMB/s",
                 "rmrg/s",
                 "r_await",
                 "rareq_MB",
                 "w/s",
                 "wMB/s",
                 "wmrg/s",
                 "w_await",
                 "wareq-MB",
                 "d/s",
                 "dMB/s",
                 "dmrg/s",
                 "d_await",
                 "max_MB",
                 "hw_max_MB",
             );
        }
        &_ => todo!(),
    }
    // https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/tests/12.0.1/rd_stats.c#L711
    if output == "sar-b" {
        let timestamp = statistics
            .get(&(
                "blockdevice".to_string(),
                disk_list[0].to_string(),
                "stat_reads_completed_success".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_list[0].to_string(),
                key3: "stat_reads_completed_success".to_string(),
            })?
            .last_timestamp;
        let mut total_reads_completed_success = 0_f64;
        let mut total_reads_sectors = 0_f64;
        let mut total_writes_completed_success = 0_f64;
        let mut total_writes_sectors = 0_f64;
        let mut total_discards_completed_success = 0_f64;
        let mut total_discards_sectors = 0_f64;
        for disk_name in &disk_list {
            total_reads_completed_success += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_reads_completed_success".to_string(),
                })?
                .per_second_value;
            total_reads_sectors += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_reads_sectors".to_string(),
                })?
                .per_second_value;
            total_writes_completed_success += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_writes_completed_success".to_string(),
                })?
                .per_second_value;
            total_writes_sectors += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_writes_sectors".to_string(),
                })?
                .per_second_value;
            total_discards_completed_success += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_discards_completed_success".to_string(),
                })?
                .per_second_value;
            total_discards_sectors += statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_list[0].to_string(),
                    key3: "stat_discards_sectors".to_string(),
                })?
                .per_second_value;
        }
        println!(
            "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
            timestamp.format("%H:%M:%S"),
            "",
            total_reads_completed_success
                + total_writes_completed_success
                + total_discards_completed_success,
            total_reads_completed_success,
            total_writes_completed_success,
            total_discards_completed_success,
            total_reads_sectors,
            total_writes_sectors,
            total_discards_sectors,
        );
    } else {
        for disk_name in disk_list {
            let timestamp = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_completed_success".to_string(),
                })?
                .last_timestamp;
            // reads
            let reads_completed_success = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_completed_success".to_string(),
                })?
                .per_second_value;
            let reads_merged = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_merged".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_merged".to_string(),
                })?
                .per_second_value;
            let reads_bytes = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_sectors".to_string(),
                })?
                .per_second_value
                * 512_f64; // convert 512 bytes sector reads to bytes
            let reads_bytes_total = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_sectors".to_string(),
                })?
                .delta_value
                * 512_f64; // convert 512 bytes sector reads to bytes
            let reads_time_ms = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_reads_time_spent_ms".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_reads_time_spent_ms".to_string(),
                })?
                .per_second_value;
            // writes
            let writes_completed_success = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_writes_completed_success".to_string(),
                })?
                .per_second_value;
            let writes_merged = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_merged".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_writes_merged".to_string(),
                })?
                .per_second_value;
            let writes_bytes = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_writes_sectors".to_string(),
                })?
                .per_second_value
                * 512_f64; // convert 512 bytes sector reads to bytes
            let writes_bytes_total = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_writes_sectors".to_string(),
                })?
                .delta_value
                * 512_f64; // convert 512 bytes sector reads to bytes
            let writes_time_ms = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_writes_time_spent_ms".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_writes_time_spent_ms".to_string(),
                })?
                .per_second_value;
            // discards
            let discards_completed_success = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_completed_success".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_discards_completed_success".to_string(),
                })?
                .per_second_value;
            let discards_merged = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_merged".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_discards_merged".to_string(),
                })?
                .per_second_value;
            let discards_bytes = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_sectors".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_discards_sectors".to_string(),
                })?
                .per_second_value
                * 512_f64; // convert 512 bytes sector reads to bytes
            let discards_time_ms = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_discards_time_spent_ms".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_discards_time_spent_ms".to_string(),
                })?
                .per_second_value;
            //
            let queue_size = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "stat_ios_weighted_time_spent_ms".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "stat_ios_weighted_time_spent_ms".to_string(),
                })?
                .per_second_value
                / 1000_f64; // convert milliseconds to seconds
            let inflight_reads = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "inflight_reads".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "inflight_reads".to_string(),
                })?
                .last_value;
            let inflight_writes = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "inflight_writes".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "inflight_writes".to_string(),
                })?
                .last_value;
            let queue_nr_requests = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "queue_nr_requests".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "queue_nr_requests".to_string(),
                })?
                .last_value;
            let current_max_io_size = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "queue_max_sectors_kb".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "queue_max_sectors_kb".to_string(),
                })?
                .last_value;
            let limit_max_io_size = statistics
                .get(&(
                    "blockdevice".to_string(),
                    disk_name.to_string(),
                    "queue_max_hw_sectors_kb".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "blockdevice".to_string(),
                    key2: disk_name.to_string(),
                    key3: "queue_max_hw_sectors_kb".to_string(),
                })?
                .last_value;

            let mut total_average_request_size =
                (reads_bytes + writes_bytes) / (reads_completed_success + writes_completed_success);
            total_average_request_size = if total_average_request_size.is_nan() {
                0_f64
            } else {
                total_average_request_size
            };
            let mut total_average_request_time = (reads_time_ms + writes_time_ms)
                / (reads_completed_success + writes_completed_success);
            total_average_request_time = if total_average_request_time.is_nan() {
                0_f64
            } else {
                total_average_request_time
            };
            let mut reads_percentage_merged =
                (reads_merged / (reads_merged + reads_completed_success)) * 100_f64;
            reads_percentage_merged = if reads_percentage_merged.is_nan() {
                0_f64
            } else {
                reads_percentage_merged
            };
            let mut writes_percentage_merged =
                (writes_merged / (writes_merged + writes_completed_success)) * 100_f64;
            writes_percentage_merged = if writes_percentage_merged.is_nan() {
                0_f64
            } else {
                writes_percentage_merged
            };
            let mut reads_average_time = reads_time_ms / reads_completed_success;
            reads_average_time = if reads_average_time.is_nan() {
                0_f64
            } else {
                reads_average_time
            };
            let mut writes_average_time = writes_time_ms / writes_completed_success;
            writes_average_time = if writes_average_time.is_nan() {
                0_f64
            } else {
                writes_average_time
            };
            let mut reads_average_request_size = reads_bytes / reads_completed_success;
            reads_average_request_size = if reads_average_request_size.is_nan() {
                0_f64
            } else {
                reads_average_request_size
            };
            let mut writes_average_request_size = writes_bytes / writes_completed_success;
            writes_average_request_size = if writes_average_request_size.is_nan() {
                0_f64
            } else {
                writes_average_request_size
            };
            let mut discards_average_time = discards_time_ms / discards_completed_success;
            discards_average_time = if discards_average_time.is_nan() {
                0_f64
            } else {
                discards_average_time
            };

            match output {
                "sar-d" => {
                    println!(
                        "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                        timestamp.format("%H:%M:%S"),
                        disk_name,
                        reads_completed_success + writes_completed_success,
                        reads_bytes / (1024_f64 * 1024_f64),
                        writes_bytes / (1024_f64 * 1024_f64),
                        total_average_request_size / (1024_f64 * 1024_f64),
                        queue_size,
                        total_average_request_time,
                    );
                }
                "iostat" => {
                    println!(
                        "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                        timestamp.format("%H:%M:%S"),
                        disk_name,
                        reads_completed_success + writes_completed_success,
                        reads_bytes / (1024_f64 * 1024_f64),
                        writes_bytes / (1024_f64 * 1024_f64),
                        reads_bytes_total / (1024_f64 * 1024_f64),
                        writes_bytes_total / (1024_f64 * 1024_f64),
                    );
                }
                "iostat-x" => {
                    println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                        timestamp.format("%H:%M:%S"),
                        disk_name,
                        reads_completed_success,
                        writes_completed_success,
                        reads_bytes / (1024_f64 * 1024_f64),
                        writes_bytes / (1024_f64 * 1024_f64),
                        reads_merged,
                        writes_merged,
                        reads_percentage_merged,
                        writes_percentage_merged,
                        reads_average_time,
                        writes_average_time,
                        queue_size,
                        reads_average_request_size / (1024_f64 * 1024_f64),
                        writes_average_request_size / (1024_f64 * 1024_f64),
                    );
                }
                "ioq" => {
                    println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.0} {:10.0} {:10.0}",
                        timestamp.format("%H:%M:%S"),
                        disk_name,
                        reads_completed_success,
                        reads_bytes / (1024_f64 * 1024_f64),
                        reads_merged,
                        reads_average_time,
                        writes_completed_success,
                        writes_bytes / (1024_f64 * 1024_f64),
                        writes_merged,
                        writes_average_time,
                        discards_completed_success,
                        discards_bytes / (1024_f64 * 1024_f64),
                        discards_merged,
                        discards_average_time,
                        queue_size,
                        queue_nr_requests,
                        inflight_reads,
                        inflight_writes,
                    );
                }
                "ios" => {
                    println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                        timestamp.format("%H:%M:%S"),
                        disk_name,
                        reads_completed_success,
                        reads_bytes / (1024_f64 * 1024_f64),
                        reads_merged,
                        reads_average_time,
                        reads_average_request_size / (1024_f64 * 1024_f64),
                        writes_completed_success,
                        writes_bytes / (1024_f64 * 1024_f64),
                        writes_merged,
                        writes_average_time,
                        writes_average_request_size / (1024_f64 * 1024_f64),
                        discards_completed_success,
                        discards_bytes / (1024_f64 * 1024_f64),
                        discards_merged,
                        discards_average_time,
                        current_max_io_size / 1024_f64,
                        limit_max_io_size / 1024_f64,
                    );
                }
                &_ => todo!(),
            }
        }
    }
    Ok(())
}
