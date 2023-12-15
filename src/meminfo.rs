use std::collections::HashMap;
use crate::{ProcData, single_statistic, Statistic};

pub async fn process_meminfo_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    macro_rules! add_meminfo_data_to_statistics {
        ($($field_name:ident),*) => {
            $(
                single_statistic("meminfo", "", stringify!($field_name), proc_data.timestamp, proc_data.meminfo.$field_name, statistics).await;
            )*
        };
    }
    add_meminfo_data_to_statistics!(memtotal, memfree, memavailable, buffers, cached, swapcached, active, inactive, active_anon, inactive_anon, active_file, inactive_file, unevictable, mlocked, swaptotal, swapfree, zswap, zswapped, dirty, writeback, anonpages, mapped, shmem, kreclaimable, slab, sreclaimable, sunreclaim, kernelstack, shadowcallstack, pagetables, secpagetables, nfs_unstable, bounce, writebacktmp, commitlimit, committed_as, vmalloctotal, vmallocused, vmallocchunk, percpu, hardwarecorrupted, anonhugepages, shmemhugepages, shmempmdmapped, filehugepages, filepmdmapped, cmatotal, cmafree, hugepages_total, hugepages_free, hugepages_rsvd, hugepages_surp, hugepagesize, hugetlb);
    /*
    single_statistic("meminfo", "","memtotal", proc_data.timestamp, proc_data.meminfo.memtotal, statistics).await;
    single_statistic("meminfo", "","memfee", proc_data.timestamp, proc_data.meminfo.memfree, statistics).await;
    single_statistic("meminfo", "","memavailable", proc_data.timestamp, proc_data.meminfo.memavailable, statistics).await;
    single_statistic("meminfo", "","buffers", proc_data.timestamp, proc_data.meminfo.buffers, statistics).await;
    single_statistic("meminfo", "","cached", proc_data.timestamp, proc_data.meminfo.cached, statistics).await;
    single_statistic("meminfo", "","swapcached", proc_data.timestamp, proc_data.meminfo.swapcached, statistics).await;
    single_statistic("meminfo", "","active", proc_data.timestamp, proc_data.meminfo.active, statistics).await;
    single_statistic("meminfo", "","inactive", proc_data.timestamp, proc_data.meminfo.inactive, statistics).await;
    single_statistic("meminfo", "","active_anon", proc_data.timestamp, proc_data.meminfo.active_anon, statistics).await;
    single_statistic("meminfo", "","inactive_anon", proc_data.timestamp, proc_data.meminfo.inactive_anon, statistics).await;
    single_statistic("meminfo", "","active_file", proc_data.timestamp, proc_data.meminfo.active_file, statistics).await;
    single_statistic("meminfo", "","inactive_file", proc_data.timestamp, proc_data.meminfo.inactive_file, statistics).await;
    single_statistic("meminfo", "","unevictable", proc_data.timestamp, proc_data.meminfo.unevictable, statistics).await;
    single_statistic("meminfo", "","mlocked", proc_data.timestamp, proc_data.meminfo.mlocked, statistics).await;
    single_statistic("meminfo", "","swaptotal", proc_data.timestamp, proc_data.meminfo.swaptotal, statistics).await;
    single_statistic("meminfo", "","swapfree", proc_data.timestamp, proc_data.meminfo.swapfree, statistics).await;
    single_statistic("meminfo", "","zswap", proc_data.timestamp, proc_data.meminfo.zswap, statistics).await;
    single_statistic("meminfo", "","zswapped", proc_data.timestamp, proc_data.meminfo.zswapped, statistics).await;
    single_statistic("meminfo", "","dirty", proc_data.timestamp, proc_data.meminfo.dirty, statistics).await;
    single_statistic("meminfo", "","writeback", proc_data.timestamp, proc_data.meminfo.writeback, statistics).await;
    single_statistic("meminfo", "","anonpages", proc_data.timestamp, proc_data.meminfo.anonpages, statistics).await;
    single_statistic("meminfo", "","mapped", proc_data.timestamp, proc_data.meminfo.mapped, statistics).await;
    single_statistic("meminfo", "","shmem", proc_data.timestamp, proc_data.meminfo.shmem, statistics).await;
    single_statistic("meminfo", "","kreclaimable", proc_data.timestamp, proc_data.meminfo.kreclaimable, statistics).await;
    single_statistic("meminfo", "","slab", proc_data.timestamp, proc_data.meminfo.slab, statistics).await;
    single_statistic("meminfo", "","sreclaimable", proc_data.timestamp, proc_data.meminfo.sreclaimable, statistics).await;
    single_statistic("meminfo", "","sunreclaim", proc_data.timestamp, proc_data.meminfo.sunreclaim, statistics).await;
    single_statistic("meminfo", "","kernelstack", proc_data.timestamp, proc_data.meminfo.kernelstack, statistics).await;
    single_statistic("meminfo", "","shadowcallstack", proc_data.timestamp, proc_data.meminfo.shadowcallstack, statistics).await;
    single_statistic("meminfo", "","pagetables", proc_data.timestamp, proc_data.meminfo.pagetables, statistics).await;
    single_statistic("meminfo", "","secpagetables", proc_data.timestamp, proc_data.meminfo.secpagetables, statistics).await;
    single_statistic("meminfo", "","nfs_unstable", proc_data.timestamp, proc_data.meminfo.nfs_unstable, statistics).await;
    single_statistic("meminfo", "","bounce", proc_data.timestamp, proc_data.meminfo.bounce, statistics).await;
    single_statistic("meminfo", "","writebacktmp", proc_data.timestamp, proc_data.meminfo.writebacktmp, statistics).await;
    single_statistic("meminfo", "","commitlimit", proc_data.timestamp, proc_data.meminfo.commitlimit, statistics).await;
    single_statistic("meminfo", "","committed_as", proc_data.timestamp, proc_data.meminfo.committed_as, statistics).await;
    single_statistic("meminfo", "","vmalloctotal", proc_data.timestamp, proc_data.meminfo.vmalloctotal, statistics).await;
    single_statistic("meminfo", "","vmallocused", proc_data.timestamp, proc_data.meminfo.vmallocused, statistics).await;
    single_statistic("meminfo", "","vmallocchunk", proc_data.timestamp, proc_data.meminfo.vmallocchunk, statistics).await;
    single_statistic("meminfo", "","percpu", proc_data.timestamp, proc_data.meminfo.percpu, statistics).await;
    single_statistic("meminfo", "","hardwarecorrupted", proc_data.timestamp, proc_data.meminfo.hardwarecorrupted, statistics).await;
    single_statistic("meminfo", "","anonhugepages", proc_data.timestamp, proc_data.meminfo.anonhugepages, statistics).await;
    single_statistic("meminfo", "","shmemhugepages", proc_data.timestamp, proc_data.meminfo.shmemhugepages, statistics).await;
    single_statistic("meminfo", "","shmempmdmapped", proc_data.timestamp, proc_data.meminfo.shmempmdmapped, statistics).await;
    single_statistic("meminfo", "","filehugepages", proc_data.timestamp, proc_data.meminfo.filehugepages, statistics).await;
    single_statistic("meminfo", "","filepmdmapped", proc_data.timestamp, proc_data.meminfo.filepmdmapped, statistics).await;
    single_statistic("meminfo", "","cmatotal", proc_data.timestamp, proc_data.meminfo.cmatotal, statistics).await;
    single_statistic("meminfo", "","cmafree", proc_data.timestamp, proc_data.meminfo.cmafree, statistics).await;
    single_statistic("meminfo", "","hugepages_total", proc_data.timestamp, proc_data.meminfo.hugepages_total, statistics).await;
    single_statistic("meminfo", "","hugepages_free", proc_data.timestamp, proc_data.meminfo.hugepages_free, statistics).await;
    single_statistic("meminfo", "","hugepages_rsvd", proc_data.timestamp, proc_data.meminfo.hugepages_rsvd, statistics).await;
    single_statistic("meminfo", "","hugepages_surp", proc_data.timestamp, proc_data.meminfo.hugepages_surp, statistics).await;
    single_statistic("meminfo", "","hugepagesize", proc_data.timestamp, proc_data.meminfo.hugepagesize, statistics).await;
    single_statistic("meminfo", "","hugetlb", proc_data.timestamp, proc_data.meminfo.hugetlb, statistics).await;

     */
}

