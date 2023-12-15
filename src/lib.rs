use chrono::{DateTime, Local};
use std::collections::HashMap;

pub mod stat;
pub mod schedstat;
pub mod meminfo;
pub mod diskstats;
pub mod net_dev;

use stat::process_stat_data;
use schedstat::process_schedstat_data;
use meminfo::process_meminfo_data;
use diskstats::process_diskstats_data;
use net_dev::process_net_dev_data;

#[derive(Debug)]
pub struct ProcData
{
    pub timestamp: DateTime<Local>,
    pub stat: proc_sys_parser::stat::ProcStat,
    pub schedstat: proc_sys_parser::schedstat::ProcSchedStat,
    pub meminfo: proc_sys_parser::meminfo::ProcMemInfo,
    pub diskstats: proc_sys_parser::diskstats::ProcDiskStats,
    pub net_dev: proc_sys_parser::net_dev::ProcNetDev,
}

#[derive(Debug, Default)]
pub struct Statistic
{
    pub last_timestamp: DateTime<Local>,
    pub last_value: f64,
    pub delta_value: f64,
    pub per_second_value: f64,
    pub updated_value: bool,
}

pub async fn read_proc_data() -> ProcData
{
    let timestamp = Local::now();
    let proc_stat = proc_sys_parser::stat::read();
    let proc_schedstat = proc_sys_parser::schedstat::read();
    let proc_meminfo = proc_sys_parser::meminfo::read();
    let proc_diskstats = proc_sys_parser::diskstats::read();
    let proc_netdev = proc_sys_parser::net_dev::read();
    ProcData {
        timestamp,
        stat: proc_stat,
        schedstat: proc_schedstat,
        meminfo: proc_meminfo,
        diskstats: proc_diskstats,
        net_dev: proc_netdev,
    }
}

pub async fn process_data(proc_data: ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    process_stat_data(&proc_data, statistics).await;
    process_schedstat_data(&proc_data, statistics).await;
    process_meminfo_data(&proc_data, statistics).await;
    process_diskstats_data(&proc_data, statistics).await;
    process_net_dev_data(&proc_data, statistics).await;
}

pub async fn single_statistic(
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