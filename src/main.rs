use time::Duration;
use tokio::time;
use std::collections::HashMap;
use clap::{Parser, ValueEnum};

use procstat::{read_proc_data, process_data, Statistic, stat};
use stat::{print_all_cpu, print_per_cpu};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputOptions
{
    SarU,
    #[clap(name = "sar-u-ALL")]
    SarUAll,
    CpuAll,
    MpstatU,
    #[clap(name = "mpstat-u-ALL")]
    MpstatUAll,
    PerCpuAll,
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
}
#[tokio::main]
async fn main()
{
    let args = Opts::parse();

    let mut interval = time::interval(Duration::from_secs(args.interval));

    let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;
        process_data(data, &mut statistics).await;

        match args.output
        {
            OutputOptions::SarU => print_all_cpu(&statistics, "sar-u").await,
            OutputOptions::SarUAll => print_all_cpu(&statistics, "sar-u-ALL").await,
            OutputOptions::CpuAll => print_all_cpu(&statistics, "cpu-all").await,
            OutputOptions::MpstatU => print_per_cpu(&statistics, "mpstat-u").await,
            OutputOptions::MpstatUAll => print_per_cpu(&statistics, "mpstat-u-ALL").await,
            OutputOptions::PerCpuAll => print_per_cpu(&statistics, "per-cpu-all").await,
        }
    }
}
