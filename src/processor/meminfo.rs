use crate::add_list_of_u64_data_to_statistics;
use crate::processor::{single_statistic_u64, ProcData, ProcessorError, Statistic};
use crate::HISTORY;
use anyhow::Result;
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::meminfo::ProcMemInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MemInfo {
    pub timestamp: DateTime<Local>,
    pub memfree: f64,
    pub memavailable: f64,
    pub memtotal: f64,
    pub buffers: f64,
    pub cached: f64,
    pub swapcached: f64,
    pub kernelstack: f64,
    pub hardwarecorrupted: f64,
    pub slab: f64,
    pub pagetables: f64,
    pub dirty: f64,
    pub shmem: f64,
    pub mapped: f64,
    pub anonpages: f64,
    pub hugepages_total: f64,
    pub hugepages_free: f64,
    pub hugepages_reserved: f64,
    pub hugepagesize: f64,
    pub swaptotal: f64,
    pub swapfree: f64,
    pub sunreclaim: f64,
    pub sreclaimable: f64,
    pub active_anon: f64,
    pub inactive_anon: f64,
    pub active_file: f64,
    pub inactive_file: f64,
    pub committed_as: f64,
    pub commitlimit: f64,
}

pub async fn read_meminfo_proc_data() -> Result<ProcMemInfo> {
    let proc_meminfo = proc_sys_parser::meminfo::read()?;
    debug!("{:?}", proc_meminfo);
    Ok(proc_meminfo)
}

pub async fn process_meminfo_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    add_list_of_u64_data_to_statistics!(
        meminfo,
        "",
        proc_data.timestamp,
        proc_data,
        meminfo,
        statistics,
        memtotal,
        memfree,
        memavailable,
        buffers,
        cached,
        swapcached,
        active,
        inactive,
        active_anon,
        inactive_anon,
        active_file,
        inactive_file,
        unevictable,
        mlocked,
        swaptotal,
        swapfree,
        zswap,
        zswapped,
        dirty,
        writeback,
        anonpages,
        mapped,
        shmem,
        kreclaimable,
        slab,
        sreclaimable,
        sunreclaim,
        kernelstack,
        shadowcallstack,
        pagetables,
        secpagetables,
        nfs_unstable,
        bounce,
        writebacktmp,
        commitlimit,
        committed_as,
        vmalloctotal,
        vmallocused,
        vmallocchunk,
        percpu,
        hardwarecorrupted,
        anonhugepages,
        shmemhugepages,
        shmempmdmapped,
        filehugepages,
        filepmdmapped,
        cmatotal,
        cmafree,
        hugepages_total,
        hugepages_free,
        hugepages_rsvd,
        hugepages_surp,
        hugepagesize,
        hugetlb
    );
    Ok(())
}

