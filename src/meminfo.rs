use std::collections::HashMap;
use crate::common::{ProcData, single_statistic, Statistic};

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
}

pub async fn print_meminfo(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool
)
{
    if print_header
    {
        match output
        {
            "sar-r" => {
                println!("{:10}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
                         "Timestamp",
                         "mbmemfree",
                         "mbavail",
                         "mbmemused",
                         "%memused",
                         "mbbuffers",
                         "mbcached",
                         "mbcommit",
                         "%commit",
                         "mbactive",
                         "mbinact",
                         "mbdirty",
                );
            },
            &_ => todo!(),
        }
    }

    let timestamp = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_timestamp;
    let memfree = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_value;
    let memavailable = statistics.get(&("meminfo".to_string(), "".to_string(), "memavailable".to_string())).unwrap().last_value;
    let memtotal = statistics.get(&("meminfo".to_string(), "".to_string(), "memtotal".to_string())).unwrap().last_value;
    let buffers = statistics.get(&("meminfo".to_string(), "".to_string(), "buffers".to_string())).unwrap().last_value;
    let cached = statistics.get(&("meminfo".to_string(), "".to_string(), "cached".to_string())).unwrap().last_value;
    let committed_as = statistics.get(&("meminfo".to_string(), "".to_string(), "committed_as".to_string())).unwrap().last_value;
    let swaptotal = statistics.get(&("meminfo".to_string(), "".to_string(), "swaptotal".to_string())).unwrap().last_value;
    let active = statistics.get(&("meminfo".to_string(), "".to_string(), "active".to_string())).unwrap().last_value;
    let inactive = statistics.get(&("meminfo".to_string(), "".to_string(), "inactive".to_string())).unwrap().last_value;
    let dirty = statistics.get(&("meminfo".to_string(), "".to_string(), "dirty".to_string())).unwrap().last_value;

    match output
    {
        "sar-r" => {
            println!("{:10}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                timestamp.format("%H:%M:%S"),
                memfree / (1024_f64 * 1024_f64),
                memavailable / (1024_f64 * 1024_f64),
                (memtotal - memfree) / (1024_f64 * 1024_f64),
                (memtotal - memfree) / memtotal * 100_f64,
                buffers / (1024_f64 * 1024_f64),
                cached / (1024_f64 * 1024_f64),
                committed_as / (1024_f64 * 1024_f64),
                committed_as / (memtotal + swaptotal) * 100_f64,
                active / (1024_f64 * 1024_f64),
                inactive / (1024_f64 * 1024_f64),
                dirty / (1024_f64 * 1024_f64),
            );
        },
        &_ => todo!(),
    }
}