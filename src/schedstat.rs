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
    use proc_sys_parser::diskstats::DiskStats;
    use proc_sys_parser::diskstats::ProcDiskStats;
    use proc_sys_parser::meminfo::ProcMemInfo;
    use proc_sys_parser::schedstat::ProcSchedStat;
    use proc_sys_parser::stat::ProcStat;
    use proc_sys_parser::stat::CpuStat;
    use chrono::DateTime;
    use super::*;

    #[tokio::test]
    async fn process_schedstat_data_output() {
        let proc_data = ProcData { timestamp: DateTime::parse_from_rfc3339("2023-12-13T15:20:24.291337737+00:00").unwrap().into(),
                                   stat: ProcStat { cpu_total: CpuStat { name: "cpu".to_string(), user: 331400, nice: 1170, system: 479040, idle: 445855520, iowait: 15240, irq: 0, softirq: 3860, steal: 0, guest: 0, guest_nice: 0 },
                                       cpu_individual: vec![CpuStat { name: "cpu0".to_string(), user: 53340, nice: 430, system: 83740, idle: 74272640, iowait: 2900, irq: 0, softirq: 3680, steal: 0, guest: 0, guest_nice: 0 },
                                           CpuStat { name: "cpu1".to_string(), user: 58090, nice: 370, system: 81090, idle: 74311330, iowait: 2210, irq: 0, softirq: 10, steal: 0, guest: 0, guest_nice: 0 },
                                           CpuStat { name: "cpu2".to_string(), user: 53540, nice: 0, system: 79770, idle: 74316290, iowait: 2830, irq: 0, softirq: 30, steal: 0, guest: 0, guest_nice: 0 },
                                           CpuStat { name: "cpu3".to_string(), user: 56090, nice: 0, system: 78810, idle: 74313730, iowait: 2780, irq: 0, softirq: 30, steal: 0, guest: 0, guest_nice: 0 },
                                           CpuStat { name: "cpu4".to_string(), user: 55520, nice: 0, system: 77680, idle: 74322780, iowait: 2380, irq: 0, softirq: 40, steal: 0, guest: 0, guest_nice: 0 },
                                           CpuStat { name: "cpu5".to_string(), user: 54800, nice: 370, system: 77930, idle: 74318720, iowait: 2120, irq: 0, softirq: 40, steal: 0, guest: 0, guest_nice: 0 }],
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
            diskstats: ProcDiskStats { disk_stats: vec![DiskStats { block_major: 7, block_minor: 0, device_name: "loop0".to_string(), reads_completed_success: 11, reads_merged: 0, reads_sectors: 28, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 8, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 1, device_name: "loop1".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 2, device_name: "loop2".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 3, device_name: "loop3".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 4, device_name: "loop4".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 5, device_name: "loop5".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 6, device_name: "loop6".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 7, block_minor: 7, device_name: "loop7".to_string(), reads_completed_success: 0, reads_merged: 0, reads_sectors: 0, reads_time_spent_ms: 0, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 0, ios_weighted_time_spent_ms: 0, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 253, block_minor: 0, device_name: "vda".to_string(), reads_completed_success: 14918, reads_merged: 4546, reads_sectors: 1651931, reads_time_spent_ms: 4441, writes_completed_success: 10791, writes_merged: 15418, writes_sectors: 2790997, writes_time_spent_ms: 16247, ios_in_progress: 0, ios_time_spent_ms: 23608, ios_weighted_time_spent_ms: 22933, discards_completed_success: 8119, discards_merged: 0, discards_sectors: 90375803, discards_time_spent_ms: 506, flush_requests_completed_success: 3149, flush_requests_time_spent_ms: 1737 },
                DiskStats { block_major: 253, block_minor: 1, device_name: "vda1".to_string(), reads_completed_success: 14554, reads_merged: 2984, reads_sectors: 1628717, reads_time_spent_ms: 4374, writes_completed_success: 10756, writes_merged: 15331, writes_sectors: 2790680, writes_time_spent_ms: 16227, ios_in_progress: 0, ios_time_spent_ms: 23584, ios_weighted_time_spent_ms: 21106, discards_completed_success: 8092, discards_merged: 0, discards_sectors: 88550232, discards_time_spent_ms: 505, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 253, block_minor: 15, device_name: "vda15".to_string(), reads_completed_success: 136, reads_merged: 1547, reads_sectors: 9919, reads_time_spent_ms: 18, writes_completed_success: 1, writes_merged: 0, writes_sectors: 1, writes_time_spent_ms: 1, ios_in_progress: 0, ios_time_spent_ms: 48, ios_weighted_time_spent_ms: 20, discards_completed_success: 1, discards_merged: 0, discards_sectors: 186691, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 259, block_minor: 0, device_name: "vda16".to_string(), reads_completed_success: 181, reads_merged: 15, reads_sectors: 11583, reads_time_spent_ms: 39, writes_completed_success: 34, writes_merged: 87, writes_sectors: 316, writes_time_spent_ms: 18, ios_in_progress: 0, ios_time_spent_ms: 96, ios_weighted_time_spent_ms: 59, discards_completed_success: 26, discards_merged: 0, discards_sectors: 1638880, discards_time_spent_ms: 1, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 },
                DiskStats { block_major: 11, block_minor: 0, device_name: "sr0".to_string(), reads_completed_success: 291, reads_merged: 0, reads_sectors: 75108, reads_time_spent_ms: 66, writes_completed_success: 0, writes_merged: 0, writes_sectors: 0, writes_time_spent_ms: 0, ios_in_progress: 0, ios_time_spent_ms: 156, ios_weighted_time_spent_ms: 66, discards_completed_success: 0, discards_merged: 0, discards_sectors: 0, discards_time_spent_ms: 0, flush_requests_completed_success: 0, flush_requests_time_spent_ms: 0 }] },
            net_dev: ProcNetDev { interface: vec![InterfaceStats { name: "lo".to_string(), receive_bytes: 0, receive_packets: 0, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 0, transmit_packets: 0, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 },
                InterfaceStats { name: "eth0".to_string(), receive_bytes: 13351708, receive_packets: 3311, receive_errors: 0, receive_drop: 0, receive_fifo: 0, receive_frame: 0, receive_compressed: 0, receive_multicast: 0, transmit_bytes: 227904, transmit_packets: 2477, transmit_errors: 0, transmit_drop: 0, transmit_fifo: 0, transmit_collisions: 0, transmit_carrier: 0, transmit_compressed: 0 }] } };
        let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
        process_schedstat_data(&proc_data, &mut statistics).await;
        println!("{:#?}", statistics);
    }
}