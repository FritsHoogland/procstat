use proc_sys_parser::{schedstat as schedstat_internal, stat as stat_internal, meminfo as meminfo_internal, diskstats as diskstats_internal, net_dev as net_dev_internal};
use chrono::{DateTime, Local};
use std::collections::HashMap;

pub mod stat;
pub mod schedstat;

use stat::process_stat_data;
use schedstat::process_schedstat_data;

#[derive(Debug)]
pub struct ProcData
{
    pub timestamp: DateTime<Local>,
    pub stat: stat_internal::ProcStat,
    pub schedstat: schedstat_internal::ProcSchedStat,
    pub meminfo: meminfo_internal::ProcMemInfo,
    pub diskstats: diskstats_internal::ProcDiskStats,
    pub net_dev: net_dev_internal::ProcNetDev,
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
    let proc_stat = stat_internal::read();
    let proc_schedstat = schedstat_internal::read();
    let proc_meminfo = meminfo_internal::read();
    let proc_diskstats = diskstats_internal::read();
    let proc_netdev = net_dev_internal::read();
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
    println!("{:?}", &proc_data.schedstat);
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