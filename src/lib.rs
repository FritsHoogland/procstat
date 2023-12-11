use proc_sys_parser::{schedstat, stat, meminfo, diskstats, net_dev};
use chrono::{DateTime, Local};
use std::collections::HashMap;

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
    statistics.entry(("cpu".to_string(), "user".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.user as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.user as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.user as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "nice".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.nice as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.nice as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.nice as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "system".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.system as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.system as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.system as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "idle".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.idle as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.idle as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.idle as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "iowait".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.iowait as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.iowait as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.iowait as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "irq".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.irq as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.irq as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.irq as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "softirq".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.softirq as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.softirq as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.softirq as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "steal".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.steal as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.steal as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.steal as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "guest".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.guest as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.guest as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.guest as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );
    statistics.entry(("cpu".to_string(), "guest_nice".to_string()))
        .and_modify(|row|{
            row.delta_value = proc_data.stat.cpu_total.guest_nice as f64 - row.last_value;
            row.per_second_value = row.delta_value / (proc_data.timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
            row.last_value = proc_data.stat.cpu_total.guest_nice as f64;
            row.last_timestamp = proc_data.timestamp;
            row.new_value = false;
        })
        .or_insert(
            Statistic {
                last_timestamp: proc_data.timestamp,
                last_value: proc_data.stat.cpu_total.guest_nice as f64,
                delta_value: 0.0,
                per_second_value: 0.0,
                new_value: true,
            }
        );

}