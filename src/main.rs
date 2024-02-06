use tokio::time::Instant;

use std::process;
use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use log::info;
use env_logger;
use chrono::Local;

// this is a feature-gated option to perform memory usage analysis.
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

mod processor;
mod webserver;
mod archiver;
mod app;

use processor::HistoricalData;
use archiver::{archive, reader};
use app::app;

static LABEL_AREA_SIZE_LEFT: i32 = 100;
static LABEL_AREA_SIZE_RIGHT: i32 = 100;
static LABEL_AREA_SIZE_BOTTOM: i32 = 50;
static CAPTION_STYLE_FONT: &str = "monospace";
static CAPTION_STYLE_FONT_SIZE: i32 = 30;
static MESH_STYLE_FONT: &str = "monospace";
static MESH_STYLE_FONT_SIZE: i32 = 17;
static LABELS_STYLE_FONT: &str = "monospace";
static LABELS_STYLE_FONT_SIZE: i32 = 15;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputOptions {
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
    Ioq,
    Ios,
    Schedstat,
}

#[derive(Debug, Parser, Clone)]
#[clap(version, about, long_about = None)]
pub struct Opts {
    /// Interval
    #[arg(short = 'i', long, value_name = "time (s)", default_value = "1")]
    interval: u64,
    /// run Until
    #[arg(short = 'u', long, value_name = "run until cycle nr")]
    until: Option<u64>,
    /// Output
    #[arg(short = 'o', long, value_name = "option", value_enum, default_value_t = OutputOptions::SarU )]
    output: OutputOptions,
    /// Print header
    #[arg(short = 'n', long, value_name = "print header interval (rows)", default_value = "30")]
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
    /// archiver interval minutes
    #[arg(short = 'I', long, value_name = "archiver interval (minutes)", default_value = "10")]
    archiver_interval: i64,
    /// graph buffer width
    #[arg(short = 'W', long, value_name = "graph buffer width", default_value = "1800")]
    graph_width: u32,
    /// graph buffer heighth
    #[arg(short = 'H', long, value_name = "graph buffer height", default_value = "1200")]
    graph_height: u32,
}

static HISTORY: Lazy<HistoricalData> = Lazy::new(|| {
    HistoricalData::new(Opts::parse().history)
});

static ARGS: Lazy<Opts> = Lazy::new(|| { Opts::parse() });

#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap(); 

    env_logger::init();
    info!("Start procstat");
    let timer = Instant::now();

    // spawn the ctrlc thead
    ctrlc::set_handler(move || { 
        if ARGS.archiver { archive(Local::now()) }
        info!("End procstat, total time: {:?}", timer.elapsed());
        process::exit(0);
    }).unwrap();

    // spawn the webserver 
    if ARGS.webserver || ARGS.read.is_some() {
        tokio::spawn( async move {
            webserver::webserver().await;
        });
    }
    // spawn the archiver; only linux
    if ARGS.archiver { 
        tokio::spawn( async move { 
            archiver::archiver().await 
        }); 
    };

    // reader function.
    // execution loops in the reader if called.
    if ARGS.read.is_some() { 
        reader(ARGS.read.as_ref().unwrap().to_string()).await; 
    }

    // run the fetching and CLI output.
    app().await;

    info!("End procstat, total time: {:?}", timer.elapsed());
}
