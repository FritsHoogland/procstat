use std::collections::HashMap;
use crate::{ProcData, single_statistic, Statistic};

pub async fn process_schedstat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{

    let mut scheduler_total_time_running = 0;
    let mut scheduler_total_time_waiting = 0;
    let mut scheduler_total_timeslices = 0;
    for cpu_data in &proc_data.schedstat.cpu {
        single_statistic("schedstat", format!("cpu{}", cpu_data.iter().nth(0).unwrap()).as_str(), "time_running", proc_data.timestamp, *cpu_data.iter().nth(7).unwrap(), statistics).await;
        scheduler_total_time_running += cpu_data.iter().nth(7).unwrap();
        single_statistic("schedstat", format!("cpu{}", cpu_data.iter().nth(0).unwrap()).as_str(), "time_waiting", proc_data.timestamp, *cpu_data.iter().nth(8).unwrap(), statistics).await;
        scheduler_total_time_waiting += cpu_data.iter().nth(8).unwrap();
        single_statistic("schedstat", format!("cpu{}", cpu_data.iter().nth(0).unwrap()).as_str(), "timeslices", proc_data.timestamp, *cpu_data.iter().nth(9).unwrap(), statistics).await;
        scheduler_total_timeslices += cpu_data.iter().nth(9).unwrap();
    }
    single_statistic("schedstat", "all","time_running", proc_data.timestamp, scheduler_total_time_running, statistics).await;
    single_statistic("schedstat", "all","time_waiting", proc_data.timestamp, scheduler_total_time_waiting, statistics).await;
    single_statistic("schedstat", "all","timeslices", proc_data.timestamp, scheduler_total_timeslices, statistics).await;
}