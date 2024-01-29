use time::Duration;
use tokio::time::{self, Instant, MissedTickBehavior};

use std::collections::HashMap;
use std::process;
use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use axum::{Router, routing::get};
use log::info;
use env_logger;
use std::sync::Arc;
use chrono::Local;

mod common;
mod stat;
mod schedstat;
mod meminfo;
mod blockdevice;
mod net_dev;
mod webserver;
mod loadavg;
mod pressure;
mod vmstat;
mod archiver;

use common::{read_proc_data_and_process, Statistic,};
use common::HistoricalData;
use archiver::{archive, reader};
use stat::{print_all_cpu, print_per_cpu};
use blockdevice::print_diskstats;
use meminfo::print_meminfo;
use net_dev::print_net_dev;
use pressure::print_psi;
use vmstat::print_vmstat;
use loadavg::print_loadavg;
use webserver::{root_handler, handler_html, handler_plotter};

static LABEL_AREA_SIZE_LEFT: i32 = 100;
static LABEL_AREA_SIZE_RIGHT: i32 = 100;
static LABEL_AREA_SIZE_BOTTOM: i32 = 50;
static CAPTION_STYLE_FONT: &str = "monospace";
static CAPTION_STYLE_FONT_SIZE: i32 = 30;
static MESH_STYLE_FONT: &str = "monospace";
static MESH_STYLE_FONT_SIZE: i32 = 17;
static LABELS_STYLE_FONT: &str = "monospace";
static LABELS_STYLE_FONT_SIZE: i32 = 15;

static GRAPH_BUFFER_WIDTH: u32 = 1800;
static GRAPH_BUFFER_HEIGHTH: u32 = 1250;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputOptions
{
    SarU,
    #[clap(name = "sar-u-ALL")]
    SarUAll,
    CpuAll,
    #[clap(name = "mpstat-P-ALL")]
    MpstatPAll,
    PerCpuAll,
    SarD,
    #[clap(name = "sar-b")]
    Sarb,
    #[clap(name = "sar-B")]
    SarB,
    #[clap(name = "sar-H")]
    SarH,
    #[clap(name = "sar-S")]
    SarS,
    Iostat,
    IostatX,
    SarR,
    #[clap(name = "sar-r-ALL")]
    SarRAll,
    #[clap(name = "sar-n-DEV")]
    SarNDev,
    #[clap(name = "sar-n-EDEV")]
    SarNEdev,
    #[clap(name = "sar-q-LOAD")]
    SarQLoad,
    SarQ,
    #[clap(name = "sar-q-CPU")]
    SarQCpu,
    #[clap(name = "sar-q-IO")]
    SarQIo,
    #[clap(name = "sar-q-MEM")]
    SarQMem,
    #[clap(name = "sar-W")]
    SarW,
    #[clap(name = "sar-w")]
    Sarw,
    Vmstat,
}

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct Opts {
    /// Interval
    #[arg(short = 'i', long, value_name = "time (s)", default_value = "5")]
    interval: u64,
    /// Output
    #[arg(short = 'o', long, value_name = "option", value_enum, default_value_t = OutputOptions::SarU )]
    output: OutputOptions,
    /// Print header
    #[arg(short = 'n', long, value_name = "nr", default_value = "30")]
    header_print: u64,
    /// History size
    #[arg(short = 's', long, value_name = "nr statistics", default_value = "10800")]
    history: usize,
    /// Read history (only read archives, no active fetching)
    #[arg(short = 'r', long, value_name = "read archives")]
    read: Option<String>,
    /// Enable webserver 
    #[arg(short = 'w', long, value_name = "enable webserver")]
    webserver: bool,
    /// Webserver port
    #[arg(short = 'P', long, value_name = "webserver port", default_value = "1111")]
    webserver_port: u64,
    /// Enable archiver 
    #[arg(short = 'A', long, value_name = "enable archiving")]
    archiver: bool,
    /// Deamon mode
    #[arg(short = 'D', long, value_name = "daemon mode")]
    deamon: bool,
}

