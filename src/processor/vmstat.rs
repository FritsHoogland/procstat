use crate::processor::{
    single_statistic_option_u64, single_statistic_u64, ProcData, ProcessorError, Statistic,
};
use crate::Data;
use crate::ARGS;
use crate::DATA;
use crate::{add_list_of_option_u64_data_to_statistics, add_list_of_u64_data_to_statistics};
use anyhow::Result;
use chrono::{DateTime, Local};
use log::debug;
use proc_sys_parser::vmstat::ProcVmStat;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VmStatInfo {
    pub timestamp: DateTime<Local>,
    pub nr_free_pages: f64,
    pub nr_zone_inactive_anon: f64,
    pub nr_zone_active_anon: f64,
    pub nr_zone_inactive_file: f64,
    pub nr_zone_active_file: f64,
    pub nr_zone_unevictable: f64,
    pub nr_zone_write_pending: f64,
    pub nr_mlock: f64,
    pub nr_bounce: f64,
    pub nr_zspages: f64,
    pub nr_free_cma: f64,
    pub numa_hit: f64,
    pub numa_miss: f64,
    pub numa_foreign: f64,
    pub numa_interleave: f64,
    pub numa_local: f64,
    pub numa_other: f64,
    pub nr_inactive_anon: f64,
    pub nr_active_anon: f64,
    pub nr_inactive_file: f64,
    pub nr_active_file: f64,
    pub nr_unevictable: f64,
    pub nr_slab_reclaimable: f64,
    pub nr_slab_unreclaimable: f64,
    pub nr_isolated_anon: f64,
    pub nr_isolated_file: f64,
    pub workingset_nodes: f64,
    pub workingset_refault_anon: f64,
    pub workingset_refault_file: f64,
    pub workingset_activate_anon: f64,
    pub workingset_activate_file: f64,
    pub workingset_restore_anon: f64,
    pub workingset_restore_file: f64,
    pub workingset_nodereclaim: f64,
    pub nr_anon_pages: f64,
    pub nr_mapped: f64,
    pub nr_file_pages: f64,
    pub nr_dirty: f64,
    pub nr_writeback: f64,
    pub nr_writeback_temp: f64,
    pub nr_shmem: f64,
    pub nr_shmem_hugepages: f64,
    pub nr_shmem_pmdmapped: f64,
    pub nr_file_hugepages: f64,
    pub nr_file_pmdmapped: f64,
    pub nr_anon_transparent_hugepages: f64,
    pub nr_vmscan_write: f64,
    pub nr_vmscan_immediate_reclaim: f64,
    pub nr_dirtied: f64,
    pub nr_written: f64,
    pub nr_throttled_written: f64,
    pub nr_kernel_misc_reclaimable: f64,
    pub nr_foll_pin_acquired: f64,
    pub nr_foll_pin_released: f64,
    pub nr_kernel_stack: f64,
    pub nr_shadow_call_stack: f64,
    pub nr_page_table_pages: f64,
    pub nr_sec_page_table_pages: f64,
    pub nr_swapcached: f64,
    pub pgpromote_success: f64,
    pub pgpromote_candidate: f64,
    pub nr_dirty_threshold: f64,
    pub nr_dirty_background_threshold: f64,
    pub pgpgin: f64,
    pub pgpgout: f64,
    pub pswpin: f64,
    pub pswpout: f64,
    pub pgalloc_dma: f64,
    pub pgalloc_dma32: f64,
    pub pgalloc_normal: f64,
    pub pgalloc_movable: f64,
    pub pgalloc_device: f64,
    pub allocstall_dma: f64,
    pub allocstall_dma32: f64,
    pub allocstall_normal: f64,
    pub allocstall_movable: f64,
    pub allocstall_device: f64,
    pub pgskip_dma: f64,
    pub pgskip_dma32: f64,
    pub pgskip_normal: f64,
    pub pgskip_movable: f64,
    pub pgskip_device: f64,
    pub pgfree: f64,
    pub pgactivate: f64,
    pub pgdeactivate: f64,
    pub pglazyfree: f64,
    pub pglazyfreed: f64,
    pub pgfault: f64,
    pub pgmajfault: f64,
    pub pgrefill: f64,
    pub pgreuse: f64,
    pub pgsteal_kswapd: f64,
    pub pgsteal_direct: f64,
    pub pgsteal_khugepaged: f64,
    pub pgdemote_kswapd: f64,
    pub pgdemote_direct: f64,
    pub pgdemote_khugepaged: f64,
    pub pgscan_kswapd: f64,
    pub pgscan_direct: f64,
    pub pgscan_khugepaged: f64,
    pub pgscan_direct_throttle: f64,
    pub pgscan_anon: f64,
    pub pgscan_file: f64,
    pub pgsteal_anon: f64,
    pub pgsteal_file: f64,
    pub zone_reclaim_failed: f64,
    pub pginodesteal: f64,
    pub slabs_scanned: f64,
    pub kswapd_inodesteal: f64,
    pub kswapd_low_wmark_hit_quickly: f64,
    pub kswapd_high_wmark_hit_quickly: f64,
    pub pageoutrun: f64,
    pub pgrotated: f64,
    pub drop_pagecache: f64,
    pub drop_slab: f64,
    pub oom_kill: f64,
    pub numa_pte_updates: f64,
    pub numa_huge_pte_updates: f64,
    pub numa_hint_faults: f64,
    pub numa_hint_faults_local: f64,
    pub numa_pages_migrated: f64,
    pub pgmigrate_success: f64,
    pub pgmigrate_fail: f64,
    pub thp_migration_success: f64,
    pub thp_migration_fail: f64,
    pub thp_migration_split: f64,
    pub compact_migrate_scanned: f64,
    pub compact_free_scanned: f64,
    pub compact_isolated: f64,
    pub compact_stall: f64,
    pub compact_fail: f64,
    pub compact_success: f64,
    pub compact_daemon_wake: f64,
    pub compact_daemon_migrate_scanned: f64,
    pub compact_daemon_free_scanned: f64,
    pub htlb_buddy_alloc_success: f64,
    pub htlb_buddy_alloc_fail: f64,
    pub cma_alloc_success: f64,
    pub cma_alloc_fail: f64,
    pub unevictable_pgs_culled: f64,
    pub unevictable_pgs_scanned: f64,
    pub unevictable_pgs_rescued: f64,
    pub unevictable_pgs_mlocked: f64,
    pub unevictable_pgs_munlocked: f64,
    pub unevictable_pgs_cleared: f64,
    pub unevictable_pgs_stranded: f64,
    pub thp_fault_alloc: f64,
    pub thp_fault_fallback: f64,
    pub thp_fault_fallback_charge: f64,
    pub thp_collapse_alloc: f64,
    pub thp_collapse_alloc_failed: f64,
    pub thp_file_alloc: f64,
    pub thp_file_fallback: f64,
    pub thp_file_fallback_charge: f64,
    pub thp_file_mapped: f64,
    pub thp_split_page: f64,
    pub thp_split_page_failed: f64,
    pub thp_deferred_split_page: f64,
    pub thp_split_pmd: f64,
    pub thp_scan_exceed_none_pte: f64,
    pub thp_scan_exceed_swap_pte: f64,
    pub thp_scan_exceed_share_pte: f64,
    pub thp_zero_page_alloc: f64,
    pub thp_zero_page_alloc_failed: f64,
    pub thp_swpout: f64,
    pub thp_swpout_fallback: f64,
    pub balloon_inflate: f64,
    pub balloon_deflate: f64,
    pub balloon_migrate: f64,
    pub swap_ra: f64,
    pub swap_ra_hit: f64,
    pub ksm_swpin_copy: f64,
    pub cow_ksm: f64,
    pub zswpin: f64,
    pub zswpout: f64,
    pub nr_unstable: f64,
    pub pgfault_delta: f64,
    pub pgmajfault_delta: f64,
}

