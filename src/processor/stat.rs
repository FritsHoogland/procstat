use crate::add_list_of_u64_data_to_statistics;
use crate::processor::{
    single_statistic_option_u64, single_statistic_u64, ProcData, ProcessorError, Statistic,
};
use crate::Data;
use crate::ARGS;
use crate::DATA;
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::stat::ProcStat;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

pub async fn read_stat_proc_data() -> Result<ProcStat> {
    let proc_stat = proc_sys_parser::stat::read()?;
    debug!("{:?}", proc_stat);
    Ok(proc_stat)
}

pub async fn process_stat_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    process_cpu_statistics(&proc_data.stat.cpu_total, proc_data.timestamp, statistics)
        .await
        .with_context(|| "Cpu total statistics")?;
    for cpu_stat in &proc_data.stat.cpu_individual {
        process_cpu_statistics(cpu_stat, proc_data.timestamp, statistics)
            .await
            .with_context(|| "Cpu individual statistics")?;
    }
    add_list_of_u64_data_to_statistics!(
        stat,
        "",
        proc_data.timestamp,
        proc_data,
        stat,
        statistics,
        context_switches,
        processes,
        processes_running,
        processes_blocked
    );
    single_statistic_u64(
        "stat",
        "",
        "interrupts_total",
        proc_data.timestamp,
        proc_data.stat.interrupts.first().cloned().unwrap(),
        statistics,
    )
    .await;
    single_statistic_u64(
        "stat",
        "",
        "softirq_total",
        proc_data.timestamp,
        proc_data.stat.softirq.first().cloned().unwrap(),
        statistics,
    )
    .await;
    Ok(())
}

pub async fn process_cpu_statistics(
    cpu_data: &proc_sys_parser::stat::CpuStat,
    timestamp: DateTime<Local>,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    let cpu_name = match cpu_data.name.as_str() {
        "cpu" => "all",
        cpunr => cpunr,
    };
    macro_rules! add_cpu_data_field_to_statistics_u64 {
        ($($field_name:ident),*) => {
            $(
                single_statistic_u64("stat", cpu_name, stringify!($field_name), timestamp, cpu_data.$field_name, statistics).await;
            )*
        };
    }
    add_cpu_data_field_to_statistics_u64!(user, nice, system, idle);
    macro_rules! add_cpu_data_field_to_statistics_option_u64 {
        ($($field_name:ident),*) => {
            $(
                single_statistic_option_u64("stat", cpu_name, stringify!($field_name), timestamp, cpu_data.$field_name, statistics).await;
            )*
        };
    }
    add_cpu_data_field_to_statistics_option_u64!(iowait, irq, softirq, steal, guest, guest_nice);
    Ok(())
}

