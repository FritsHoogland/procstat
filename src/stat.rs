use plotters::coord::Shift;
use std::collections::{HashMap, BTreeSet};
use chrono::{DateTime, Local};
use plotters::backend::RGBPixel;
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::prelude::*;
use plotters::style::full_palette::{GREEN_A400, GREY, LIGHTBLUE, PURPLE, YELLOW_600};
use crate::HISTORY;
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, MESH_STYLE_FONT_SIZE, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT};
use crate::common::{ProcData, single_statistic_u64, single_statistic_option_u64, Statistic};
use crate::loadavg::load_plot;
use crate::pressure::pressure_cpu_some_plot;
use crate::{GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH};
use crate::add_list_of_u64_data_to_statistics;

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
pub async fn process_stat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    process_cpu_statistics(&proc_data.stat.cpu_total, proc_data.timestamp, statistics).await;
    for cpu_stat in &proc_data.stat.cpu_individual {
        process_cpu_statistics(cpu_stat, proc_data.timestamp, statistics).await;
    }
    add_list_of_u64_data_to_statistics!(stat, "", proc_data.timestamp, proc_data, stat, statistics, context_switches, processes, processes_running, processes_blocked);
    single_statistic_u64("stat", "", "interrupts_total", proc_data.timestamp, proc_data.stat.interrupts.first().cloned().unwrap(), statistics).await;
    single_statistic_u64("stat", "", "softirq_total", proc_data.timestamp, proc_data.stat.softirq.first().cloned().unwrap(), statistics).await;
}

pub async fn process_cpu_statistics(cpu_data: &proc_sys_parser::stat::CpuStat, timestamp: DateTime<Local>, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    let cpu_name = match cpu_data.name.as_str()
    {
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
}

// sar cpu statistics: https://github.com/sysstat/sysstat/blob/dbc0b6a59fea1437025208aa12a612181c804fb4/rd_stats.c#L76
pub async fn print_all_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str, print_header: bool)
{
    if print_header
    {
        match output
        {
            "sar-u" => {
                println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
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
            },
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
    let scheduler_running = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_running".to_string())).unwrap().per_second_value/1_000_000_f64;
    let scheduler_waiting = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_waiting".to_string())).unwrap().per_second_value/1_000_000_f64;
    match output
    {
        "sar-u" => {
            println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100_f64,
                     nice/total*100_f64,
                     system/total*100_f64,
                     iowait/total*100_f64,
                     steal/total*100_f64,
                     idle/total*100_f64,
            );
        },
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
        },
        "cpu-all" => {
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
        },
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
        let scheduler_running = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_running".to_string())).unwrap().per_second_value / 1_000_000_f64;
        let scheduler_waiting = statistics.get(&("schedstat".to_string(), cpu_name.to_string(), "time_waiting".to_string())).unwrap().per_second_value / 1_000_000_f64;
        match output
        {
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
            },
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
            },
            &_ => todo! {},
        }
    }
}

pub async fn add_cpu_total_to_history(statistics: &HashMap<(String, String, String), Statistic>)
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