pub async fn print_meminfo(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool,
) -> Result<()> {
    if print_header {
        match output {
            "sar-r" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
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
            }
            "sar-r-ALL" => {
                println!("{:10}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
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
                         "mbanonpg",
                         "mbslab",
                         "mbstack",
                         "mbpgtbl",
                         "mbvmused",
                );
            }
            "sar-H" => {
                println!(
                    "{:10}    {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp", "mbhugfree", "mbhugused", "%hugused", "mbhugrsvd", "mbhugsurp",
                );
            }
            "sar-S" => {
                println!(
                    "{:10}    {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp", "mbswpfree", "mbswpused", "%swpused", "mbswpcad", "%swpcad",
                );
            }
            &_ => todo!(),
        }
    }

    let timestamp = statistics
        .get(&("meminfo".to_string(), "".to_string(), "memfree".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memfree".to_string(),
        })?
        .last_timestamp;
    let memfree = statistics
        .get(&("meminfo".to_string(), "".to_string(), "memfree".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memfree".to_string(),
        })?
        .last_value;
    let memavailable = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memavailable".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memavailable".to_string(),
        })?
        .last_value;
    let memtotal = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memtotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memtotal".to_string(),
        })?
        .last_value;
    let buffers = statistics
        .get(&("meminfo".to_string(), "".to_string(), "buffers".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "buffers".to_string(),
        })?
        .last_value;
    let cached = statistics
        .get(&("meminfo".to_string(), "".to_string(), "cached".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "cached".to_string(),
        })?
        .last_value;
    let committed_as = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "committed_as".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "committed_as".to_string(),
        })?
        .last_value;
    let commitlimit = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "commitlimit".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "commitlimit".to_string(),
        })?
        .last_value;
    let active = statistics
        .get(&("meminfo".to_string(), "".to_string(), "active".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "active".to_string(),
        })?
        .last_value;
    let inactive = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "inactive".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "inactive".to_string(),
        })?
        .last_value;
    let dirty = statistics
        .get(&("meminfo".to_string(), "".to_string(), "dirty".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "dirty".to_string(),
        })?
        .last_value;
    let anonpages = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "anonpages".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "anonpages".to_string(),
        })?
        .last_value;
    let slab = statistics
        .get(&("meminfo".to_string(), "".to_string(), "slab".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "slab".to_string(),
        })?
        .last_value;
    let kernelstack = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "kernelstack".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "kernelstack".to_string(),
        })?
        .last_value;
    let pagetables = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "pagetables".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "pagetables".to_string(),
        })?
        .last_value;
    let vmallocused = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "vmallocused".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "vmallocused".to_string(),
        })?
        .last_value;
    let hugepages_total = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_total".to_string(),
        })?
        .last_value;
    let hugepages_free = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_free".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_free".to_string(),
        })?
        .last_value;
    let hugepagesize = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepagesize".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepagesize".to_string(),
        })?
        .last_value;
    let hugepages_reserved = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_rsvd".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_rsvd".to_string(),
        })?
        .last_value;
    let hugepages_surplus = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_surp".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_surp".to_string(),
        })?
        .last_value;
    let swap_free = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swapfree".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swapfree".to_string(),
        })?
        .last_value;
    let swap_total = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swaptotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swaptotal".to_string(),
        })?
        .last_value;
    let swap_cached = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swapcached".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swapcached".to_string(),
        })?
        .last_value;
    // this is what sar defines as non-used memory; see: https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/pr_stats.c#L809
    let mut non_used_memory = memfree + buffers + cached + slab;
    if non_used_memory > memtotal {
        non_used_memory = memtotal
    };

    match output {
        // https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/pr_stats.c#L789
        "sar-r" => {
            println!("{:10}    {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                memfree / 1024_f64,
                memavailable / 1024_f64,
                (memtotal - non_used_memory) / 1024_f64,
                (memtotal - non_used_memory) / memtotal * 100_f64,
                buffers / 1024_f64,
                cached / 1024_f64,
                committed_as / 1024_f64,
                committed_as / commitlimit * 100_f64,
                active / 1024_f64,
                inactive / 1024_f64,
                dirty / 1024_f64,
            );
        }
        "sar-r-ALL" => {
            println!("{:10}    {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.2} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                memfree / 1024_f64,
                memavailable / 1024_f64,
                (memtotal - non_used_memory) / 1024_f64,
                (memtotal - non_used_memory) / memtotal * 100_f64,
                buffers / 1024_f64,
                cached / 1024_f64,
                committed_as / 1024_f64,
                committed_as / commitlimit * 100_f64,
                active / 1024_f64,
                inactive / 1024_f64,
                dirty / 1024_f64,
                anonpages / 1024_f64,
                slab / 1024_f64,
                kernelstack / 1024_f64,
                pagetables / 1024_f64,
                vmallocused / 1024_f64,
            );
        }
        "sar-H" => {
            println!(
                "{:10}    {:10.0} {:10.0} {:10.2} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                (hugepages_free * hugepagesize) / (1024_f64 * 1024_f64),
                ((hugepages_total - hugepages_free) * hugepagesize) / (1024_f64 * 1024_f64),
                if hugepages_total == 0_f64 {
                    0_f64
                } else {
                    (hugepages_total - hugepages_free) / hugepages_total * 100_f64
                },
                (hugepages_reserved * hugepagesize) / (1024_f64 * 1024_f64),
                (hugepages_surplus * hugepagesize) / (1024_f64 * 1024_f64),
            );
        }
        "sar-S" => {
            println!(
                "{:10}    {:10.0} {:10.0} {:10.2} {:10.0} {:10.0}",
                timestamp.format("%H:%M:%S"),
                swap_free / 1024_f64,
                (swap_total - swap_free) / 1024_f64,
                if swap_total == 0_f64 {
                    0_f64
                } else {
                    (swap_total - swap_free) / swap_total * 100_f64
                },
                swap_cached / 1024_f64,
                if swap_total - swap_free == 0_f64 {
                    0_f64
                } else {
                    swap_cached / (swap_total - swap_free) * 100_f64
                },
            );
        }
        &_ => todo!(),
    }
    Ok(())
}

