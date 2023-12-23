use std::collections::HashMap;
use chrono::{DateTime, Local};
use bounded_vec_deque::BoundedVecDeque;
use std::sync::RwLock;
use crate::HISTORY;

use crate::stat::process_stat_data;
use crate::schedstat::process_schedstat_data;
use crate::meminfo::process_meminfo_data;
use crate::diskstats::process_diskstats_data;
use crate::net_dev::process_net_dev_data;
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
#[derive(Debug)]
pub struct CpuStat {
    pub timestamp: DateTime<Local>,
    pub user: f64,
    pub nice: f64,
    pub system: f64,
    pub idle: f64,
    pub iowait: f64,
    pub irq: f64,
    pub softirq: f64,
    pub steal: f64,
    pub guest: f64,
    pub guest_nice: f64,
    pub scheduler_running: f64,
    pub scheduler_waiting: f64,
}
#[derive(Debug)]
pub struct HistoricalData
{
    pub cpu: RwLock<BoundedVecDeque<CpuStat>>,
}

impl HistoricalData
{
    pub fn new(history: usize) -> HistoricalData {
        HistoricalData {
            cpu: RwLock::new(BoundedVecDeque::new(history)),
        }
    }
}

pub async fn add_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().last_timestamp;
    let user = statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().per_second_value/1000_f64;
    let nice = statistics.get(&("stat".to_string(), "all".to_string(), "nice".to_string())).unwrap().per_second_value/1000_f64;
    let system = statistics.get(&("stat".to_string(), "all".to_string(), "system".to_string())).unwrap().per_second_value/1000_f64;
    let iowait = statistics.get(&("stat".to_string(), "all".to_string(), "iowait".to_string())).unwrap().per_second_value/1000_f64;
    let steal = statistics.get(&("stat".to_string(), "all".to_string(), "steal".to_string())).unwrap().per_second_value/1000_f64;
    let irq = statistics.get(&("stat".to_string(), "all".to_string(), "irq".to_string())).unwrap().per_second_value/1000_f64;
    let softirq = statistics.get(&("stat".to_string(), "all".to_string(), "softirq".to_string())).unwrap().per_second_value/1000_f64;
    let guest= statistics.get(&("stat".to_string(), "all".to_string(), "guest".to_string())).unwrap().per_second_value/1000_f64;
    let guest_nice = statistics.get(&("stat".to_string(), "all".to_string(), "guest_nice".to_string())).unwrap().per_second_value/1000_f64;
    let idle = statistics.get(&("stat".to_string(), "all".to_string(), "idle".to_string())).unwrap().per_second_value/1000_f64;
    //let total = user+nice+system+iowait+steal+irq+softirq+guest_user+guest_nice+idle;
    let scheduler_running = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_running".to_string())).unwrap().per_second_value/1_000_000_f64/1000_f64;
    let scheduler_waiting = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_waiting".to_string())).unwrap().per_second_value/1_000_000_f64/1000_f64;
    HISTORY.cpu.write().unwrap().push_back( CpuStat {
        timestamp,
        user,
        nice,
        system,
        idle,
        iowait,
        irq,
        softirq,
        steal,
        guest,
        guest_nice,
        scheduler_running,
        scheduler_waiting,
    });
}

//static HISTORY: Lazy<HistoricalData> = Lazy::new(|| {
//    println!("initializing history");
//    HistoricalData::new(100)
//});

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
