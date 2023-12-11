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
    process_cpu_data(proc_data, statistics).await;
}

pub async fn process_cpu_data(proc_data: ProcData, statistics: &mut HashMap<(String, String), Statistic>)
{
    set_cpu_statistics(proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in proc_data.stat.cpu_individual {
        set_cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }

    //let mut individual_cpus = proc_data.stat.cpu_individual.iter().map(|row| row.name).collect();


    //for keys in statistics.keys().map(|(stat_type, _)| stat_type).filter("cpu")

}
pub async fn set_cpu_statistics(cpu_data: CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String), Statistic>)
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
    println!("{} {} {}",
             statistics.get(&("cpu".to_string(), "user".to_string())).unwrap().delta_value,
             statistics.get(&("cpu".to_string(), "nice".to_string())).unwrap().delta_value,
             statistics.get(&("cpu".to_string(), "system".to_string())).unwrap().delta_value,
    );
}