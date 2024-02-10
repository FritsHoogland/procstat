use crate::ARGS;
use std::time::Duration;
use tokio::time::{self, MissedTickBehavior};
use std::collections::HashMap;
use anyhow::{Result, Context};
use crate::processor::Statistic;
use crate::processor::read_proc_data_and_process;
use crate::OutputOptions;

use crate::processor::stat::{print_all_cpu, print_per_cpu};
use crate::processor::vmstat::print_vmstat;
use crate::processor::blockdevice::print_diskstats;
use crate::processor::meminfo::print_meminfo;
use crate::processor::net_dev::print_net_dev;
use crate::processor::pressure::print_psi;
use crate::processor::loadavg::print_loadavg;

pub async fn app() -> Result<()> {
    let mut interval = time::interval(Duration::from_secs(ARGS.interval));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut current_statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    let mut output_counter = 0_u64;
    loop {
        interval.tick().await;

        read_proc_data_and_process(&mut current_statistics).await.with_context(|| "Processor: read proc data and process")?;

        if ! ARGS.deamon {
            let print_header = output_counter % ARGS.header_print == 0;
            match ARGS.output {
                OutputOptions::SarU => print_all_cpu(&current_statistics, "sar-u", print_header).await,
                OutputOptions::SarB => print_vmstat(&current_statistics, "sar-B", print_header).await,
                OutputOptions::Sarb => print_diskstats(&current_statistics, "sar-b", print_header).await,
                OutputOptions::SarUAll => print_all_cpu(&current_statistics, "sar-u-ALL", print_header).await,
                OutputOptions::CpuAll => print_all_cpu(&current_statistics, "cpu-all", print_header).await,
                OutputOptions::Schedstat => print_per_cpu(&current_statistics, "schedstat").await,
                OutputOptions::MpstatPAll => print_per_cpu(&current_statistics, "mpstat-P-ALL").await,
                OutputOptions::PerCpuAll => print_per_cpu(&current_statistics, "per-cpu-all").await,
                OutputOptions::SarD => print_diskstats(&current_statistics, "sar-d", print_header).await,
                OutputOptions::Iostat => print_diskstats(&current_statistics, "iostat", print_header).await,
                OutputOptions::IostatX => print_diskstats(&current_statistics, "iostat-x", print_header).await,
                OutputOptions::Ioq => print_diskstats(&current_statistics, "ioq", print_header).await,
                OutputOptions::Ios => print_diskstats(&current_statistics, "ios", print_header).await,
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

            if let Some(until) = ARGS.until {
                if until < output_counter { break };
            };
        }
    }
    Ok(())
}

