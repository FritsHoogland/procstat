use chrono::{DateTime, Local};
use plotters::style::full_palette::{BLUE_300, BLUE_900, ORANGE_900, ORANGE_300, LIGHTGREEN_900, LIGHTGREEN_300};
use crate::meminfo::memory_plot;
use crate::pressure::pressure_memory_plot;
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::ARGS;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use proc_sys_parser::vmstat::ProcVmStat;
use std::collections::{HashMap, BTreeSet};
use serde::{Serialize, Deserialize};
use crate::common::{ProcData, Statistic, single_statistic_u64, single_statistic_option_u64};
use crate::{add_list_of_u64_data_to_statistics, add_list_of_option_u64_data_to_statistics};
use log::debug;

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
}

pub async fn read_vmstat_proc_data() -> ProcVmStat {
    let proc_vmstat = proc_sys_parser::vmstat::read();
    debug!("{:?}", proc_vmstat);
    proc_vmstat
}

pub async fn process_vmstat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>) {
    add_list_of_u64_data_to_statistics!(vmstat, "", proc_data.timestamp, proc_data, vmstat, statistics, nr_free_pages, nr_zone_inactive_anon, nr_zone_active_anon, nr_zone_inactive_file, nr_zone_active_file, nr_zone_unevictable, nr_zone_write_pending, nr_mlock, nr_bounce, nr_zspages, nr_free_cma, numa_hit, numa_miss, numa_foreign, numa_interleave, numa_local, numa_other, nr_inactive_anon, nr_active_anon, nr_active_file, nr_inactive_file, nr_unevictable, nr_slab_reclaimable, nr_slab_unreclaimable, nr_isolated_anon, nr_isolated_file, workingset_nodereclaim, nr_anon_pages, nr_mapped, nr_file_pages, nr_dirty, nr_writeback, nr_writeback_temp, nr_shmem, nr_shmem_hugepages, nr_shmem_pmdmapped, nr_anon_transparent_hugepages, nr_vmscan_write, nr_vmscan_immediate_reclaim, nr_dirtied, nr_written, nr_kernel_stack, nr_page_table_pages, nr_dirty_threshold, nr_dirty_background_threshold, pgpgin, pgpgout, pswpin, pswpout, pgalloc_dma, pgalloc_dma32, pgalloc_normal, pgalloc_movable, allocstall_dma, allocstall_dma32, allocstall_normal, allocstall_movable, pgskip_dma, pgskip_dma32, pgskip_normal, pgskip_movable, pgfree, pgactivate, pgdeactivate, pglazyfree, pglazyfreed, pgrefill, pgfault, pgmajfault, pgsteal_kswapd, pgsteal_direct, pgscan_kswapd, pgscan_direct, pgscan_direct_throttle, zone_reclaim_failed, pginodesteal, kswapd_inodesteal, kswapd_low_wmark_hit_quickly, kswapd_high_wmark_hit_quickly, pageoutrun, pgrotated, drop_pagecache, drop_slab, oom_kill, numa_pte_updates, numa_huge_pte_updates, numa_hint_faults, numa_hint_faults_local, numa_pages_migrated, pgmigrate_success, pgmigrate_fail, compact_migrate_scanned, compact_free_scanned, compact_isolated, compact_stall, compact_fail, compact_success, compact_daemon_wake, compact_daemon_migrate_scanned, compact_daemon_free_scanned, htlb_buddy_alloc_success, htlb_buddy_alloc_fail, unevictable_pgs_culled, unevictable_pgs_scanned, unevictable_pgs_rescued, unevictable_pgs_mlocked, unevictable_pgs_munlocked, unevictable_pgs_cleared, unevictable_pgs_stranded, thp_fault_alloc, thp_fault_fallback, thp_collapse_alloc, thp_collapse_alloc_failed, thp_file_alloc, thp_file_mapped, thp_split_page, thp_split_page_failed, thp_deferred_split_page, thp_split_pmd, thp_zero_page_alloc, thp_zero_page_alloc_failed, thp_swpout, thp_swpout_fallback, balloon_inflate, balloon_deflate, balloon_migrate, swap_ra, swap_ra_hit, nr_unstable);
    add_list_of_option_u64_data_to_statistics!(vmstat, "", proc_data.timestamp, proc_data, vmstat, statistics, workingset_nodes, workingset_restore_anon, workingset_refault_file, workingset_activate_anon, workingset_activate_file, workingset_restore_anon, workingset_restore_file, nr_file_hugepages, nr_file_pmdmapped, nr_throttled_written, nr_kernel_misc_reclaimable, nr_foll_pin_acquired, nr_foll_pin_released, nr_shadow_call_stack, nr_sec_page_table_pages, nr_swapcached, pgpromote_success, pgpromote_candidate, pgalloc_device, pgskip_device, pgreuse, pgsteal_khugepaged, pgdemote_kswapd, pgdemote_direct, pgdemote_khugepaged, pgscan_khugepaged, pgscan_anon, pgscan_file, pgsteal_anon, pgsteal_file, slabs_scanned, thp_migration_success, thp_migration_fail, thp_migration_split, cma_alloc_success, cma_alloc_fail, thp_fault_fallback_charge, thp_file_fallback, thp_file_fallback_charge, thp_scan_exceed_none_pte, thp_scan_exceed_swap_pte, thp_scan_exceed_share_pte, ksm_swpin_copy, cow_ksm, zswpin, zswpout);
}

