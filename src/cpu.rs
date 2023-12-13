use std::collections::HashMap;
use chrono::{DateTime, Local};
use proc_sys_parser::stat::CpuStat;
use crate::{ProcData, single_statistic, Statistic};

pub async fn process_stat_data(proc_data: ProcData, statistics: &mut HashMap<(String, String), Statistic>)
{
    cpu_statistics(proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in proc_data.stat.cpu_individual {
        cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }
    single_statistic("cpu", "context_switches", proc_data.timestamp, proc_data.stat.context_switches, statistics).await;
    single_statistic("cpu", "processes", proc_data.timestamp, proc_data.stat.processes, statistics).await;
    single_statistic("cpu", "processes_running", proc_data.timestamp, proc_data.stat.processes_running, statistics).await;
    single_statistic("cpu", "processes_blocked", proc_data.timestamp, proc_data.stat.processes_blocked, statistics).await;
    single_statistic("cpu", "interrupts_total", proc_data.timestamp, proc_data.stat.interrupts.first().cloned().unwrap(), statistics).await;
    single_statistic("cpu", "softirq_total", proc_data.timestamp, proc_data.stat.softirq.first().cloned().unwrap(), statistics).await;
}

pub async fn cpu_statistics(cpu_data: CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String), Statistic>)
{
    macro_rules! add_cpu_data_field_to_statistics {
        ($($field_name:ident),*) => {
            $(
                statistics.entry((cpu_data.name.to_string(), stringify!($field_name).to_string()))
                .and_modify(|row| {
                    row.delta_value = cpu_data.$field_name as f64 - row.last_value;
                    row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
                    row.last_value = cpu_data.$field_name as f64;
                    row.last_timestamp = timestamp;
                    row.new_value = false;
                })
                .or_insert(
                    Statistic {
                        last_timestamp: timestamp,
                        last_value: cpu_data.$field_name as f64,
                        delta_value: 0.0,
                        per_second_value: 0.0,
                        new_value: true,
                    }
                );
            )*
        };
    }
    add_cpu_data_field_to_statistics!(user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice);
}

pub async fn print_cpu(statistics: &HashMap<(String, String), Statistic>)
{
    if statistics.get(&("cpu".to_string(), "user".to_string())).unwrap().new_value { return };
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

pub async fn print_per_cpu(statistics: &HashMap<(String, String), Statistic>)
{
    let cpu_list: Vec<_> = statistics.keys().map(|(cpu_number, _)| cpu_number).filter(|cpu_number| cpu_number.starts_with("cpu") && cpu_number.len() > 3).collect();
    println!("{:?}", cpu_list);
    /*
    if statistics.get(&("cpu".to_string(), "user".to_string())).unwrap().new_value { return };
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
     */
}