pub async fn add_memory_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if !statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memtotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memtotal".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memtotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memtotal".to_string(),
        })?
        .last_timestamp;
    let memfree = statistics
        .get(&("meminfo".to_string(), "".to_string(), "memfree".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memtotal".to_string(),
        })?
        .last_value;
    let memavailable = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memavailable".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memavailable".to_string(),
        })?
        .last_value;
    let memtotal = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "memtotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memtotal".to_string(),
        })?
        .last_value;
    let buffers = statistics
        .get(&("meminfo".to_string(), "".to_string(), "buffers".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "buffers".to_string(),
        })?
        .last_value;
    let cached = statistics
        .get(&("meminfo".to_string(), "".to_string(), "cached".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "cached".to_string(),
        })?
        .last_value;
    let swapcached = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swapcached".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swapcached".to_string(),
        })?
        .last_value;
    let kernelstack = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "kernelstack".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "kernelstack".to_string(),
        })?
        .last_value;
    let hardwarecorrupted = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hardwarecorrupted".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hardwarecorrupted".to_string(),
        })?
        .last_value;
    let slab = statistics
        .get(&("meminfo".to_string(), "".to_string(), "slab".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "slab".to_string(),
        })?
        .last_value;
    let pagetables = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "pagetables".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "pagetables".to_string(),
        })?
        .last_value;
    let dirty = statistics
        .get(&("meminfo".to_string(), "".to_string(), "dirty".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "dirty".to_string(),
        })?
        .last_value;
    let shmem = statistics
        .get(&("meminfo".to_string(), "".to_string(), "shmem".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "shmem".to_string(),
        })?
        .last_value;
    let mapped = statistics
        .get(&("meminfo".to_string(), "".to_string(), "mapped".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "mapped".to_string(),
        })?
        .last_value;
    let anonpages = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "anonpages".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "anonpages".to_string(),
        })?
        .last_value;
    let hugepages_total = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_total".to_string(),
        })?
        .last_value;
    let hugepages_free = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_free".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_free".to_string(),
        })?
        .last_value;
    let hugepages_reserved = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepages_rsvd".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepages_rsvd".to_string(),
        })?
        .last_value;
    let hugepagesize = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "hugepagesize".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "hugepagesize".to_string(),
        })?
        .last_value;
    let swaptotal = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swaptotal".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swaptotal".to_string(),
        })?
        .last_value;
    let swapfree = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "swapfree".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "swapfree".to_string(),
        })?
        .last_value;
    let sunreclaim = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "sunreclaim".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "sunreclaim".to_string(),
        })?
        .last_value;
    let sreclaimable = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "sreclaimable".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "sreclaimable".to_string(),
        })?
        .last_value;
    let active_anon = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "active_anon".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "active_anon".to_string(),
        })?
        .last_value;
    let inactive_anon = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "inactive_anon".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "inactive_anon".to_string(),
        })?
        .last_value;
    let active_file = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "active_file".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "active_file".to_string(),
        })?
        .last_value;
    let inactive_file = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "inactive_file".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "inactive_file".to_string(),
        })?
        .last_value;
    let committed_as = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "commited_as".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "committed_as".to_string(),
        })?
        .last_value;
    let commitlimit = statistics
        .get(&(
            "meminfo".to_string(),
            "".to_string(),
            "commitlimit".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "commitlimit".to_string(),
        })?
        .last_value;
    HISTORY.memory.write().unwrap().push_back(MemInfo {
        timestamp,
        memfree,
        memavailable,
        memtotal,
        buffers,
        cached,
        swapcached,
        kernelstack,
        hardwarecorrupted,
        slab,
        pagetables,
        dirty,
        shmem,
        mapped,
        anonpages,
        hugepages_total,
        hugepages_free,
        hugepages_reserved,
        hugepagesize,
        swaptotal,
        swapfree,
        sunreclaim,
        sreclaimable,
        active_anon,
        inactive_anon,
        active_file,
        inactive_file,
        committed_as,
        commitlimit,
    });

    Ok(())
}