pub async fn read_vmstat_proc_data() -> Result<ProcVmStat> {
    let proc_vmstat = proc_sys_parser::vmstat::read()?;
    debug!("{:?}", proc_vmstat);
    Ok(proc_vmstat)
}

pub async fn process_vmstat_data(
    proc_data: &ProcData,
    statistics: &mut HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    add_list_of_u64_data_to_statistics!(
        vmstat,
        "",
        proc_data.timestamp,
        proc_data,
        vmstat,
        statistics,
        nr_free_pages,
        nr_zone_inactive_anon,
        nr_zone_active_anon,
        nr_zone_inactive_file,
        nr_zone_active_file,
        nr_zone_unevictable,
        nr_zone_write_pending,
        nr_mlock,
        nr_bounce,
        nr_zspages,
        nr_free_cma,
        numa_hit,
        numa_miss,
        numa_foreign,
        numa_interleave,
        numa_local,
        numa_other,
        nr_inactive_anon,
        nr_active_anon,
        nr_active_file,
        nr_inactive_file,
        nr_unevictable,
        nr_slab_reclaimable,
        nr_slab_unreclaimable,
        nr_isolated_anon,
        nr_isolated_file,
        workingset_nodereclaim,
        nr_anon_pages,
        nr_mapped,
        nr_file_pages,
        nr_dirty,
        nr_writeback,
        nr_writeback_temp,
        nr_shmem,
        nr_shmem_hugepages,
        nr_shmem_pmdmapped,
        nr_anon_transparent_hugepages,
        nr_vmscan_write,
        nr_vmscan_immediate_reclaim,
        nr_dirtied,
        nr_written,
        nr_kernel_stack,
        nr_page_table_pages,
        nr_dirty_threshold,
        nr_dirty_background_threshold,
        pgpgin,
        pgpgout,
        pswpin,
        pswpout,
        pgalloc_dma,
        pgalloc_dma32,
        pgalloc_normal,
        pgalloc_movable,
        allocstall_dma,
        allocstall_dma32,
        allocstall_normal,
        allocstall_movable,
        pgskip_dma,
        pgskip_dma32,
        pgskip_normal,
        pgskip_movable,
        pgfree,
        pgactivate,
        pgdeactivate,
        pglazyfree,
        pglazyfreed,
        pgrefill,
        pgfault,
        pgmajfault,
        pgsteal_kswapd,
        pgsteal_direct,
        pgscan_kswapd,
        pgscan_direct,
        pgscan_direct_throttle,
        zone_reclaim_failed,
        pginodesteal,
        kswapd_inodesteal,
        kswapd_low_wmark_hit_quickly,
        kswapd_high_wmark_hit_quickly,
        pageoutrun,
        pgrotated,
        drop_pagecache,
        drop_slab,
        oom_kill,
        numa_pte_updates,
        numa_huge_pte_updates,
        numa_hint_faults,
        numa_hint_faults_local,
        numa_pages_migrated,
        pgmigrate_success,
        pgmigrate_fail,
        compact_migrate_scanned,
        compact_free_scanned,
        compact_isolated,
        compact_stall,
        compact_fail,
        compact_success,
        compact_daemon_wake,
        compact_daemon_migrate_scanned,
        compact_daemon_free_scanned,
        htlb_buddy_alloc_success,
        htlb_buddy_alloc_fail,
        unevictable_pgs_culled,
        unevictable_pgs_scanned,
        unevictable_pgs_rescued,
        unevictable_pgs_mlocked,
        unevictable_pgs_munlocked,
        unevictable_pgs_cleared,
        unevictable_pgs_stranded,
        thp_fault_alloc,
        thp_fault_fallback,
        thp_collapse_alloc,
        thp_collapse_alloc_failed,
        thp_file_alloc,
        thp_file_mapped,
        thp_split_page,
        thp_split_page_failed,
        thp_deferred_split_page,
        thp_split_pmd,
        thp_zero_page_alloc,
        thp_zero_page_alloc_failed,
        thp_swpout,
        thp_swpout_fallback,
        balloon_inflate,
        balloon_deflate,
        balloon_migrate,
        swap_ra,
        swap_ra_hit,
        nr_unstable
    );
    add_list_of_option_u64_data_to_statistics!(
        vmstat,
        "",
        proc_data.timestamp,
        proc_data,
        vmstat,
        statistics,
        workingset_nodes,
        workingset_restore_anon,
        workingset_refault_file,
        workingset_activate_anon,
        workingset_activate_file,
        workingset_restore_anon,
        workingset_restore_file,
        nr_file_hugepages,
        nr_file_pmdmapped,
        nr_throttled_written,
        nr_kernel_misc_reclaimable,
        nr_foll_pin_acquired,
        nr_foll_pin_released,
        nr_shadow_call_stack,
        nr_sec_page_table_pages,
        nr_swapcached,
        pgpromote_success,
        pgpromote_candidate,
        pgalloc_device,
        pgskip_device,
        pgreuse,
        pgsteal_khugepaged,
        pgdemote_kswapd,
        pgdemote_direct,
        pgdemote_khugepaged,
        pgscan_khugepaged,
        pgscan_anon,
        pgscan_file,
        pgsteal_anon,
        pgsteal_file,
        slabs_scanned,
        thp_migration_success,
        thp_migration_fail,
        thp_migration_split,
        cma_alloc_success,
        cma_alloc_fail,
        thp_fault_fallback_charge,
        thp_file_fallback,
        thp_file_fallback_charge,
        thp_scan_exceed_none_pte,
        thp_scan_exceed_swap_pte,
        thp_scan_exceed_share_pte,
        ksm_swpin_copy,
        cow_ksm,
        zswpin,
        zswpout
    );
    Ok(())
}