pub fn create_cpu_load_pressure_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    cpu_total_plot(&mut multi_backend, 0);
    load_plot(&mut multi_backend, 1);
    pressure_cpu_some_plot(&mut multi_backend, 2);
}
pub fn create_cpu_load_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    cpu_total_plot(&mut multi_backend, 0);
    load_plot(&mut multi_backend, 1);
}
pub fn create_cpu_plot(
    buffer: &mut [u8]
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((1, 1));
    cpu_total_plot(&mut multi_backend, 0);
}
fn cpu_total_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
    let historical_data_read = HISTORY.cpu.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|cpustat| cpustat.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|cpustat| cpustat.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value_cpu = historical_data_read
        .iter()
        .map(|cpustat| cpustat.user+cpustat.nice+cpustat.system+cpustat.iowait+cpustat.steal+cpustat.irq+cpustat.softirq+cpustat.idle)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_schedstat = historical_data_read
        .iter()
        .map(|cpustat| cpustat.scheduler_running+cpustat.scheduler_waiting)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value = vec![high_value_cpu, high_value_schedstat]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Total CPU usage", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("CPU per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // colour picker
    let mut palette99_pick = 1_usize;
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .take(1)
                                                .map(|cpustat| (cpustat.timestamp, cpustat.scheduler_waiting)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    // scheduler times
    // scheduler waiting = scheduler_waiting + scheduler_running
    let min_scheduler_wait = historical_data_read.iter().map(|cpustat| cpustat.scheduler_waiting).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_scheduler_wait = historical_data_read.iter().map(|cpustat| cpustat.scheduler_waiting).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.scheduler_waiting + cpustat.scheduler_running)), 0.0, PURPLE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "scheduler wait", min_scheduler_wait, max_scheduler_wait, latest.scheduler_waiting))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE.filled()));
    //palette99_pick += 1;
    // scheduler running
    let min_scheduler_run = historical_data_read.iter().map(|cpustat| cpustat.scheduler_running).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_scheduler_run = historical_data_read.iter().map(|cpustat| cpustat.scheduler_running).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.scheduler_running)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "scheduler run", min_scheduler_run, max_scheduler_run, latest.scheduler_running))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //palette99_pick += 1;
    // cpu states
    // guest_nice = guest_nice + guest_user + softirq + irq + steal + iowait + system + nice + user
    let min_guest_nice = historical_data_read.iter().map(|cpustat| cpustat.guest_nice).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_guest_nice = historical_data_read.iter().map(|cpustat| cpustat.guest_nice).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.guest_nice + cpustat.guest + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, YELLOW_600))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "guest_nice", min_guest_nice, max_guest_nice, latest.guest_nice))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], YELLOW_600.filled()));
    //palette99_pick += 1;
    //
    // guest_user = guest_user + softirq + irq + steal + iowait + system + nice + user
    let min_guest_user = historical_data_read.iter().map(|cpustat| cpustat.guest).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_guest_user = historical_data_read.iter().map(|cpustat| cpustat.guest).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.guest + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, GREEN_A400))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "guest_user", min_guest_user, max_guest_user, latest.guest))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN_A400.filled()));
    palette99_pick += 1;
    //
    // softirq = softirq + irq + steal + iowait + system + nice + user
    let min_softirq = historical_data_read.iter().map(|cpustat| cpustat.softirq).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_softirq = historical_data_read.iter().map(|cpustat| cpustat.softirq).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, LIGHTBLUE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "softirq", min_softirq, max_softirq, latest.softirq))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTBLUE.filled()));
    palette99_pick += 1;
    //
    // irq = irq + steal + iowait + system + nice + user
    let min_irq = historical_data_read.iter().map(|cpustat| cpustat.irq).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_irq = historical_data_read.iter().map(|cpustat| cpustat.irq).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "irq", min_irq, max_irq, latest.irq))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // steal = steal + iowait + system + nice + user
    let min_steal = historical_data_read.iter().map(|cpustat| cpustat.steal).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_steal = historical_data_read.iter().map(|cpustat| cpustat.steal).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "steal", min_steal, max_steal, latest.steal))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //palette99_pick += 1;
    //
    // iowait = iowait + system + nice + user
    let min_iowait = historical_data_read.iter().map(|cpustat| cpustat.iowait).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_iowait = historical_data_read.iter().map(|cpustat| cpustat.iowait).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, GREY))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "iowait", min_iowait, max_iowait, latest.iowait))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREY.filled()));
    //palette99_pick += 1;
    //
    // system = system + nice + user
    let min_system = historical_data_read.iter().map(|cpustat| cpustat.system).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_system = historical_data_read.iter().map(|cpustat| cpustat.system).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.system + cpustat.nice + cpustat.user)), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "system", min_system, max_system, latest.system))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //palette99_pick += 1;
    //
    // nice = nice + user
    let min_nice = historical_data_read.iter().map(|cpustat| cpustat.nice).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_nice = historical_data_read.iter().map(|cpustat| cpustat.nice).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.nice + cpustat.user)), 0.0, YELLOW))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "nice", min_nice, max_nice, latest.nice))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], YELLOW.filled()));
    //palette99_pick += 1;
    //
    // user
    let min_user = historical_data_read.iter().map(|cpustat| cpustat.user).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_user = historical_data_read.iter().map(|cpustat| cpustat.user).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, cpustat.user)), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "user", min_user, max_user, latest.user))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    //
    // draw a line for total cpu
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .map(|cpustat| (cpustat.timestamp, (cpustat.guest_nice + cpustat.guest + cpustat.idle + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user).round())),
                                            ShapeStyle { color: RED.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:10} {:10} {:10.2}", "total (v)cpu", "", "", (latest.idle + latest.guest_nice + latest.guest + latest.softirq + latest.irq + latest.steal + latest.iowait + latest.system + latest.nice + latest.user).round()))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
