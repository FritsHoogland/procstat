use std::collections::{HashMap,HashSet};
use chrono::{DateTime, Local};
use proc_sys_parser::stat::CpuStat;
use crate::{ProcData, single_statistic, Statistic};

pub async fn process_stat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    cpu_statistics(&proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in &proc_data.stat.cpu_individual {
        cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }
    single_statistic("stat", "","context_switches", proc_data.timestamp, proc_data.stat.context_switches, statistics).await;
    single_statistic("stat", "", "processes", proc_data.timestamp, proc_data.stat.processes, statistics).await;
    single_statistic("stat", "", "processes_running", proc_data.timestamp, proc_data.stat.processes_running, statistics).await;
    single_statistic("stat", "", "processes_blocked", proc_data.timestamp, proc_data.stat.processes_blocked, statistics).await;
    single_statistic("stat", "", "interrupts_total", proc_data.timestamp, proc_data.stat.interrupts.first().cloned().unwrap(), statistics).await;
    single_statistic("stat", "", "softirq_total", proc_data.timestamp, proc_data.stat.softirq.first().cloned().unwrap(), statistics).await;
}

pub async fn cpu_statistics(cpu_data: &CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    let cpu_name = match cpu_data.name.as_str()
    {
        "cpu" => "all",
        cpunr => cpunr,
    };
    macro_rules! add_cpu_data_field_to_statistics {
        ($($field_name:ident),*) => {
            $(
                statistics.entry(("stat".to_string(), cpu_name.to_string(), stringify!($field_name).to_string()))
                .and_modify(|row| {
                    row.delta_value = cpu_data.$field_name as f64 - row.last_value;
                    row.per_second_value = row.delta_value / (timestamp.signed_duration_since(row.last_timestamp).num_milliseconds() as f64 / 1000_f64);
                    row.last_value = cpu_data.$field_name as f64;
                    row.last_timestamp = timestamp;
                    row.updated_value = true;
                })
                .or_insert(
                    Statistic {
                        last_timestamp: timestamp,
                        last_value: cpu_data.$field_name as f64,
                        delta_value: 0.0,
                        per_second_value: 0.0,
                        updated_value: false,
                    }
                );
            )*
        };
    }
    add_cpu_data_field_to_statistics!(user, nice, system, idle, iowait, irq, softirq, steal, guest, guest_nice);
}

pub async fn print_all_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str)
{
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
    let scheduler_running = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_running".to_string())).unwrap().per_second_value/1_000_000_f64;
    let scheduler_waiting = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_waiting".to_string())).unwrap().per_second_value/1_000_000_f64;
    match output
    {
        "sar-u" => {
            println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
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
            println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100.,
                     nice/total*100.,
                     system/total*100.,
                     iowait/total*100.,
                     steal/total*100.,
                     idle/total*100.,
                     irq/total*100.,
                     softirq/total*100.,
                     guest_user/total*100.,
                     guest_nice/total*100.,
            );
        },
        "cpu-all" => {
            println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/1000.,
                     nice/1000.,
                     system/1000.,
                     iowait/1000.,
                     steal/1000.,
                     idle/1000.,
                     irq/1000.,
                     softirq/1000.,
                     guest_user/1000.,
                     guest_nice/1000.,
                     scheduler_running/1000.,
                     scheduler_waiting/1000.,
            );
        },
        &_ => todo!{},
    }
}
pub async fn print_per_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str)
{
    let mut cpu_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "stat" || group == "schedstat")
        .map(|(_, cpu_specification, _)| cpu_specification)
        .filter(|cpu_specification| cpu_specification.starts_with("cpu") || *cpu_specification == "all")
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();
    cpu_list.sort();
    for cpu_name in cpu_list
    {
        if !statistics.get(&("stat".to_string(), cpu_name.to_string(), "user".to_string())).unwrap().updated_value { return };
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
            "mpstat-u" => {
                println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / total * 100.,
                         nice / total * 100.,
                         system / total * 100.,
                         iowait / total * 100.,
                         steal / total * 100.,
                         idle / total * 100.,
                );
            },
            "mpstat-u-ALL" => {
                println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / total * 100.,
                         nice / total * 100.,
                         system / total * 100.,
                         iowait / total * 100.,
                         steal / total * 100.,
                         idle / total * 100.,
                         irq / total * 100.,
                         softirq / total * 100.,
                         guest_user / total * 100.,
                         guest_nice / total * 100.,
                );
            },
            "per-cpu-all" => {
                println!("{:8} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / 1000.,
                         nice / 1000.,
                         system / 1000.,
                         iowait / 1000.,
                         steal / 1000.,
                         idle / 1000.,
                         irq / 1000.,
                         softirq / 1000.,
                         guest_user / 1000.,
                         guest_nice / 1000.,
                         scheduler_running / 1000.,
                         scheduler_waiting / 1000.,
                );
            },
            &_ => todo! {},
        }
    }
}

/*
pub async fn print_per_cpu(statistics: &HashMap<(String, String, String), Statistic>)
{
    let mut cpu_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "stat" || group == "schedstat")
        .map(|(_, cpu_specification, _)| cpu_specification)
        .filter(|cpu_specification| cpu_specification.starts_with("cpu") || *cpu_specification == "all")
        //.filter(|cpu_number| cpu_number.starts_with("cpu"))
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();
    cpu_list.sort();
    for cpu_name in cpu_list
    {
        if statistics.get(&("stat".to_string(), cpu_name.to_string(), "user".to_string())).unwrap().new_value { return };
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
        let scheduler_running = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_running".to_string())).unwrap().per_second_value/1000000_f64;
        let scheduler_waiting = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_waiting".to_string())).unwrap().per_second_value/1000000_f64;
        //let total = user+nice+system+iowait+steal+irq+softirq+guest_user+guest_nice+idle;


    }

    //println!("{:?}", cpu_list);
    /*

     */
}
 */