pub async fn add_vmstat_to_history(
    statistics: &HashMap<(String, String, String), Statistic>,
) -> Result<()> {
    if !statistics
        .get(&(
            "vmstat".to_string(),
            "".to_string(),
            "nr_free_pages".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "vmstat".to_string(),
            key2: "".to_string(),
            key3: "nr_free_pages".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&(
            "vmstat".to_string(),
            "".to_string(),
            "nr_free_pages".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "vmstat".to_string(),
            key2: "".to_string(),
            key3: "nr_free_pages".to_string(),
        })?
        .last_timestamp;

    macro_rules! generate_assignments_for_history_addition_per_second_value {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string())).unwrap_or(&Statistic::default()).per_second_value;
            )*
        };
    }
    generate_assignments_for_history_addition_per_second_value!(
        nr_free_pages,
        nr_zone_inactive_anon,
        nr_zone_active_anon,
        nr_zone_inactive_file,
        nr_zone_active_file,
        nr_zone_unevictable,
        nr_zone_write_pending,
        nr_mlock,
        nr_bounce,
        nr_zspages,
        nr_free_cma,
        numa_hit,
        numa_miss,
        numa_foreign,
        numa_interleave,
        numa_local,
        numa_other,
        nr_inactive_anon,
        nr_active_anon,
        nr_active_file,
        nr_inactive_file,
        nr_unevictable,
        nr_slab_reclaimable,
        nr_slab_unreclaimable,
        nr_isolated_anon,
        nr_isolated_file,
        workingset_nodes,
        workingset_refault_anon,
        workingset_refault_file,
        workingset_activate_anon,
        workingset_activate_file,
        workingset_restore_anon,
        workingset_restore_file,
        workingset_nodereclaim,
        nr_anon_pages,
        nr_mapped,
        nr_file_pages,
        nr_dirty,
        nr_writeback,
        nr_writeback_temp,
        nr_shmem,
        nr_shmem_hugepages,
        nr_shmem_pmdmapped,
        nr_file_hugepages,
        nr_file_pmdmapped,
        nr_anon_transparent_hugepages,
        nr_vmscan_write,
        nr_vmscan_immediate_reclaim,
        nr_dirtied,
        nr_written,
        nr_throttled_written,
        nr_kernel_misc_reclaimable,
        nr_foll_pin_acquired,
        nr_foll_pin_released,
        nr_kernel_stack,
        nr_shadow_call_stack,
        nr_page_table_pages,
        nr_sec_page_table_pages,
        nr_swapcached,
        pgpromote_success,
        pgpromote_candidate,
        nr_dirty_threshold,
        nr_dirty_background_threshold,
        pgpgin,
        pgpgout,
        pswpin,
        pswpout,
        allocstall_dma,
        allocstall_dma32,
        allocstall_normal,
        allocstall_movable,
        allocstall_device,
        pgskip_dma,
        pgskip_dma32,
        pgskip_normal,
        pgskip_movable,
        pgskip_device,
        pgactivate,
        pgdeactivate,
        pglazyfree,
        pglazyfreed,
        pgrefill,
        pgfault,
        pgmajfault,
        pgreuse,
        pgdemote_kswapd,
        pgdemote_direct,
        pgdemote_khugepaged,
        pgscan_direct_throttle,
        pgscan_anon,
        pgscan_file,
        pgsteal_anon,
        pgsteal_file,
        zone_reclaim_failed,
        pginodesteal,
        slabs_scanned,
        kswapd_inodesteal,
        kswapd_low_wmark_hit_quickly,
        kswapd_high_wmark_hit_quickly,
        pageoutrun,
        pgrotated,
        drop_pagecache,
        drop_slab,
        numa_pte_updates,
        numa_huge_pte_updates,
        numa_hint_faults,
        numa_hint_faults_local,
        numa_pages_migrated,
        pgmigrate_success,
        pgmigrate_fail,
        thp_migration_success,
        thp_migration_fail,
        thp_migration_split,
        compact_migrate_scanned,
        compact_free_scanned,
        compact_isolated,
        compact_stall,
        compact_fail,
        compact_success,
        compact_daemon_wake,
        compact_daemon_migrate_scanned,
        compact_daemon_free_scanned,
        htlb_buddy_alloc_success,
        htlb_buddy_alloc_fail,
        cma_alloc_success,
        cma_alloc_fail,
        unevictable_pgs_culled,
        unevictable_pgs_scanned,
        unevictable_pgs_rescued,
        unevictable_pgs_mlocked,
        unevictable_pgs_munlocked,
        unevictable_pgs_cleared,
        unevictable_pgs_stranded,
        thp_fault_alloc,
        thp_fault_fallback,
        thp_fault_fallback_charge,
        thp_collapse_alloc,
        thp_collapse_alloc_failed,
        thp_file_alloc,
        thp_file_fallback,
        thp_file_fallback_charge,
        thp_file_mapped,
        thp_split_page,
        thp_split_page_failed,
        thp_deferred_split_page,
        thp_split_pmd,
        thp_scan_exceed_none_pte,
        thp_scan_exceed_swap_pte,
        thp_scan_exceed_share_pte,
        thp_zero_page_alloc,
        thp_zero_page_alloc_failed,
        thp_swpout,
        thp_swpout_fallback,
        balloon_inflate,
        balloon_deflate,
        balloon_migrate,
        swap_ra,
        swap_ra_hit,
        ksm_swpin_copy,
        cow_ksm,
        zswpin,
        zswpout,
        nr_unstable
    );
    macro_rules! generate_assignments_for_history_addition_delta_value {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string())).unwrap_or(&Statistic::default()).delta_value;
            )*
        };
    }

    // the reason for the pgfault and pgmajfault statistics to have a separate delta statistic is
    // that the per second value is used for sar-B.
    let pgfault_delta = statistics
        .get(&("vmstat".to_string(), "".to_string(), "pgfault".to_string()))
        .unwrap_or(&Statistic::default())
        .delta_value;
    let pgmajfault_delta = statistics
        .get(&(
            "vmstat".to_string(),
            "".to_string(),
            "pgmajfault".to_string(),
        ))
        .unwrap_or(&Statistic::default())
        .delta_value;

    generate_assignments_for_history_addition_delta_value!(
        oom_kill,
        pgfree,
        pgalloc_dma,
        pgalloc_dma32,
        pgalloc_normal,
        pgalloc_device,
        pgalloc_movable,
        pgscan_kswapd,
        pgscan_direct,
        pgscan_khugepaged,
        pgsteal_khugepaged,
        pgsteal_kswapd,
        pgsteal_direct
    );
    Data::push_vmstat(VmStatInfo {
        timestamp,
        nr_free_pages,
        nr_zone_inactive_anon,
        nr_zone_active_anon,
        nr_zone_inactive_file,
        nr_zone_active_file,
        nr_zone_unevictable,
        nr_zone_write_pending,
        nr_mlock,
        nr_bounce,
        nr_zspages,
        nr_free_cma,
        numa_hit,
        numa_miss,
        numa_foreign,
        numa_interleave,
        numa_local,
        numa_other,
        nr_inactive_anon,
        nr_active_anon,
        nr_inactive_file,
        nr_active_file,
        nr_unevictable,
        nr_slab_reclaimable,
        nr_slab_unreclaimable,
        nr_isolated_anon,
        nr_isolated_file,
        workingset_nodes,
        workingset_refault_anon,
        workingset_refault_file,
        workingset_activate_anon,
        workingset_activate_file,
        workingset_restore_anon,
        workingset_restore_file,
        workingset_nodereclaim,
        nr_anon_pages,
        nr_mapped,
        nr_file_pages,
        nr_dirty,
        nr_writeback,
        nr_writeback_temp,
        nr_shmem,
        nr_shmem_hugepages,
        nr_shmem_pmdmapped,
        nr_file_hugepages,
        nr_file_pmdmapped,
        nr_anon_transparent_hugepages,
        nr_vmscan_write,
        nr_vmscan_immediate_reclaim,
        nr_dirtied,
        nr_written,
        nr_throttled_written,
        nr_kernel_misc_reclaimable,
        nr_foll_pin_acquired,
        nr_foll_pin_released,
        nr_kernel_stack,
        nr_shadow_call_stack,
        nr_page_table_pages,
        nr_sec_page_table_pages,
        nr_swapcached,
        pgpromote_success,
        pgpromote_candidate,
        nr_dirty_threshold,
        nr_dirty_background_threshold,
        pgpgin,
        pgpgout,
        pswpin,
        pswpout,
        pgalloc_dma,
        pgalloc_dma32,
        pgalloc_normal,
        pgalloc_movable,
        pgalloc_device,
        allocstall_dma,
        allocstall_dma32,
        allocstall_normal,
        allocstall_movable,
        allocstall_device,
        pgskip_dma,
        pgskip_dma32,
        pgskip_normal,
        pgskip_movable,
        pgskip_device,
        pgfree,
        pgactivate,
        pgdeactivate,
        pglazyfree,
        pglazyfreed,
        pgfault,
        pgmajfault,
        pgrefill,
        pgreuse,
        pgsteal_kswapd,
        pgsteal_direct,
        pgsteal_khugepaged,
        pgdemote_kswapd,
        pgdemote_direct,
        pgdemote_khugepaged,
        pgscan_kswapd,
        pgscan_direct,
        pgscan_khugepaged,
        pgscan_direct_throttle,
        pgscan_anon,
        pgscan_file,
        pgsteal_anon,
        pgsteal_file,
        zone_reclaim_failed,
        pginodesteal,
        slabs_scanned,
        kswapd_inodesteal,
        kswapd_low_wmark_hit_quickly,
        kswapd_high_wmark_hit_quickly,
        pageoutrun,
        pgrotated,
        drop_pagecache,
        drop_slab,
        oom_kill,
        numa_pte_updates,
        numa_huge_pte_updates,
        numa_hint_faults,
        numa_hint_faults_local,
        numa_pages_migrated,
        pgmigrate_success,
        pgmigrate_fail,
        thp_migration_success,
        thp_migration_fail,
        thp_migration_split,
        compact_migrate_scanned,
        compact_free_scanned,
        compact_isolated,
        compact_stall,
        compact_fail,
        compact_success,
        compact_daemon_wake,
        compact_daemon_migrate_scanned,
        compact_daemon_free_scanned,
        htlb_buddy_alloc_success,
        htlb_buddy_alloc_fail,
        cma_alloc_success,
        cma_alloc_fail,
        unevictable_pgs_culled,
        unevictable_pgs_scanned,
        unevictable_pgs_rescued,
        unevictable_pgs_mlocked,
        unevictable_pgs_munlocked,
        unevictable_pgs_cleared,
        unevictable_pgs_stranded,
        thp_fault_alloc,
        thp_fault_fallback,
        thp_fault_fallback_charge,
        thp_collapse_alloc,
        thp_collapse_alloc_failed,
        thp_file_alloc,
        thp_file_fallback,
        thp_file_fallback_charge,
        thp_file_mapped,
        thp_split_page,
        thp_split_page_failed,
        thp_deferred_split_page,
        thp_split_pmd,
        thp_scan_exceed_none_pte,
        thp_scan_exceed_swap_pte,
        thp_scan_exceed_share_pte,
        thp_zero_page_alloc,
        thp_zero_page_alloc_failed,
        thp_swpout,
        thp_swpout_fallback,
        balloon_inflate,
        balloon_deflate,
        balloon_migrate,
        swap_ra,
        swap_ra_hit,
        ksm_swpin_copy,
        cow_ksm,
        zswpin,
        zswpout,
        nr_unstable,
        pgfault_delta,
        pgmajfault_delta,
    })
    .await;
    /*
        DATA.vmstat.write().unwrap().push_back(VmStatInfo {
            timestamp,
            nr_free_pages,
            nr_zone_inactive_anon,
            nr_zone_active_anon,
            nr_zone_inactive_file,
            nr_zone_active_file,
            nr_zone_unevictable,
            nr_zone_write_pending,
            nr_mlock,
            nr_bounce,
            nr_zspages,
            nr_free_cma,
            numa_hit,
            numa_miss,
            numa_foreign,
            numa_interleave,
            numa_local,
            numa_other,
            nr_inactive_anon,
            nr_active_anon,
            nr_inactive_file,
            nr_active_file,
            nr_unevictable,
            nr_slab_reclaimable,
            nr_slab_unreclaimable,
            nr_isolated_anon,
            nr_isolated_file,
            workingset_nodes,
            workingset_refault_anon,
            workingset_refault_file,
            workingset_activate_anon,
            workingset_activate_file,
            workingset_restore_anon,
            workingset_restore_file,
            workingset_nodereclaim,
            nr_anon_pages,
            nr_mapped,
            nr_file_pages,
            nr_dirty,
            nr_writeback,
            nr_writeback_temp,
            nr_shmem,
            nr_shmem_hugepages,
            nr_shmem_pmdmapped,
            nr_file_hugepages,
            nr_file_pmdmapped,
            nr_anon_transparent_hugepages,
            nr_vmscan_write,
            nr_vmscan_immediate_reclaim,
            nr_dirtied,
            nr_written,
            nr_throttled_written,
            nr_kernel_misc_reclaimable,
            nr_foll_pin_acquired,
            nr_foll_pin_released,
            nr_kernel_stack,
            nr_shadow_call_stack,
            nr_page_table_pages,
            nr_sec_page_table_pages,
            nr_swapcached,
            pgpromote_success,
            pgpromote_candidate,
            nr_dirty_threshold,
            nr_dirty_background_threshold,
            pgpgin,
            pgpgout,
            pswpin,
            pswpout,
            pgalloc_dma,
            pgalloc_dma32,
            pgalloc_normal,
            pgalloc_movable,
            pgalloc_device,
            allocstall_dma,
            allocstall_dma32,
            allocstall_normal,
            allocstall_movable,
            allocstall_device,
            pgskip_dma,
            pgskip_dma32,
            pgskip_normal,
            pgskip_movable,
            pgskip_device,
            pgfree,
            pgactivate,
            pgdeactivate,
            pglazyfree,
            pglazyfreed,
            pgfault,
            pgmajfault,
            pgrefill,
            pgreuse,
            pgsteal_kswapd,
            pgsteal_direct,
            pgsteal_khugepaged,
            pgdemote_kswapd,
            pgdemote_direct,
            pgdemote_khugepaged,
            pgscan_kswapd,
            pgscan_direct,
            pgscan_khugepaged,
            pgscan_direct_throttle,
            pgscan_anon,
            pgscan_file,
            pgsteal_anon,
            pgsteal_file,
            zone_reclaim_failed,
            pginodesteal,
            slabs_scanned,
            kswapd_inodesteal,
            kswapd_low_wmark_hit_quickly,
            kswapd_high_wmark_hit_quickly,
            pageoutrun,
            pgrotated,
            drop_pagecache,
            drop_slab,
            oom_kill,
            numa_pte_updates,
            numa_huge_pte_updates,
            numa_hint_faults,
            numa_hint_faults_local,
            numa_pages_migrated,
            pgmigrate_success,
            pgmigrate_fail,
            thp_migration_success,
            thp_migration_fail,
            thp_migration_split,
            compact_migrate_scanned,
            compact_free_scanned,
            compact_isolated,
            compact_stall,
            compact_fail,
            compact_success,
            compact_daemon_wake,
            compact_daemon_migrate_scanned,
            compact_daemon_free_scanned,
            htlb_buddy_alloc_success,
            htlb_buddy_alloc_fail,
            cma_alloc_success,
            cma_alloc_fail,
            unevictable_pgs_culled,
            unevictable_pgs_scanned,
            unevictable_pgs_rescued,
            unevictable_pgs_mlocked,
            unevictable_pgs_munlocked,
            unevictable_pgs_cleared,
            unevictable_pgs_stranded,
            thp_fault_alloc,
            thp_fault_fallback,
            thp_fault_fallback_charge,
            thp_collapse_alloc,
            thp_collapse_alloc_failed,
            thp_file_alloc,
            thp_file_fallback,
            thp_file_fallback_charge,
            thp_file_mapped,
            thp_split_page,
            thp_split_page_failed,
            thp_deferred_split_page,
            thp_split_pmd,
            thp_scan_exceed_none_pte,
            thp_scan_exceed_swap_pte,
            thp_scan_exceed_share_pte,
            thp_zero_page_alloc,
            thp_zero_page_alloc_failed,
            thp_swpout,
            thp_swpout_fallback,
            balloon_inflate,
            balloon_deflate,
            balloon_migrate,
            swap_ra,
            swap_ra_hit,
            ksm_swpin_copy,
            cow_ksm,
            zswpin,
            zswpout,
            nr_unstable,
            pgfault_delta,
            pgmajfault_delta,
        });
    */
    debug!("{:?}", DATA.vmstat.read().unwrap());
    Ok(())
}

