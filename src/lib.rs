use proc_sys_parser::{schedstat, stat, meminfo, diskstats, net_dev};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use proc_sys_parser::stat::CpuStat;

#[derive(Debug)]
pub struct ProcData
{
    pub timestamp: DateTime<Local>,
    pub stat: stat::ProcStat,
    pub schedstat: schedstat::ProcSchedStat,
    pub meminfo: meminfo::ProcMemInfo,
    pub diskstats: diskstats::ProcDiskStats,
    pub net_dev: net_dev::ProcNetDev,
}

#[derive(Debug)]
pub struct Statistic
{
    pub last_timestamp: DateTime<Local>,
    pub last_value: f64,
    pub delta_value: f64,
    pub per_second_value: f64,
    pub new_value: bool,
}

pub async fn read_proc_data() -> ProcData
{
    let timestamp = Local::now();
    let proc_stat = stat::read();
    let proc_schedstat = schedstat::read();
    let proc_meminfo = meminfo::read();
    let proc_diskstats = diskstats::read();
    let proc_netdev = net_dev::read();
    ProcData {
        timestamp: timestamp,
        stat: proc_stat,
        schedstat: proc_schedstat,
        meminfo: proc_meminfo,
        diskstats: proc_diskstats,
        net_dev: proc_netdev,
    }
}

pub async fn process_data(proc_data: ProcData, statistics: &mut HashMap<(String, String), Statistic>)
{
    process_stat_data(proc_data, statistics).await;
}

pub async fn process_stat_data(proc_data: ProcData, statistics: &mut HashMap<(String, String), Statistic>)
{
    cpu_statistics(proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in proc_data.stat.cpu_individual {
        cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }
    single_statistic("stat", "context_switches", proc_data.timestamp, proc_data.stat.context_switches, statistics).await;
    single_statistic("stat", "processes", proc_data.timestamp, proc_data.stat.processes, statistics).await;
    single_statistic("stat", "processes_running", proc_data.timestamp, proc_data.stat.processes_running, statistics).await;
    single_statistic("stat", "processes_blocked", proc_data.timestamp, proc_data.stat.processes_blocked, statistics).await;
    single_statistic("stat", "interrupts_total", proc_data.timestamp, proc_data.stat.interrupts.first().cloned().unwrap(), statistics).await;
    single_statistic("stat", "softirq_total", proc_data.timestamp, proc_data.stat.softirq.first().cloned().unwrap(), statistics).await;
}
pub async fn single_statistic(
    category: &str,
    name: &str,
    timestamp: DateTime<Local>,
    value: u64,
    statistics: &mut HashMap<(String, String), Statistic>,
)
{
    statistics.entry((category.to_string(), name.to_string()))
        .and_modify(|row| {
            row.delta_value = value as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = value as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: value as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );

}
pub async fn cpu_statistics(cpu_data: CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String), Statistic>)
{
    statistics.entry((cpu_data.name.clone(), "user".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.user as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.user as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.user as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "nice".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.nice as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.nice as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.nice as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "system".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.system as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.system as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.system as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "idle".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.idle as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.idle as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.idle as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "iowait".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.iowait as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.iowait as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.iowait as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "irq".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.irq as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.irq as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.irq as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "softirq".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.softirq as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.softirq as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.softirq as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "steal".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.steal as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.steal as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.steal as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "guest".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.guest as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.guest as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.guest as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry((cpu_data.name.clone(), "guest_nice".to_string()))
        .and_modify(|row| {
            row.delta_value = cpu_data.guest_nice as f64 - row.last_value;
            row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = cpu_data.guest_nice as f64;
            row.last_timestamp = timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: timestamp,
                last_value: cpu_data.guest_nice as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
}
pub async fn print_cpu(statistics: &HashMap<(String, String), Statistic>)
{
    let timestamp = statistics.get(&("cpu".to_string(), "user".to_string())).unwrap().last_timestamp;
    let user = statistics.get(&("cpu".to_string(), "user".to_string())).unwrap().delta_value;
    let nice = statistics.get(&("cpu".to_string(), "nice".to_string())).unwrap().delta_value;
    let system = statistics.get(&("cpu".to_string(), "system".to_string())).unwrap().delta_value;
    let iowait = statistics.get(&("cpu".to_string(), "iowait".to_string())).unwrap().delta_value;
    let steal = statistics.get(&("cpu".to_string(), "steal".to_string())).unwrap().delta_value;
    let irq = statistics.get(&("cpu".to_string(), "irq".to_string())).unwrap().delta_value;
    let softirq = statistics.get(&("cpu".to_string(), "softirq".to_string())).unwrap().delta_value;
    let guest_user = statistics.get(&("cpu".to_string(), "guest".to_string())).unwrap().delta_value;
    let guest_nice = statistics.get(&("cpu".to_string(), "guest_nice".to_string())).unwrap().delta_value;
    let idle = statistics.get(&("cpu".to_string(), "idle".to_string())).unwrap().delta_value;
    let total = user+nice+system+iowait+steal+irq+softirq+guest_user+guest_nice+idle;
    println!("{:8} {:3} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
             timestamp.format("%H:%M:%S"),
             "all",
             user/total* 100.,
             nice/total* 100.,
             system/total* 100.,
             iowait/total* 100.,
             steal/total* 100.,
             idle/total* 100.,
    );
}