static HISTORY: Lazy<HistoricalData> = Lazy::new(|| {
    HistoricalData::new(Opts::parse().history)
});

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Start procstat");
    let timer = Instant::now();
    let args = Opts::parse();

    ctrlc::set_handler(move || {
        if args.archiver {
            archive(Local::now());
        }
        info!("End procstat, total time: {:?}", timer.elapsed());
        process::exit(0);
    }).unwrap();

    // spawn the webserver 
    if args.webserver {
        let port = Arc::new(args.webserver_port);
        let port_clone = Arc::clone(&port);
        tokio::spawn( async move {
            let app = Router::new()
                .route("/handler/:plot_1/:plot_2", get(handler_html))
                .route("/plotter/:plot_1/:plot_2", get(handler_plotter))
                .route("/", get(root_handler));
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port_clone)).await.unwrap();
            axum::serve(listener, app.into_make_service()).await.unwrap();
        });
    }
    // spawn the archiver; only linux
    if args.archiver { tokio::spawn( async move { archiver::archiver().await; }); };

    if args.read.is_some() {
        let reader_files = Arc::new(args.read.as_ref().unwrap());
        let reader_files_clone = Arc::clone(&reader_files);
        reader(reader_files_clone.to_string());
    }

    let mut interval = time::interval(Duration::from_secs(args.interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut current_statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    let mut output_counter = 0_u64;
    loop
    {
        interval.tick().await;
        if args.read.is_some() { continue };

        read_proc_data_and_process(&mut current_statistics).await;

        if ! args.deamon {
            let print_header = output_counter % args.header_print == 0;
            match args.output {
                OutputOptions::SarU => print_all_cpu(&current_statistics, "sar-u", print_header).await,
                OutputOptions::SarB => print_vmstat(&current_statistics, "sar-B", print_header).await,
                OutputOptions::Sarb => print_diskstats(&current_statistics, "sar-b", print_header).await,
                OutputOptions::SarUAll => print_all_cpu(&current_statistics, "sar-u-ALL", print_header).await,
                OutputOptions::CpuAll => print_all_cpu(&current_statistics, "cpu-all", print_header).await,
                OutputOptions::MpstatPAll => print_per_cpu(&current_statistics, "mpstat-P-ALL").await,
                OutputOptions::PerCpuAll => print_per_cpu(&current_statistics, "per-cpu-all").await,
                OutputOptions::SarD => print_diskstats(&current_statistics, "sar-d", print_header).await,
                OutputOptions::Iostat => print_diskstats(&current_statistics, "iostat", print_header).await,
                OutputOptions::IostatX => print_diskstats(&current_statistics, "iostat-x", print_header).await,
                OutputOptions::SarH => print_meminfo(&current_statistics, "sar-H", print_header).await,
                OutputOptions::SarR => print_meminfo(&current_statistics, "sar-r", print_header).await,
                OutputOptions::SarRAll => print_meminfo(&current_statistics, "sar-r-ALL", print_header).await,
                OutputOptions::SarNDev => print_net_dev(&current_statistics, "sar-n-DEV").await,
                OutputOptions::SarNEdev => print_net_dev(&current_statistics, "sar-n-EDEV").await,
                OutputOptions::SarQCpu => print_psi(&current_statistics, "sar-q-CPU", print_header).await,
                OutputOptions::SarQLoad => print_loadavg(&current_statistics, "sar-q-LOAD", print_header).await,
                OutputOptions::SarQIo => print_psi(&current_statistics, "sar-q-IO", print_header).await,
                OutputOptions::SarQMem => print_psi(&current_statistics, "sar-q-MEM", print_header).await,
                OutputOptions::SarQ => print_loadavg(&current_statistics, "sar-q-LOAD", print_header).await,
                OutputOptions::SarS => print_meminfo(&current_statistics, "sar-S", print_header).await,
                OutputOptions::SarW => print_vmstat(&current_statistics, "sar-W", print_header).await,
                OutputOptions::Sarw => print_all_cpu(&current_statistics, "sar-w", print_header).await,
                OutputOptions::Vmstat => print_vmstat(&current_statistics, "vmstat", print_header).await,
            }
            output_counter += 1;
        }
    }
}