impl Data {
    pub async fn push_vmstat(vmstat: VmStatInfo) {
        while DATA.vmstat.read().unwrap().len() >= ARGS.history {
            DATA.vmstat.write().unwrap().pop_front();
        }
        DATA.vmstat.write().unwrap().push_back(vmstat);
    }
}

pub async fn print_vmstat(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
    print_header: bool,
) -> Result<()> {
    if print_header {
        match output {
            //https://github.com/sysstat/sysstat/blob/499f5b153e9707892bb8841d37e6ed3a0aa617e2/rd_stats.c#L737
            "sar-B" => {
                println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                         "Timestamp",
                         "cpu",
                         "pgpgin/s",
                         "pgpgout/s",
                         "fault/s",
                         "majfault/s",
                         "pgfree/s",
                         "pgscank/s",
                         "pgscand/s",
                         "pgsteal/s",
                         "pgprom/s",
                         "pgdem/s",
                );
            }
            "sar-W" => {
                println!(
                    "{:10} {:7}    {:>10} {:>10}",
                    "Timestamp", "", "pswpin/s", "pswpout/s",
                );
            }
            "vmstat" => {
                println!(
                    "{:10} {:9} {:35} {:17} {:17} {:17} {:25}",
                    "",
                    "--procs--",
                    "------------memory (mb)------------",
                    "-------swap------",
                    "-----io (mb)-----",
                    "------system-----",
                    "--------------cpu------------",
                );
                println!("{:10} {:>4} {:>4} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8} {:>4} {:>4} {:>4} {:>4} {:>4} {:>4}",
                    "Timestamp",
                    "r",
                    "b",
                    "swpd",
                    "free",
                    "buff",
                    "cache",
                    "si",
                    "so",
                    "bi",
                    "bo",
                    "in",
                    "cs",
                    "us",
                    "sy",
                    "id",
                    "wa",
                    "st",
                    "gu",
                );
            }
            "free" => {
                //                total        used        free      shared  buff/cache   available
                // Mem:            3907         280        3080           0         690        3627
                // Swap:              0           0           0
                println!(
                    "{:10}       {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                    "Timestamp", "Total", "Used", "Free", "Shared", "Buff/cache", "Available",
                );
            }
            &_ => todo! {},
        }
    }
    if !statistics
        .get(&("vmstat".to_string(), "".to_string(), "pgpgin".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "vmstat".to_string(),
            key2: "".to_string(),
            key3: "pgpgin".to_string(),
        })?
        .updated_value
    {
        return Ok(());
    };
    let timestamp = statistics
        .get(&("vmstat".to_string(), "".to_string(), "pgpgin".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "vmstat".to_string(),
            key2: "".to_string(),
            key3: "pgpgin".to_string(),
        })?
        .last_timestamp;

    macro_rules! generate_assignmets_for_used_statistics {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string()))
                    .ok_or(ProcessorError::UnableToFindKeyInHashMap { hashmap: "statistics".to_string(), key1: "vmstat".to_string(), key2: "".to_string(), key3: stringify!($field_name).to_string() })?.per_second_value;
            )*
        };
    }
    generate_assignmets_for_used_statistics!(
        pswpin,
        pswpout,
        pgpgin,
        pgpgout,
        pgfault,
        pgmajfault,
        pgfree,
        pgscan_kswapd,
        pgscan_direct,
        pgsteal_anon,
        pgsteal_file,
        pgpromote_success,
        pgdemote_kswapd,
        pgdemote_direct,
        pgdemote_khugepaged
    );
    let processes_running = statistics
        .get(&(
            "stat".to_string(),
            "".to_string(),
            "processes_running".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "".to_string(),
            key3: "processes_running".to_string(),
        })?
        .last_value;
    let processes_blocked = statistics
        .get(&(
            "stat".to_string(),
            "".to_string(),
            "processes_blocked".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "".to_string(),
            key3: "processes_blocked".to_string(),
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
    let mem_free = statistics
        .get(&("meminfo".to_string(), "".to_string(), "memfree".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "memfree".to_string(),
        })?
        .last_value;
    let mem_total = statistics
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
    let mem_buffers = statistics
        .get(&("meminfo".to_string(), "".to_string(), "buffers".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "buffers".to_string(),
        })?
        .last_value;
    let mem_cached = statistics
        .get(&("meminfo".to_string(), "".to_string(), "cached".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "meminfo".to_string(),
            key2: "".to_string(),
            key3: "cached".to_string(),
        })?
        .last_value;
    let mem_available = statistics
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
    let disk_list: Vec<_> = statistics
        .keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();
    let mut total_reads_sectors = 0_f64;
    let mut total_writes_sectors = 0_f64;
    for disk_name in &disk_list {
        total_reads_sectors += statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_reads_sectors".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_reads_sectors".to_string(),
            })?
            .per_second_value;
        total_writes_sectors += statistics
            .get(&(
                "blockdevice".to_string(),
                disk_name.to_string(),
                "stat_writes_sectors".to_string(),
            ))
            .ok_or(ProcessorError::UnableToFindKeyInHashMap {
                hashmap: "statistics".to_string(),
                key1: "blockdevice".to_string(),
                key2: disk_name.to_string(),
                key3: "stat_writes_sectors".to_string(),
            })?
            .per_second_value;
    }
    let interrupts = statistics
        .get(&(
            "stat".to_string(),
            "".to_string(),
            "interrupts_total".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "".to_string(),
            key3: "interrupts_total".to_string(),
        })?
        .per_second_value;
    let context_switches = statistics
        .get(&(
            "stat".to_string(),
            "".to_string(),
            "context_switches".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "".to_string(),
            key3: "context_switches".to_string(),
        })?
        .per_second_value;
    let user = statistics
        .get(&("stat".to_string(), "all".to_string(), "user".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "user".to_string(),
        })?
        .per_second_value;
    let nice = statistics
        .get(&("stat".to_string(), "all".to_string(), "nice".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "nice".to_string(),
        })?
        .per_second_value;
    let system = statistics
        .get(&("stat".to_string(), "all".to_string(), "system".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "system".to_string(),
        })?
        .per_second_value;
    let iowait = statistics
        .get(&("stat".to_string(), "all".to_string(), "iowait".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "iowait".to_string(),
        })?
        .per_second_value;
    let steal = statistics
        .get(&("stat".to_string(), "all".to_string(), "steal".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "steal".to_string(),
        })?
        .per_second_value;
    let irq = statistics
        .get(&("stat".to_string(), "all".to_string(), "irq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "irq".to_string(),
        })?
        .per_second_value;
    let softirq = statistics
        .get(&("stat".to_string(), "all".to_string(), "softirq".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "softirq".to_string(),
        })?
        .per_second_value;
    let guest_user = statistics
        .get(&("stat".to_string(), "all".to_string(), "guest".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest".to_string(),
        })?
        .per_second_value;
    let guest_nice = statistics
        .get(&(
            "stat".to_string(),
            "all".to_string(),
            "guest_nice".to_string(),
        ))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "guest_nice".to_string(),
        })?
        .per_second_value;
    let idle = statistics
        .get(&("stat".to_string(), "all".to_string(), "idle".to_string()))
        .ok_or(ProcessorError::UnableToFindKeyInHashMap {
            hashmap: "statistics".to_string(),
            key1: "stat".to_string(),
            key2: "all".to_string(),
            key3: "idle".to_string(),
        })?
        .per_second_value;
    let total =
        user + nice + system + iowait + steal + irq + softirq + guest_user + guest_nice + idle;

    match output {
        "sar-B" => {
            println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                     timestamp.format("%H:%M:%S"),
                     "all",
                    pgpgin,
                    pgpgout,
                    pgfault,
                    pgmajfault,
                    pgfree,
                    pgscan_kswapd,
                    pgscan_direct,
                    pgsteal_anon + pgsteal_file,
                    pgpromote_success,
                    pgdemote_kswapd + pgdemote_direct + pgdemote_khugepaged,

            );
        }
        "sar-W" => {
            println!(
                "{:10} {:7}    {:10.2} {:10.2}",
                timestamp.format("%H:%M:%S"),
                "",
                pswpin,
                pswpout,
            );
        }
        "vmstat" => {
            println!("{:10} {:4.0} {:4.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:8.0} {:4.0} {:4.0} {:4.0} {:4.0} {:4.0} {:4.0}",
                timestamp.format("%H:%M:%S"),
                processes_running - 1_f64, // not count ourselves
                processes_blocked,
                (swap_total-swap_free).max(0_f64) / 1024_f64,
                mem_free / 1024_f64,
                mem_buffers / 1024_f64,
                mem_cached / 1024_f64,
                pswpin,
                pswpout,
                total_reads_sectors / 1024_f64,
                total_writes_sectors / 1024_f64,
                interrupts,
                context_switches,
                user / total * 100_f64,
                system / total * 100_f64,
                idle / total * 100_f64,
                iowait / total * 100_f64,
                steal / total * 100_f64,
                 guest_user / total * 100_f64,
            );
        }
        "free" => {
            //                total        used        free      shared  buff/cache   available
            // Mem:            3907         280        3080           0         690        3627
            // Swap:              0           0           0
            println!(
                "{:10} Mem:  {:>10.0} {:>10.0} {:>10.0} {:>10.0} {:>10.0} {:>10.0}",
                timestamp.format("%H:%M:%S"),
                mem_total / 1024_f64,
                // it turns out there are multiple explanations of how 'used' is calculated
                // .. and so far none of them create the same value as 'used' with the free command
                // used on ubuntu 23.10. Let's take total-free-buffers-cached for now.
                (mem_total - mem_free - mem_buffers - mem_cached) / 1024_f64,
                mem_free / 1024_f64,
                0_f64,
                mem_buffers + mem_cached / 1024_f64,
                mem_available / 1024_f64,
            );
            println!(
                "{:10} Swap: {:>10.0} {:>10.0} {:>10.0}",
                timestamp.format("%H:%M:%S"),
                swap_total / 1024_f64,
                (swap_total - swap_free).max(0_f64) / 1024_f64,
                swap_free / 1024_f64,
            );
        }
        &_ => todo! {},
    }
    Ok(())
}
