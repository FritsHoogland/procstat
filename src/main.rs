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
    #[clap(name = "mpstat-P-ALL")]
    MpstatPAll,
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
    /// Print header
    #[arg(short = 'n', long, value_name = "nr", default_value = "30")]
    header_print: u64,
}
#[tokio::main]
async fn main()
{
    let args = Opts::parse();

    let mut interval = time::interval(Duration::from_secs(args.interval));

    let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    let mut output_counter = 0_u64;
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;
        process_data(data, &mut statistics).await;

        let print_header = args.header_print % output_counter == 0;
        match args.output
        {
            OutputOptions::SarU => print_all_cpu(&statistics, "sar-u", print_header).await,
            OutputOptions::SarUAll => print_all_cpu(&statistics, "sar-u-ALL", print_header).await,
            OutputOptions::CpuAll => print_all_cpu(&statistics, "cpu-all", print_header).await,
            OutputOptions::MpstatPAll => print_per_cpu(&statistics, "mpstat-P-ALL").await,
            OutputOptions::PerCpuAll => print_per_cpu(&statistics, "per-cpu-all").await,
        }
        output_counter += 1;
    }
}