pub async fn add_vmstat_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("vmstat".to_string(), "".to_string(), "nr_free_pages".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("vmstat".to_string(), "".to_string(), "nr_free_pages".to_string())).unwrap().last_timestamp;

    macro_rules! generate_assignments_for_history_addition_per_second_value {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string())).unwrap_or(&Statistic::default()).per_second_value; 
            )*
        };
    }
    generate_assignments_for_history_addition_per_second_value! (nr_free_pages, nr_zone_inactive_anon, nr_zone_active_anon, nr_zone_inactive_file, nr_zone_active_file, nr_zone_unevictable, nr_zone_write_pending, nr_mlock, nr_bounce, nr_zspages, nr_free_cma, numa_hit, numa_miss, numa_foreign, numa_interleave, numa_local, numa_other, nr_inactive_anon, nr_active_anon, nr_active_file, nr_inactive_file, nr_unevictable, nr_slab_reclaimable, nr_slab_unreclaimable, nr_isolated_anon, nr_isolated_file, workingset_nodes, workingset_refault_anon, workingset_refault_file, workingset_activate_anon, workingset_activate_file, workingset_restore_anon, workingset_restore_file, workingset_nodereclaim, nr_anon_pages, nr_mapped, nr_file_pages, nr_dirty, nr_writeback, nr_writeback_temp, nr_shmem, nr_shmem_hugepages, nr_shmem_pmdmapped, nr_file_hugepages, nr_file_pmdmapped, nr_anon_transparent_hugepages, nr_vmscan_write, nr_vmscan_immediate_reclaim, nr_dirtied, nr_written, nr_throttled_written, nr_kernel_misc_reclaimable, nr_foll_pin_acquired, nr_foll_pin_released, nr_kernel_stack, nr_shadow_call_stack, nr_page_table_pages, nr_sec_page_table_pages, nr_swapcached, pgpromote_success, pgpromote_candidate, nr_dirty_threshold, nr_dirty_background_threshold, pgpgin, pgpgout, pswpin, pswpout, allocstall_dma, allocstall_dma32, allocstall_normal, allocstall_movable, allocstall_device, pgskip_dma, pgskip_dma32, pgskip_normal, pgskip_movable, pgskip_device, pgactivate, pgdeactivate, pglazyfree, pglazyfreed, pgrefill, pgfault, pgmajfault, pgreuse, pgdemote_kswapd, pgdemote_direct, pgdemote_khugepaged, pgscan_direct_throttle, pgscan_anon, pgscan_file, pgsteal_anon, pgsteal_file, zone_reclaim_failed, pginodesteal, slabs_scanned, kswapd_inodesteal, kswapd_low_wmark_hit_quickly, kswapd_high_wmark_hit_quickly, pageoutrun, pgrotated, drop_pagecache, drop_slab, numa_pte_updates, numa_huge_pte_updates, numa_hint_faults, numa_hint_faults_local, numa_pages_migrated, pgmigrate_success, pgmigrate_fail, thp_migration_success, thp_migration_fail, thp_migration_split, compact_migrate_scanned, compact_free_scanned, compact_isolated, compact_stall, compact_fail, compact_success, compact_daemon_wake, compact_daemon_migrate_scanned, compact_daemon_free_scanned, htlb_buddy_alloc_success, htlb_buddy_alloc_fail, cma_alloc_success, cma_alloc_fail, unevictable_pgs_culled, unevictable_pgs_scanned, unevictable_pgs_rescued, unevictable_pgs_mlocked, unevictable_pgs_munlocked, unevictable_pgs_cleared, unevictable_pgs_stranded, thp_fault_alloc, thp_fault_fallback, thp_fault_fallback_charge, thp_collapse_alloc, thp_collapse_alloc_failed, thp_file_alloc, thp_file_fallback, thp_file_fallback_charge, thp_file_mapped, thp_split_page, thp_split_page_failed, thp_deferred_split_page, thp_split_pmd, thp_scan_exceed_none_pte, thp_scan_exceed_swap_pte, thp_scan_exceed_share_pte, thp_zero_page_alloc, thp_zero_page_alloc_failed, thp_swpout, thp_swpout_fallback, balloon_inflate, balloon_deflate, balloon_migrate, swap_ra, swap_ra_hit, ksm_swpin_copy, cow_ksm, zswpin, zswpout, nr_unstable);
    macro_rules! generate_assignments_for_history_addition_delta_value {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string())).unwrap_or(&Statistic::default()).delta_value; 
            )*
        };
    }
    generate_assignments_for_history_addition_delta_value!(oom_kill, pgfree, pgalloc_dma, pgalloc_dma32, pgalloc_normal, pgalloc_device, pgalloc_movable, pgscan_kswapd, pgscan_direct, pgscan_khugepaged, pgsteal_khugepaged, pgsteal_kswapd, pgsteal_direct);
    HISTORY.vmstat.write().unwrap().push_back( VmStatInfo {
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
    });
    debug!("{:?}", HISTORY.vmstat.read().unwrap());
}