// sar cpu statistics: https://github.com/sysstat/sysstat/blob/dbc0b6a59fea1437025208aa12a612181c804fb4/rd_stats.c#L76
pub async fn print_all_cpu(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool,
) -> Result<()> {
    if print_header {
        match output {
            "sar-u" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp", "cpu", "%usr", "%nice", "%sys", "%iowait", "%steal", "%idle",
                );
            }
            "sar-u-ALL" => {
                println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "cpu",
                         "%usr",
                         "%nice",
                         "%sys",
                         "%iowait",
                         "%steal",
                         "%irq",
                         "%soft",
                         "%guest",
                         "%gnice",
                         "%idle",
                );
            }
            "cpu-all" => {
                println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "cpu",
                         "usr_s",
                         "nice_s",
                         "sys_s",
                         "iowait_s",
                         "steal_s",
                         "irq_s",
                         "soft_s",
                         "guest_s",
                         "gnice_s",
                         "idle_s",
                         "sched_r_s",
                         "sched_w_s",
                );
            }
            "sar-w" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10}",
                    "Timestamp", "", "proc/s", "cswch/s",
                );
            }
            &_ => todo! {},
        }
    }
    if !statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .last_timestamp;
    let user = statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .per_second_value;
    let nice = statistics
        .get(&("stat".to_string(), "all".to_string(), "nice".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "nice".to_string(),
        })?
        .per_second_value;
    let system = statistics
        .get(&("stat".to_string(), "all".to_string(), "system".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "system".to_string(),
        })?
        .per_second_value;
    let iowait = statistics
        .get(&("stat".to_string(), "all".to_string(), "iowait".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "iowait".to_string(),
        })?
        .per_second_value;
    let steal = statistics
        .get(&("stat".to_string(), "all".to_string(), "steal".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "steal".to_string(),
        })?
        .per_second_value;
    let irq = statistics
        .get(&("stat".to_string(), "all".to_string(), "irq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "irq".to_string(),
        })?
        .per_second_value;
    let softirq = statistics
        .get(&("stat".to_string(), "all".to_string(), "softirq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "softirq".to_string(),
        })?
        .per_second_value;
    let guest_user = statistics
        .get(&("stat".to_string(), "all".to_string(), "guest".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest".to_string(),
        })?
        .per_second_value;
    let guest_nice = statistics
        .get(&(
            "stat".to_string(),
            "all".to_string(),
            "guest_nice".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest_nice".to_string(),
        })?
        .per_second_value;
    let idle = statistics
        .get(&("stat".to_string(), "all".to_string(), "idle".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "idle".to_string(),
        })?
        .per_second_value;
    let total =
        user + nice + system + iowait + steal + irq + softirq + guest_user + guest_nice + idle;
    match output {
        "sar-u" => {
            println!(
                "{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "all",
                user / total * 100_f64,
                nice / total * 100_f64,
                system / total * 100_f64,
                iowait / total * 100_f64,
                steal / total * 100_f64,
                idle / total * 100_f64,
            );
        }
        "sar-u-ALL" => {
            println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100_f64,
                     nice/total*100_f64,
                     system/total*100_f64,
                     iowait/total*100_f64,
                     steal/total*100_f64,
                     irq/total*100_f64,
                     softirq/total*100_f64,
                     guest_user/total*100_f64,
                     guest_nice/total*100_f64,
                     idle/total*100_f64,
            );
        }
        "cpu-all" => {
            let scheduler_running = statistics
                .get(&(
                    "schedstat".to_string(),
                    "all".to_string(),
                    "time_running".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "schedstat".to_string(),
                    key2: "all".to_string(),
                    key3: "time_running".to_string(),
                })?
                .per_second_value
                / 1_000_000_f64;
            let scheduler_waiting = statistics
                .get(&(
                    "schedstat".to_string(),
                    "all".to_string(),
                    "time_waiting".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "schedstat".to_string(),
                    key2: "all".to_string(),
                    key3: "time_waiting".to_string(),
                })?
                .per_second_value
                / 1_000_000_f64;
            println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/1000_f64,
                     nice/1000_f64,
                     system/1000_f64,
                     iowait/1000_f64,
                     steal/1000_f64,
                     irq/1000_f64,
                     softirq/1000_f64,
                     guest_user/1000_f64,
                     guest_nice/1000_f64,
                     idle/1000_f64,
                     scheduler_running/1000_f64,
                     scheduler_waiting/1000_f64,
            );
        }
        "sar-w" => {
            let processes = statistics
                .get(&("stat".to_string(), "".to_string(), "processes".to_string()))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "stat".to_string(),
                    key2: "".to_string(),
                    key3: "processes".to_string(),
                })?
                .per_second_value;
            let context_switches = statistics
                .get(&(
                    "stat".to_string(),
                    "".to_string(),
                    "context_switches".to_string(),
                ))
                .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                    hashmap: "statistics".to_string(),
                    key1: "stat".to_string(),
                    key2: "".to_string(),
                    key3: "context_switches".to_string(),
                })?
                .per_second_value;
            println!(
                "{:10} {:7}    {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "",
                processes,
                context_switches,
            );
        }
        &_ => todo! {},
    }
    Ok(())
}
pub async fn print_per_cpu(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
) -> Result<()> {
    let cpu_list: Vec<_> = statistics
        .keys()
        .filter(|(group, _, _)| group == "stat" || group == "schedstat")
        .map(|(_, cpu_specification, _)| cpu_specification)
        .filter(|cpu_specification| {
            cpu_specification.starts_with("cpu") || *cpu_specification == "all"
        })
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics
        .get(&(
            "stat".to_string(),
            cpu_list[0].to_string(),
            "user".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: cpu_list[0].to_string(),
            key3: "user".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };

    match output {
        "mpstat-P-ALL" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                     "Timestamp",
                     "cpu",
                     "%usr",
                     "%nice",
                     "%sys",
                     "%iowait",
                     "%irq",
                     "%soft",
                     "%steal",
                     "%guest",
                     "%gnice",
                     "%idle",
            );
        }
        "per-cpu-all" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                     "Timestamp",
                     "cpu",
                     "usr_s",
                     "nice_s",
                     "sys_s",
                     "iowait_s",
                     "steal_s",
                     "irq_s",
                     "soft_s",
                     "guest_s",
                     "gnice_s",
                     "idle_s",
                     "sched_r_s",
                     "sched_w_s",
            );
        }
        "schedstat" => {
            println!(
                "{:10} {:7}    {:>10} {:>10} {:>10}",
                "Timestamp", "cpu", "sched_w/s", "sched_r/s", "avg slice",
            );
        }
        &_ => todo! {},
    }
    for cpu_name in cpu_list {
        let timestamp = statistics
            .get(&("stat".to_string(), cpu_name.to_string(), "user".to_string()))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "user".to_string(),
            })?
            .last_timestamp;
        let user = statistics
            .get(&("stat".to_string(), cpu_name.to_string(), "user".to_string()))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "user".to_string(),
            })?
            .per_second_value;
        let nice = statistics
            .get(&("stat".to_string(), cpu_name.to_string(), "nice".to_string()))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "nice".to_string(),
            })?
            .per_second_value;
        let system = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "system".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "system".to_string(),
            })?
            .per_second_value;
        let iowait = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "iowait".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "iowait".to_string(),
            })?
            .per_second_value;
        let steal = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "steal".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "steal".to_string(),
            })?
            .per_second_value;
        let irq = statistics
            .get(&("stat".to_string(), cpu_name.to_string(), "irq".to_string()))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "irq".to_string(),
            })?
            .per_second_value;
        let softirq = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "softirq".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "softirq".to_string(),
            })?
            .per_second_value;
        let guest_user = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "guest".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "guest".to_string(),
            })?
            .per_second_value;
        let guest_nice = statistics
            .get(&(
                "stat".to_string(),
                cpu_name.to_string(),
                "guest_nice".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "guest_nice".to_string(),
            })?
            .per_second_value;
        let idle = statistics
            .get(&("stat".to_string(), cpu_name.to_string(), "idle".to_string()))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "stat".to_string(),
                key2: cpu_name.to_string(),
                key3: "idle".to_string(),
            })?
            .per_second_value;
        let total =
            user + nice + system + iowait + steal + irq + softirq + guest_user + guest_nice + idle;
        let scheduler_running = statistics
            .get(&(
                "schedstat".to_string(),
                cpu_name.to_string(),
                "time_running".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "schedstat".to_string(),
                key2: cpu_name.to_string(),
                key3: "time_running".to_string(),
            })?
            .per_second_value
            / 1_000_000_f64;
        let scheduler_waiting = statistics
            .get(&(
                "schedstat".to_string(),
                cpu_name.to_string(),
                "time_waiting".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "schedstat".to_string(),
                key2: cpu_name.to_string(),
                key3: "time_waiting".to_string(),
            })?
            .per_second_value
            / 1_000_000_f64;
        let scheduler_slices = statistics
            .get(&(
                "schedstat".to_string(),
                cpu_name.to_string(),
                "timeslices".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "schedstat".to_string(),
                key2: cpu_name.to_string(),
                key3: "timeslices".to_string(),
            })?
            .per_second_value;
        let scheduler_slice_avg_length = if scheduler_slices == 0_f64 {
            0_f64
        } else {
            scheduler_running / scheduler_slices
        };
        match output {
            "mpstat-P-ALL" => {
                println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / total * 100_f64,
                         nice / total * 100_f64,
                         system / total * 100_f64,
                         iowait / total * 100_f64,
                         irq / total * 100_f64,
                         softirq / total * 100_f64,
                         steal / total * 100_f64,
                         guest_user / total * 100_f64,
                         guest_nice / total * 100_f64,
                         idle / total * 100_f64,
                );
            }
            "per-cpu-all" => {
                println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / 1000_f64,
                         nice / 1000_f64,
                         system / 1000_f64,
                         iowait / 1000_f64,
                         irq / 1000_f64,
                         softirq / 1000_f64,
                         steal / 1000_f64,
                         guest_user / 1000_f64,
                         guest_nice / 1000_f64,
                         idle / 1000_f64,
                         scheduler_running / 1000_f64,
                         scheduler_waiting / 1000_f64,
                );
            }
            "schedstat" => {
                println!(
                    "{:10} {:7}    {:10.2} {:10.2} {:10.6}",
                    timestamp.format("%H:%M:%S"),
                    cpu_name,
                    scheduler_waiting / 1000_f64,
                    scheduler_running / 1000_f64,
                    scheduler_slice_avg_length / 1000_f64,
                );
            }
            &_ => todo! {},
        }
    }
    Ok(())
}

