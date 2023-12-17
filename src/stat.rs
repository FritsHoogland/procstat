use std::collections::{HashMap, BTreeSet};
use chrono::{DateTime, Local};
use proc_sys_parser::stat::CpuStat;
use crate::common::{ProcData, single_statistic, Statistic};

pub async fn process_stat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    process_cpu_statistics(&proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in &proc_data.stat.cpu_individual {
        process_cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }
    single_statistic("stat", "","context_switches", proc_data.timestamp, proc_data.stat.context_switches, statistics).await;
    single_statistic("stat", "", "processes", proc_data.timestamp, proc_data.stat.processes, statistics).await;
    single_statistic("stat", "", "processes_running", proc_data.timestamp, proc_data.stat.processes_running, statistics).await;
    single_statistic("stat", "", "processes_blocked", proc_data.timestamp, proc_data.stat.processes_blocked, statistics).await;
    single_statistic("stat", "", "interrupts_total", proc_data.timestamp, proc_data.stat.interrupts.first().cloned().unwrap(), statistics).await;
    single_statistic("stat", "", "softirq_total", proc_data.timestamp, proc_data.stat.softirq.first().cloned().unwrap(), statistics).await;
}

pub async fn process_cpu_statistics(cpu_data: &CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    let cpu_name = match cpu_data.name.as_str()
    {
        "cpu" => "all",
        cpunr => cpunr,
    };
    macro_rules! add_cpu_data_field_to_statistics {
        ($($field_name:ident),*) => {
            $(
                single_statistic("stat", cpu_name, stringify!($field_name), timestamp, cpu_data.$field_name, statistics).await;
            )*
        };
    }
    add_cpu_data_field_to_statistics!(user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice);
}

