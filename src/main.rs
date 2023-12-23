use time::Duration;
use tokio::time;
use std::collections::HashMap;
use std::process;
use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;
use axum::{Router, routing::get};

pub mod common;
pub mod stat;
pub mod schedstat;
pub mod meminfo;
pub mod diskstats;
pub mod net_dev;
mod webserver;

use common::{read_proc_data, process_data, Statistic, add_to_history, HistoricalData};
use stat::{print_all_cpu, print_per_cpu};
use diskstats::print_diskstats;
use meminfo::print_meminfo;
use net_dev::print_net_dev;
use webserver::{root_handler, cpu_handler_html, cpu_handler_generate};

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
    Iostat,
    IostatX,
    SarR,
    #[clap(name = "sar-r-ALL")]
    SarRAll,
    #[clap(name = "sar-n-DEV")]
    SarNDev,
    #[clap(name = "sar-n-EDEV")]
    SarNEdev,
}
#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
pub struct Opts
{
    /// Interval
    #[arg(short = 'i', long, value_name = "time (s)", default_value = "1")]
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
}
static HISTORY: Lazy<HistoricalData> = Lazy::new(|| {
    HistoricalData::new(Opts::parse().history)
});

#[tokio::main]
async fn main()
{
    let args = Opts::parse();

    ctrlc::set_handler(move || {
        println!("{:#?}", HISTORY);
        process::exit(0);
    }).unwrap();

    // spawn the webserver thread
    let _ = tokio::spawn( async {
        let app = Router::new()
            .route("/cpu_all", get(cpu_handler_html))
            .route("/cpu_all_plot", get(cpu_handler_generate))
            .route("/", get(root_handler));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();
        axum::serve(listener, app.into_make_service()).await.unwrap();
    });

    let mut interval = time::interval(Duration::from_secs(args.interval));

    let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    let mut output_counter = 0_u64;
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;
        process_data(data, &mut statistics).await;
        add_to_history(&statistics).await;

        let print_header = output_counter % args.header_print == 0;
        match args.output
        {
            OutputOptions::SarU => print_all_cpu(&statistics, "sar-u", print_header).await,
            OutputOptions::SarUAll => print_all_cpu(&statistics, "sar-u-ALL", print_header).await,
            OutputOptions::CpuAll => print_all_cpu(&statistics, "cpu-all", print_header).await,
            OutputOptions::MpstatPAll => print_per_cpu(&statistics, "mpstat-P-ALL").await,
            OutputOptions::PerCpuAll => print_per_cpu(&statistics, "per-cpu-all").await,
            OutputOptions::SarD => print_diskstats(&statistics, "sar-d").await,
            OutputOptions::Iostat => print_diskstats(&statistics, "iostat").await,
            OutputOptions::IostatX => print_diskstats(&statistics, "iostat-x").await,
            OutputOptions::SarR => print_meminfo(&statistics, "sar-r", print_header).await,
            OutputOptions::SarRAll => print_meminfo(&statistics, "sar-r-ALL", print_header).await,
            OutputOptions::SarNDev => print_net_dev(&statistics, "sar-n-DEV").await,
            OutputOptions::SarNEdev => print_net_dev(&statistics, "sar-n-EDEV").await,
        }
        output_counter += 1;
    }
}
