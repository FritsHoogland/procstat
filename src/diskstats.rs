use std::collections::HashMap;
use crate::common::{ProcData, single_statistic, Statistic};

pub async fn process_diskstats_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    for disk in &proc_data.diskstats.disk_stats
    {
        macro_rules! add_diskstats_data_to_statistics {
            ($($field_name:ident),*) => {
                $(
                    single_statistic("diskstats", &disk.device_name, stringify!($field_name), proc_data.timestamp, disk.$field_name, statistics).await;
                )*
            };
        }
        add_diskstats_data_to_statistics!(reads_completed_success, reads_merged, reads_sectors, reads_time_spent_ms, writes_completed_success, writes_merged, writes_sectors, writes_time_spent_ms, discards_completed_success, discards_merged, discards_sectors, discards_time_spent_ms, ios_in_progress, ios_time_spent_ms, ios_weighted_time_spent_ms, flush_requests_completed_success, flush_requests_time_spent_ms);
    }
}

pub async fn print_diskstats(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
)
{
    let mut disk_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "diskstats")
        .map(|(_, disk_name, _)| disk_name)
        .collect();
    disk_list.sort();

    if !statistics.get(&("diskstats".to_string(), disk_list[0].to_string(), "reads_completed_success".to_string())).unwrap().updated_value { return };

    match output
    {
        "sar-d" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
                     "timestamp",
                     "DEV",
                     "tps",
                     "rMB/s",
                     "wMB/s",
                     "areq-sz",
                     "aqu-sz",
                     "await",
            );
        },
        "iostat" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9}",
                "timestamp",
                "Device",
                "tps",
                "MB_read/s",
                "MB_wrtn/s",
                "MB_read",
                "MB_wrtn",
            );
        },
        "iostat-x" => {
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
                     "timestamp",
                     "Device",
                     "r/s",
                     "w/s",
                     "rMB/s",
                     "wMB/s",
                     "rrqm/s",
                     "wrqm/s",
                     "%rrqm/s",
                     "%wrqm/s",
                     "r_await",
                     "w_await",
                     "aqu-sz",
                     "rareq-sz",
                     "wareq-sz",
            );
        },
        &_ => todo!(),
    }

    for disk_name in disk_list
    {
        let timestamp = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_completed_success".to_string())).unwrap().last_timestamp;
        // reads
        let reads_completed_success = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_completed_success".to_string())).unwrap().per_second_value;
        let reads_merged = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_merged".to_string())).unwrap().per_second_value;
        let reads_bytes = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_sectors".to_string())).unwrap().per_second_value*512_f64; // convert 512 bytes sector reads to bytes
        let reads_bytes_total = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_sectors".to_string())).unwrap().delta_value*512_f64; // convert 512 bytes sector reads to bytes
        let reads_time_ms = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "reads_time_spent_ms".to_string())).unwrap().per_second_value;
        // writes
        let writes_completed_success = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "writes_completed_success".to_string())).unwrap().per_second_value;
        let writes_merged = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "writes_merged".to_string())).unwrap().per_second_value;
        let writes_bytes = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "writes_sectors".to_string())).unwrap().per_second_value*512_f64; // convert 512 bytes sector reads to bytes
        let writes_bytes_total = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "writes_sectors".to_string())).unwrap().delta_value*512_f64; // convert 512 bytes sector reads to bytes
        let writes_time_ms = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "writes_time_spent_ms".to_string())).unwrap().per_second_value;
        //
        let queue_size = statistics.get(&("diskstats".to_string(), disk_name.to_string(), "ios_weighted_time_spent_ms".to_string())).unwrap().per_second_value;

        let mut total_average_request_size = (reads_bytes + writes_bytes) / (reads_completed_success + writes_completed_success);
        total_average_request_size = if total_average_request_size.is_nan() { 0_f64 } else { total_average_request_size };
        let mut total_average_request_time = (reads_time_ms + writes_time_ms) / (reads_completed_success + writes_completed_success);
        total_average_request_time = if total_average_request_time.is_nan() { 0_f64 } else { total_average_request_time };
        let mut reads_percentage_merged = (reads_merged / (reads_merged + reads_completed_success)) * 100_f64;
        reads_percentage_merged = if reads_percentage_merged.is_nan() { 0_f64 } else { reads_percentage_merged };
        let mut writes_percentage_merged = (writes_merged / (writes_merged + writes_completed_success)) * 100_f64;
        writes_percentage_merged = if writes_percentage_merged.is_nan() { 0_f64 } else { writes_percentage_merged };
        let mut reads_average_time = reads_time_ms / reads_completed_success;
        reads_average_time = if reads_average_time.is_nan() { 0_f64 } else { reads_average_time };
        let mut writes_average_time = writes_time_ms / writes_completed_success;
        writes_average_time = if writes_average_time.is_nan() { 0_f64 } else { writes_average_time };
        let mut reads_average_request_size = reads_bytes / reads_completed_success;
        reads_average_request_size = if reads_average_request_size.is_nan() { 0_f64 } else { reads_average_request_size };
        let mut writes_average_request_size = writes_bytes / writes_completed_success;
        writes_average_request_size = if writes_average_request_size.is_nan() { 0_f64 } else { writes_average_request_size };

        match output
        {
            "sar-d" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         disk_name,
                         reads_completed_success + writes_completed_success,
                         reads_bytes / (1024_f64 * 1024_f64),
                         writes_bytes / (1024_f64 * 1024_f64),
                         total_average_request_size / (1024_f64 * 1024_f64),
                         queue_size,
                         total_average_request_time,
                );
            },
            "iostat" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         disk_name,
                         reads_completed_success + writes_completed_success,
                         reads_bytes / (1024_f64 * 1024_f64),
                         writes_bytes / (1024_f64 * 1024_f64),
                         reads_bytes_total / (1024_f64 * 1024_f64),
                         writes_bytes_total / (1024_f64 * 1024_f64),
                );
            },
            "iostat-x" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         disk_name,
                         reads_completed_success,
                         writes_completed_success,
                         reads_bytes / (1024_f64 * 1024_f64),
                         writes_bytes / (1024_f64 * 1024_f64),
                         reads_merged,
                         writes_merged,
                         reads_percentage_merged,
                         writes_percentage_merged,
                         reads_average_time,
                         writes_average_time,
                         queue_size,
                         reads_average_request_size / (1024_f64 * 1024_f64),
                         writes_average_request_size / (1024_f64 * 1024_f64),
                );
            },
            &_ => todo!(),
        }
    }
}