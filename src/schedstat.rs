use std::collections::HashMap;
use crate::common::{ProcData, single_statistic_u64, Statistic};

pub async fn process_schedstat_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{

    let mut scheduler_total_time_running = 0;
    let mut scheduler_total_time_waiting = 0;
    let mut scheduler_total_timeslices = 0;
    for cpu_data in &proc_data.schedstat.cpu {
        single_statistic_u64("schedstat", format!("cpu{}", cpu_data[0]).as_str(), "time_running", proc_data.timestamp, cpu_data[7], statistics).await;
        scheduler_total_time_running += cpu_data[7];
        single_statistic_u64("schedstat", format!("cpu{}", cpu_data[0]).as_str(), "time_waiting", proc_data.timestamp, cpu_data[8], statistics).await;
        scheduler_total_time_waiting += cpu_data[8];
        single_statistic_u64("schedstat", format!("cpu{}", cpu_data[0]).as_str(), "timeslices", proc_data.timestamp, cpu_data[9], statistics).await;
        scheduler_total_timeslices += cpu_data[9];
    }
    single_statistic_u64("schedstat", "all","time_running", proc_data.timestamp, scheduler_total_time_running, statistics).await;
    single_statistic_u64("schedstat", "all","time_waiting", proc_data.timestamp, scheduler_total_time_waiting, statistics).await;
    single_statistic_u64("schedstat", "all","timeslices", proc_data.timestamp, scheduler_total_timeslices, statistics).await;
}

#[cfg(test)]
mod tests {
    use proc_sys_parser::net_dev::InterfaceStats;
    use proc_sys_parser::net_dev::ProcNetDev;
    use proc_sys_parser::meminfo::ProcMemInfo;
    use proc_sys_parser::schedstat::ProcSchedStat;
    use proc_sys_parser::stat::ProcStat;
    use proc_sys_parser::stat::CpuStat;
    use chrono::DateTime;
    use proc_sys_parser::block::{BlockDevice, SysBlock};
    use super::*;