pub async fn print_all_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str, print_header: bool)
{
    if print_header
    {
        match output
        {
            "sar-u" => {
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
                         "Timestamp",
                         "cpu",
                         "%usr",
                         "%nice",
                         "%sys",
                         "%iowait",
                         "%steal",
                         "%idle",
                );
            },
            "sar-u-ALL" => {
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
            },
            "cpu-all" => {
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
            },
            &_ => todo! {},
        }
    }
    if !statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().last_timestamp;
    let user = statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().per_second_value;
    let nice = statistics.get(&("stat".to_string(), "all".to_string(), "nice".to_string())).unwrap().per_second_value;
    let system = statistics.get(&("stat".to_string(), "all".to_string(), "system".to_string())).unwrap().per_second_value;
    let iowait = statistics.get(&("stat".to_string(), "all".to_string(), "iowait".to_string())).unwrap().per_second_value;
    let steal = statistics.get(&("stat".to_string(), "all".to_string(), "steal".to_string())).unwrap().per_second_value;
    let irq = statistics.get(&("stat".to_string(), "all".to_string(), "irq".to_string())).unwrap().per_second_value;
    let softirq = statistics.get(&("stat".to_string(), "all".to_string(), "softirq".to_string())).unwrap().per_second_value;
    let guest_user = statistics.get(&("stat".to_string(), "all".to_string(), "guest".to_string())).unwrap().per_second_value;
    let guest_nice = statistics.get(&("stat".to_string(), "all".to_string(), "guest_nice".to_string())).unwrap().per_second_value;
    let idle = statistics.get(&("stat".to_string(), "all".to_string(), "idle".to_string())).unwrap().per_second_value;
    let total = user+nice+system+iowait+steal+irq+softirq+guest_user+guest_nice+idle;
    let scheduler_running = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_running".to_string())).unwrap().per_second_value/10_000_000_f64;
    let scheduler_waiting = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_waiting".to_string())).unwrap().per_second_value/10_000_000_f64;
    match output
    {
        "sar-u" => {
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100.,
                     nice/total*100.,
                     system/total*100.,
                     iowait/total*100.,
                     steal/total*100.,
                     idle/total*100.,
            );
        },
        "sar-u-ALL" => {
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100.,
                     nice/total*100.,
                     system/total*100.,
                     iowait/total*100.,
                     steal/total*100.,
                     irq/total*100.,
                     softirq/total*100.,
                     guest_user/total*100.,
                     guest_nice/total*100.,
                     idle/total*100.,
            );
        },
        "cpu-all" => {
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/1000.,
                     nice/1000.,
                     system/1000.,
                     iowait/1000.,
                     steal/1000.,
                     irq/1000.,
                     softirq/1000.,
                     guest_user/1000.,
                     guest_nice/1000.,
                     idle/1000.,
                     scheduler_running/1000.,
                     scheduler_waiting/1000.,
            );
        },
        &_ => todo!{},
    }
}
pub async fn print_per_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str)
{
    let cpu_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "stat" || group == "schedstat")
        .map(|(_, cpu_specification, _)| cpu_specification)
        .filter(|cpu_specification| cpu_specification.starts_with("cpu") || *cpu_specification == "all")
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics.get(&("stat".to_string(), cpu_list[0].to_string(), "user".to_string())).unwrap().updated_value { return };

    match output
    {
        "mpstat-P-ALL" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
        },
        "per-cpu-all" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
        },
        &_ => todo! {},
    }
    for cpu_name in cpu_list
    {
        let timestamp = statistics.get(&("stat".to_string(), cpu_name.to_string(), "user".to_string())).unwrap().last_timestamp;
        let user = statistics.get(&("stat".to_string(), cpu_name.to_string(), "user".to_string())).unwrap().per_second_value;
        let nice = statistics.get(&("stat".to_string(), cpu_name.to_string(), "nice".to_string())).unwrap().per_second_value;
        let system = statistics.get(&("stat".to_string(), cpu_name.to_string(), "system".to_string())).unwrap().per_second_value;
        let iowait = statistics.get(&("stat".to_string(), cpu_name.to_string(), "iowait".to_string())).unwrap().per_second_value;
        let steal = statistics.get(&("stat".to_string(), cpu_name.to_string(), "steal".to_string())).unwrap().per_second_value;
        let irq = statistics.get(&("stat".to_string(), cpu_name.to_string(), "irq".to_string())).unwrap().per_second_value;
        let softirq = statistics.get(&("stat".to_string(), cpu_name.to_string(), "softirq".to_string())).unwrap().per_second_value;
        let guest_user = statistics.get(&("stat".to_string(), cpu_name.to_string(), "guest".to_string())).unwrap().per_second_value;
        let guest_nice = statistics.get(&("stat".to_string(), cpu_name.to_string(), "guest_nice".to_string())).unwrap().per_second_value;
        let idle = statistics.get(&("stat".to_string(), cpu_name.to_string(), "idle".to_string())).unwrap().per_second_value;
        let total = user + nice + system + iowait + steal + irq + softirq + guest_user + guest_nice + idle;
        let scheduler_running = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_running".to_string())).unwrap().per_second_value / 10_000_000_f64;
        let scheduler_waiting = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_waiting".to_string())).unwrap().per_second_value / 10_000_000_f64;
        match output
        {
            "mpstat-P-ALL" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / total * 100.,
                         nice / total * 100.,
                         system / total * 100.,
                         iowait / total * 100.,
                         irq / total * 100.,
                         softirq / total * 100.,
                         steal / total * 100.,
                         guest_user / total * 100.,
                         guest_nice / total * 100.,
                         idle / total * 100.,
                );
            },
            "per-cpu-all" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / 1000.,
                         nice / 1000.,
                         system / 1000.,
                         iowait / 1000.,
                         irq / 1000.,
                         softirq / 1000.,
                         steal / 1000.,
                         guest_user / 1000.,
                         guest_nice / 1000.,
                         idle / 1000.,
                         scheduler_running / 1000.,
                         scheduler_waiting / 1000.,
                );
            },
            &_ => todo! {},
        }
    }
}