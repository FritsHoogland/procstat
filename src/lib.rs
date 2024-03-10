
use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use processor::HistoricalData;

pub mod processor;
pub mod webserver;
pub mod archiver;
pub mod app;

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
    Free,
}

#[derive(Debug, Parser, Clone)]
#[clap(version, about, long_about = None)]
pub struct Opts {
    /// Interval
    #[arg(short = 'i', long, value_name = "time (s)", default_value = "1")]
    pub interval: u64,
    /// run Until
    #[arg(short = 'u', long, value_name = "run until cycle nr")]
    pub until: Option<u64>,
    /// Output
    #[arg(short = 'o', long, value_name = "option", value_enum, default_value_t = OutputOptions::SarU )]
    output: OutputOptions,
    /// Print header
    #[arg(short = 'n', long, value_name = "print header interval (rows)", default_value = "30")]
    pub header_print: u64,
    /// History size
    #[arg(short = 's', long, value_name = "nr statistics", default_value = "10800")]
    pub history: usize,
    /// Read history (only read archives, no active fetching)
    #[arg(short = 'r', long, value_name = "read archives")]
    pub read: Option<String>,
    /// Enable webserver 
    #[arg(short = 'w', long, value_name = "enable webserver")]
    pub webserver: bool,
    /// Webserver port
    #[arg(short = 'P', long, value_name = "webserver port", default_value = "1111")]
    pub webserver_port: u64,
    /// Enable archiver 
    #[arg(short = 'A', long, value_name = "enable archiving")]
    pub archiver: bool,
    /// Deamon mode
    #[arg(short = 'D', long, value_name = "daemon mode")]
    pub deamon: bool,
    /// archiver interval minutes
    #[arg(short = 'I', long, value_name = "archiver interval (minutes)", default_value = "10")]
    pub archiver_interval: i64,
    /// graph buffer width
    #[arg(short = 'W', long, value_name = "graph buffer width", default_value = "1800")]
    pub graph_width: u32,
    /// graph buffer heighth
    #[arg(short = 'H', long, value_name = "graph buffer height", default_value = "1200")]
    pub graph_height: u32,
}
static HISTORY: Lazy<HistoricalData> = Lazy::new(|| {
    HistoricalData::new(Opts::parse().history)
});

pub static ARGS: Lazy<Opts> = Lazy::new(|| { Opts::parse() });

