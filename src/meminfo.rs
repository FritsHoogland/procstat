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
}