pub async fn print_vmstat(statistics: &HashMap<(String, String, String), Statistic>, output: &str, print_header: bool) {
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
            },
            "sar-W" => {
                println!("{:10} {:7}    {:>10} {:>10}",
                         "Timestamp",
                         "",
                         "pswpin/s",
                         "pswpout/s",
                );
            },
            "vmstat" => {
                println!("{:10} {:9} {:35} {:17} {:17} {:17} {:25}",
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
            },
            &_ => todo! {},
        }
    }
    if !statistics.get(&("vmstat".to_string(), "".to_string(), "pgpgin".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("vmstat".to_string(), "".to_string(), "pgpgin".to_string())).unwrap().last_timestamp;

    macro_rules! generate_assignmets_for_used_statistics {
        ($($field_name:ident),*) => {
            $(
                let $field_name = statistics.get(&("vmstat".to_string(), "".to_string(), stringify!($field_name).to_string())).unwrap().per_second_value; 
            )*
        };
    }
    generate_assignmets_for_used_statistics!(pswpin, pswpout, pgpgin, pgpgout, pgfault, pgmajfault, pgfree, pgscan_kswapd, pgscan_direct, pgsteal_anon, pgsteal_file, pgpromote_success, pgdemote_kswapd, pgdemote_direct, pgdemote_khugepaged);
    let processes_running = statistics.get(&("stat".to_string(), "".to_string(), "processes_running".to_string())).unwrap().last_value;
    let processes_blocked = statistics.get(&("stat".to_string(), "".to_string(), "processes_blocked".to_string())).unwrap().last_value;
    let swap_free = statistics.get(&("meminfo".to_string(), "".to_string(), "swapfree".to_string())).unwrap().last_value;
    let swap_total = statistics.get(&("meminfo".to_string(), "".to_string(), "swaptotal".to_string())).unwrap().last_value;
    let mem_free = statistics.get(&("meminfo".to_string(), "".to_string(), "memfree".to_string())).unwrap().last_value;
    let mem_buffers = statistics.get(&("meminfo".to_string(), "".to_string(), "buffers".to_string())).unwrap().last_value;
    let mem_cached = statistics.get(&("meminfo".to_string(), "".to_string(), "cached".to_string())).unwrap().last_value;
    let disk_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();
    let mut total_reads_sectors = 0_f64;
    for disk_name in &disk_list { total_reads_sectors += statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_sectors".to_string())).unwrap().per_second_value; };
    let mut total_writes_sectors = 0_f64;
    for disk_name in &disk_list { total_writes_sectors += statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_sectors".to_string())).unwrap().per_second_value; };
    let interrupts = statistics.get(&("stat".to_string(), "".to_string(), "interrupts_total".to_string())).unwrap().per_second_value;
    let context_switches = statistics.get(&("stat".to_string(), "".to_string(), "context_switches".to_string())).unwrap().per_second_value;
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
        },
        "sar-W" => {
            println!("{:10} {:7}    {:10.2} {:10.2}",
                     timestamp.format("%H:%M:%S"),
                     "",
                    pswpin,
                    pswpout,
            );
        },
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
        &_ => todo!{},
    }
}