/*
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

pub async fn print_all_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str, print_header: bool)
{
    if print_header
    {
        match output
        {
            "sar-u" => {
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
                println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
    let scheduler_running = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_running".to_string())).unwrap().per_second_value/10_000_000_f64;
    let scheduler_waiting = statistics.get(&("schedstat".to_string(), "all".to_string(), "time_waiting".to_string())).unwrap().per_second_value/10_000_000_f64;
    match output
    {
        "sar-u" => {
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
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
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/total*100.,
                     nice/total*100.,
                     system/total*100.,
                     iowait/total*100.,
                     steal/total*100.,
                     irq/total*100.,
                     softirq/total*100.,
                     guest_user/total*100.,
                     guest_nice/total*100.,
                     idle/total*100.,
            );
        },
        "cpu-all" => {
            println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                     user/1000.,
                     nice/1000.,
                     system/1000.,
                     iowait/1000.,
                     steal/1000.,
                     irq/1000.,
                     softirq/1000.,
                     guest_user/1000.,
                     guest_nice/1000.,
                     idle/1000.,
                     scheduler_running/1000.,
                     scheduler_waiting/1000.,
            );
        },
        &_ => todo!{},
    }
}
pub async fn print_per_cpu(statistics: &HashMap<(String, String, String), Statistic>, output: &str)
{
    if !statistics.get(&("stat".to_string(), "all".to_string(), "user".to_string())).unwrap().updated_value { return };
    let mut cpu_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "stat" || group == "schedstat")
        .map(|(_, cpu_specification, _)| cpu_specification)
        .filter(|cpu_specification| cpu_specification.starts_with("cpu") || *cpu_specification == "all")
        .collect::<HashSet<&String>>()
        .into_iter()
        .collect();
    cpu_list.sort();
    match output
    {
        "mpstat-P-ALL" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
            "mpstat-P-ALL" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / total * 100.,
                         nice / total * 100.,
                         system / total * 100.,
                         iowait / total * 100.,
                         irq / total * 100.,
                         softirq / total * 100.,
                         steal / total * 100.,
                         guest_user / total * 100.,
                         guest_nice / total * 100.,
                         idle / total * 100.,
                );
            },
            "per-cpu-all" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         cpu_name,
                         user / 1000.,
                         nice / 1000.,
                         system / 1000.,
                         iowait / 1000.,
                         irq / 1000.,
                         softirq / 1000.,
                         steal / 1000.,
                         guest_user / 1000.,
                         guest_nice / 1000.,
                         idle / 1000.,
                         scheduler_running / 1000.,
                         scheduler_waiting / 1000.,
                );
            },
            &_ => todo! {},
        }
    }
}

 */