pub async fn add_cpu_total_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if !statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .last_timestamp;
    let user = statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let nice = statistics
        .get(&("stat".to_string(), "all".to_string(), "nice".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "nice".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let system = statistics
        .get(&("stat".to_string(), "all".to_string(), "system".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "system".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let iowait = statistics
        .get(&("stat".to_string(), "all".to_string(), "iowait".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "iowait".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let steal = statistics
        .get(&("stat".to_string(), "all".to_string(), "steal".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "steal".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let irq = statistics
        .get(&("stat".to_string(), "all".to_string(), "irq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "irq".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let softirq = statistics
        .get(&("stat".to_string(), "all".to_string(), "softirq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "softirq".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let guest = statistics
        .get(&("stat".to_string(), "all".to_string(), "guest".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let guest_nice = statistics
        .get(&(
            "stat".to_string(),
            "all".to_string(),
            "guest_nice".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest_nice".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    let idle = statistics
        .get(&("stat".to_string(), "all".to_string(), "idle".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "idle".to_string(),
        })?
        .per_second_value
        / 1000_f64;
    //let total = user+nice+system+iowait+steal+irq+softirq+guest_user+guest_nice+idle;
    let scheduler_running = statistics
        .get(&(
            "schedstat".to_string(),
            "all".to_string(),
            "time_running".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "schedstat".to_string(),
            key2: "all".to_string(),
            key3: "time_running".to_string(),
        })?
        .per_second_value
        / 1_000_000_f64
        / 1000_f64;
    let scheduler_waiting = statistics
        .get(&(
            "schedstat".to_string(),
            "all".to_string(),
            "time_waiting".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "schedstat".to_string(),
            key2: "all".to_string(),
            key3: "time_waiting".to_string(),
        })?
        .per_second_value
        / 1_000_000_f64
        / 1000_f64;
    Data::push_cpu(CpuStat {
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
    })
    .await;
    /*
        DATA.cpu.write().unwrap().push_back(CpuStat {
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
    */
    Ok(())
}

impl Data {
    pub async fn push_cpu(cpustat: CpuStat) {
        while DATA.cpu.read().unwrap().len() >= ARGS.history {
            DATA.cpu.write().unwrap().pop_front();
        }
        DATA.cpu.write().unwrap().push_back(cpustat);
    }
}