pub fn create_memory_alloc_plot(
    buffer: &mut [u8],
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_heighth)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    memory_plot(&mut multi_backend, 0);
    pages_allocated_and_free(&mut multi_backend, 1)
}

pub fn create_memory_alloc_psi_plot(
    buffer: &mut [u8],
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_heighth)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    memory_plot(&mut multi_backend, 0);
    pages_allocated_and_free(&mut multi_backend, 1);
    pressure_memory_plot(&mut multi_backend, 2);
}

pub fn swap_inout_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
    let historical_data_read = HISTORY.vmstat.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .max()
        .unwrap();
    let latest = historical_data_read
        .back()
        .unwrap();
    let high_value = historical_data_read
        .iter()
        .map(|vmstat| (vmstat.pswpin + vmstat.pswpout) * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Swap IO", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Swap IO (pages)")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|vmstat| (vmstat.timestamp, vmstat.pswpin)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
     
    //
    let min_total_swap = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin + vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpin + vmstat.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_swap = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin + vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpin + vmstat.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .map(|vmstat| (vmstat.timestamp, vmstat.pswpin + vmstat.pswpout)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "total", min_total_swap, max_total_swap, (latest.pswpin + latest.pswpout)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // pgspout
    let min_pswpout = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpout)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpout = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpout > 0_f64)
        .map(|vmstat| vmstat.pswpout)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|vmstat| vmstat.pswpout > 0_f64)
                                                .map(|vmstat| Circle::new((vmstat.timestamp, vmstat.pswpout), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pages swap out", min_pswpout, max_pswpout, latest.pswpout))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // pgspin
    let min_pswpin = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin > 0_f64)
        .map(|vmstat| vmstat.pswpin)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_pswpin = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pswpin > 0_f64)
        .map(|vmstat| vmstat.pswpin)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|vmstat| vmstat.pswpin > 0_f64)
                                                .map(|vmstat| Circle::new((vmstat.timestamp, vmstat.pswpin), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pages swap in", min_pswpin, max_pswpin, latest.pswpin))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

pub fn pages_allocated_and_free(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
    let historical_data_read = HISTORY.vmstat.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .map(|vmstat| vmstat.timestamp)
        .max()
        .unwrap_or_default();
    let latest = historical_data_read
        .back();
    let high_value_free = historical_data_read
        .iter()
        .map(|vmstat| vmstat.pgfree * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_alloc = historical_data_read
        .iter()
        .map(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value = high_value_free.max(high_value_alloc);

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Pages allocated and freed", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Pages")
        .y_label_formatter(&|pages| {
            if pages < &1_000_f64             { format!("{:6.0}",   pages)                       } else
            if pages < &1_000_000_f64         { format!("{:7.1} k", pages/1_000_f64)             } else
            if pages < &1_000_000_000_f64     { format!("{:7.1} m", pages/1_000_000_f64)         } else
            if pages < &1_000_000_000_000_f64 { format!("{:7.1} t", pages/1_000_000_000_f64)     } else
                                              { format!("{:7.1} p", pages/1_000_000_000_000_f64) } })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|vmstat| (vmstat.timestamp, vmstat.pgfree)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
     
    //
    let min_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfree > 0_f64)
        .map(|vmstat| vmstat.pgfree)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_free = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgfree > 0_f64)
        .map(|vmstat| vmstat.pgfree)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgfree)), ShapeStyle { color: GREEN.into(), filled: true, stroke_width: 2 }))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgfree", min_free, max_free, latest.map_or(0_f64, |latest| latest.pgfree)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) > 0_f64)
        .map(|vmstat| vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable) > 0_f64)
        .map(|vmstat| vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, (vmstat.pgalloc_dma + vmstat.pgalloc_dma32 + vmstat.pgalloc_normal + vmstat.pgalloc_device + vmstat.pgalloc_movable))), ShapeStyle { color: RED.into(), filled: true, stroke_width: 2 }))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgalloc", min_alloc, max_alloc, (latest.map_or(0_f64, |latest| latest.pgalloc_dma) + latest.map_or(0_f64, |latest| latest.pgalloc_dma32) + latest.map_or(0_f64, |latest| latest.pgalloc_normal) + latest.map_or(0_f64, |latest| latest.pgalloc_device) + latest.map_or(0_f64, |latest| latest.pgalloc_movable))))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // 
    // kswapd: blue
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgsteal_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgsteal_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_kswapd)), BLUE_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_kswapd", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_kswapd)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgscan_kswapd)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_kswapd > 0_f64)
        .map(|vmstat| vmstat.pgscan_kswapd)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_kswapd)), BLUE_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_kswapd", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_kswapd)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_300.filled()));
    //
    // direct: orange
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_direct > 0_f64)
        .map(|vmstat| vmstat.pgsteal_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_direct > 0_f64)
        .map(|vmstat| vmstat.pgsteal_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_direct)), ORANGE_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_direct", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_direct)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_direct > 0_f64)
        .map(|vmstat| vmstat.pgscan_direct)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_direct > 0_f64)
        .map(|vmstat| vmstat.pgscan_direct)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_direct)), ORANGE_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_direct", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_direct)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], ORANGE_300.filled()));
    //
    // khugepaged
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgsteal_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgsteal_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgsteal_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgsteal_khugepaged)), LIGHTGREEN_900))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgsteal_khugepaged", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgsteal_khugepaged)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_900.filled()));
    //
    let min_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgscan_khugepaged)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_alloc = historical_data_read
        .iter()
        .filter(|vmstat| vmstat.pgscan_khugepaged > 0_f64)
        .map(|vmstat| vmstat.pgscan_khugepaged)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
            .iter()
            .map(|vmstat| (vmstat.timestamp, vmstat.pgscan_khugepaged)), LIGHTGREEN_300))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "pgscan_khugepaged", min_alloc, max_alloc, latest.map_or(0_f64, |latest| latest.pgscan_khugepaged)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], LIGHTGREEN_300.filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
