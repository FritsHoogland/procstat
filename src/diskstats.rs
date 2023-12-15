use std::collections::HashMap;
use crate::{ProcData, single_statistic, Statistic};

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