    #[tokio::test]
    async fn process_schedstat_data_output() {
        let proc_data = ProcData { timestamp: DateTime::parse_from_rfc3339("2023-12-13T15:20:24.291337737+00:00").unwrap().into(),
                                   stat: ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 331400, nice: 1170, system: 479040, idle: 445855520, iowait: Some(15240), irq: Some(0), softirq: Some(3860), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                       cpu_individual: vec![CpuStat { name: "cpu0".to_string(), user: 53340, nice: 430, system: 83740, idle: 74272640, iowait: Some(2900), irq: Some(0), softirq: Some(3680), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                           CpuStat { name: "cpu1".to_string(), user: 58090, nice: 370, system: 81090, idle: 74311330, iowait: Some(2210), irq: Some(0), softirq: Some(10), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                           CpuStat { name: "cpu2".to_string(), user: 53540, nice: 0, system: 79770, idle: 74316290, iowait: Some(2830), irq: Some(0), softirq: Some(30), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                           CpuStat { name: "cpu3".to_string(), user: 56090, nice: 0, system: 78810, idle: 74313730, iowait: Some(2780), irq: Some(0), softirq: Some(30), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                           CpuStat { name: "cpu4".to_string(), user: 55520, nice: 0, system: 77680, idle: 74322780, iowait: Some(2380), irq: Some(0), softirq: Some(40), steal: Some(0), guest: Some(0), guest_nice: Some(0) },
                                           CpuStat { name: "cpu5".to_string(), user: 54800, nice: 370, system: 77930, idle: 74318720, iowait: Some(2120), irq: Some(0), softirq: Some(40), steal: Some(0), guest: Some(0), guest_nice: Some(0) }],
                                       interrupts: vec![15506823, 0, 415285, 4858998, 0, 0, 0, 2, 0, 0, 0, 9394136, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 676, 0, 0, 0, 0, 0, 2, 0, 8561, 3561, 5232, 7034, 4456, 2674, 0, 0, 0, 6752, 6297, 6217, 6530, 5971, 5885, 0, 236421, 167973, 0, 0, 1226, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 362934, 0],
                                       context_switches: 25606100,
                                       boot_time: 1702406222,
                                       processes: 307077,
                                       processes_running: 1,
                                       processes_blocked: 0,
                                       softirq: vec![5828301, 29, 1309452, 3, 362338, 11, 0, 311, 1998829, 0, 2157328] },
            schedstat: ProcSchedStat { version: 15, timestamp: 4313542756, cpu: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 217794498927, 8771639928, 2509871],
                vec![1, 0, 0, 0, 0, 0, 0, 206982770373, 7026635877, 2113821],
                vec![2, 0, 0, 0, 0, 0, 0, 202767854725, 6922696498, 2133076],
                vec![3, 0, 0, 0, 0, 0, 0, 203424540826, 6963648533, 2168592],
                vec![4, 0, 0, 0, 0, 0, 0, 199197308929, 6651544983, 2100715],
                vec![5, 0, 0, 0, 0, 0, 0, 202369695269, 6838385853, 2143574]],
                domain: vec![
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]] },
            meminfo: ProcMemInfo { memtotal: 3997876, memfree: 2372092, memavailable: 3651296, buffers: 42532, cached: 1337228, swapcached: 0, active: 696380, inactive: 768204, active_anon: 84664, inactive_anon: 5200, active_file: 611716, inactive_file: 763004, unevictable: 4000, mlocked: 0, swaptotal: 0, swapfree: 0, zswap: 0, zswapped: 0, dirty: 32172, writeback: 0, anonpages: 88940, mapped: 137120, shmem: 5016, kreclaimable: 64240, slab: 102188, sreclaimable: 64240, sunreclaim: 37948, kernelstack: 3328, shadowcallstack: 856, pagetables: 2744, secpagetables: 0, nfs_unstable: 0, bounce: 0, writebacktmp: 0, commitlimit: 1998936, committed_as: 1123336, vmalloctotal: 133141626880, vmallocused: 14260, vmallocchunk: 0, percpu: 2256, hardwarecorrupted: 0, anonhugepages: 6144, shmemhugepages: 0, shmempmdmapped: 0, filehugepages: 0, filepmdmapped: 0, cmatotal: 32768, cmafree: 31232, hugepages_total: 0, hugepages_free: 0, hugepages_rsvd: 0, hugepages_surp: 0, hugepagesize: 2048, hugetlb: 0 },
            blockdevices: SysBlock {
                block_devices: vec![
                    BlockDevice {
                    dev_block_major: 253,
                    dev_block_minor: 0,
                    device_name: "sda".to_string(),
                    discard_alignment: 0,
                    stat_reads_completed_success: 9718,
                    stat_reads_merged: 3826,
                    stat_reads_sectors: 1052371,
                    stat_reads_time_spent_ms: 3026,
                    stat_writes_completed_success: 2856,
                    stat_writes_merged: 2331,
                    stat_writes_sectors: 312397,
                    stat_writes_time_spent_ms: 1947,
                    stat_ios_in_progress: 0,
                    stat_ios_time_spent_ms: 6004,
                    stat_ios_weighted_time_spent_ms: 5554,
                    stat_discards_completed_success: Some(
                        7141,
                    ),
                    stat_discards_merged: Some(
                        0,
                    ),
                    stat_discards_sectors: Some(
                        88014755,
                    ),
                    stat_discards_time_spent_ms: Some(
                        276,
                    ),
                    stat_flush_requests_completed_success: Some(
                        591,
                    ),
                    stat_flush_requests_time_spent_ms: Some(
                        304,
                    ),
                    alignment_offset: 0,
                    cache_type: "write back".to_string(),
                    diskseq: 9,
                    hidden: 0,
                    inflight_reads: 1,
                    inflight_writes: 2,
                    range: 16,
                    removable: 0,
                    ro: 0,
                    size: 125829120,
                    queue_max_hw_sectors_kb: 2147483647,
                    queue_max_sectors_kb: 1280,
                    queue_max_discard_segments: 1,
                    queue_nr_requests: 256,
                    queue_nr_zones: Some(
                        0,
                    ),
                    queue_scheduler: "none".to_string(),
                    queue_rotational: 1,
                    queue_dax: 0,
                    queue_add_random: 0,
                    queue_discard_granularity: 512,
                    queue_discard_max_hw_bytes: 2147483136,
                    queue_discard_max_bytes: 2147483136,
                    queue_hw_sector_size: 512,
                    queue_io_poll: 0,
                    queue_io_poll_delay: -1,
                    queue_logical_block_size: 512,
                    queue_minimum_io_size: 512,
                    queue_max_integrity_segments: 0,
                    queue_max_segments: 254,
                    queue_max_segment_size: 4294967295,
                    queue_nomerges: 0,
                    queue_physical_block_size: 512,
                    queue_optimal_io_size: 0,
                    queue_read_ahead_kb: 128,
                    queue_rq_affinity: 1,
                    queue_write_cache: "write back".to_string(),
                    queue_write_same_max_bytes: 0,
                    queue_chunk_sectors: Some(
                        0,
                    ),
                    queue_zoned: Some(
                        "none".to_string(),
                    ),
                }],
            },
            net_dev: ProcNetDev { interface: vec![InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
                InterfaceStats { name: "eth0".to_string(), receive_bytes: 13351708, receive_packets: 3311, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 227904, transmit_packets: 2477, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }] },
            loadavg: Default::default(),
            pressure: Default::default(),
        };
        let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
        process_schedstat_data(&proc_data, &mut statistics).await;
        println!("{:#?}", statistics);
    }
}