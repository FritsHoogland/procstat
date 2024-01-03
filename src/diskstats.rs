use std::collections::{BTreeSet, HashMap};
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::prelude::{AreaSeries, BLACK, LineSeries, RED, ShapeStyle, TRANSPARENT, WHITE};
use plotters::prelude::full_palette::PURPLE;
use crate::common::{ProcData, single_statistic_u64, single_statistic_option_u64, Statistic};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::{GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH};
use crate::pressure::pressure_io_plot;

#[derive(Debug)]
pub struct BlockDeviceInfo {
    pub timestamp: DateTime<Local>,
    pub device_name: String,
    pub reads_completed_success: f64,
    pub reads_merged: f64,
    pub reads_bytes: f64,
    pub reads_time_spent_ms: f64,
    pub writes_completed_success: f64,
    pub writes_merged: f64,
    pub writes_bytes: f64,
    pub writes_time_spent_ms: f64,
    pub ios_in_progress: f64,
    pub ios_time_spent_ms: f64,
    pub ios_weighted_time_spent_ms: f64,
    pub discards_completed_success: f64,
    pub discards_merged: f64,
    pub discards_bytes: f64,
    pub discards_time_spent_ms: f64,
    pub flush_requests_completed_success: f64,
    pub flush_requests_time_spent_ms: f64,
}
pub async fn process_blockdevice_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    for disk in &proc_data.blockdevices.block_devices
    {
        macro_rules! add_diskstats_data_to_statistics_u64 {
            ($($field_name:ident),*) => {
                $(
                    single_statistic_u64("blockdevice", &disk.device_name, stringify!($field_name), proc_data.timestamp, disk.$field_name, statistics).await;
                )*
            };
        }
        add_diskstats_data_to_statistics_u64!(stat_reads_completed_success, stat_reads_merged, stat_reads_sectors, stat_reads_time_spent_ms, stat_writes_completed_success, stat_writes_merged, stat_writes_sectors, stat_writes_time_spent_ms, stat_ios_in_progress, stat_ios_time_spent_ms, stat_ios_weighted_time_spent_ms);
        macro_rules! add_diskstats_data_to_statistics_option_u64 {
            ($($field_name:ident),*) => {
                $(
                    single_statistic_option_u64("blockdevice", &disk.device_name, stringify!($field_name), proc_data.timestamp, disk.$field_name, statistics).await;
                )*
            };
        }
        add_diskstats_data_to_statistics_option_u64!(stat_discards_completed_success, stat_discards_merged, stat_discards_sectors, stat_discards_time_spent_ms, stat_flush_requests_completed_success, stat_flush_requests_time_spent_ms);
    }
}

pub async fn add_blockdevices_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    let disk_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics.get(&("blockdevice".to_string(), disk_list[0].to_string(), "stat_reads_completed_success".to_string())).unwrap().updated_value { return };
    
    let mut totals = [0_f64; 17];

    let timestamp = statistics.get(&("blockdevice".to_string(), disk_list[0].to_string(), "stat_reads_completed_success".to_string())).unwrap().last_timestamp;

    for disk_name in disk_list.iter().filter(|disk_name| ! disk_name.starts_with("loop") & ! disk_name.starts_with("sr"))
    {
        // reads
        let reads_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_completed_success".to_string())).unwrap().per_second_value;
        totals[0] += reads_completed_success;
        let reads_merged = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_merged".to_string())).unwrap().per_second_value;
        totals[1] += reads_merged;
        let reads_bytes = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_sectors".to_string())).unwrap().per_second_value * 512_f64; // convert 512 bytes sector reads to bytes
        totals[2] += reads_bytes;
        let reads_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[3] += reads_time_spent_ms;
        // writes
        let writes_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_completed_success".to_string())).unwrap().per_second_value;
        totals[4] += writes_completed_success;
        let writes_merged = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_merged".to_string())).unwrap().per_second_value;
        totals[5] += writes_merged;
        let writes_bytes = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_sectors".to_string())).unwrap().per_second_value * 512_f64; // convert 512 bytes sector reads to bytes
        totals[6] += writes_bytes;
        let writes_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[7] += writes_time_spent_ms;
        // ios
        let ios_in_progress = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_ios_in_progress".to_string())).unwrap().per_second_value;
        totals[8] += ios_in_progress;
        let ios_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_ios_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[9] += ios_time_spent_ms;
        let ios_weighted_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_ios_weighted_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[10] += ios_weighted_time_spent_ms;
        // discards
        let discards_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_discards_completed_success".to_string())).unwrap().per_second_value;
        totals[11] += discards_completed_success;
        let discards_merged = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_discards_merged".to_string())).unwrap().per_second_value;
        totals[12] += discards_merged;
        let discards_bytes = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_discards_sectors".to_string())).unwrap().per_second_value * 512_f64; // convert 512 bytes sector reads to bytes
        totals[13] += discards_bytes;
        let discards_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_discards_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[14] += discards_time_spent_ms;
        // flushes
        let flush_requests_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_flush_requests_completed_success".to_string())).unwrap().per_second_value;
        totals[15] += flush_requests_completed_success;
        let flush_requests_time_spent_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_flush_requests_time_spent_ms".to_string())).unwrap().per_second_value;
        totals[16] += flush_requests_time_spent_ms;

        HISTORY.blockdevices.write().unwrap().push_back(BlockDeviceInfo {
            timestamp,
            device_name: disk_name.to_string(),
            reads_completed_success,
            reads_merged,
            reads_bytes,
            reads_time_spent_ms,
            writes_completed_success,
            writes_merged,
            writes_bytes,
            writes_time_spent_ms,
            ios_in_progress,
            ios_time_spent_ms,
            ios_weighted_time_spent_ms,
            discards_completed_success,
            discards_merged,
            discards_bytes,
            discards_time_spent_ms,
            flush_requests_completed_success,
            flush_requests_time_spent_ms,
        });
    }
    HISTORY.blockdevices.write().unwrap().push_back(BlockDeviceInfo {
            timestamp,
            device_name: "TOTAL".to_string(),
            reads_completed_success: totals[0],
            reads_merged: totals[1],
            reads_bytes: totals[2],
            reads_time_spent_ms: totals[3],
            writes_completed_success: totals[4],
            writes_merged: totals[5],
            writes_bytes: totals[6],
            writes_time_spent_ms: totals[7],
            ios_in_progress: totals[8],
            ios_time_spent_ms: totals[9],
            ios_weighted_time_spent_ms: totals[10],
            discards_completed_success: totals[11],
            discards_merged: totals[12],
            discards_bytes: totals[13],
            discards_time_spent_ms: totals[14],
            flush_requests_completed_success: totals[15],
            flush_requests_time_spent_ms: totals[16],
    });
}

pub async fn print_diskstats(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
)
{
    let disk_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "blockdevice")
        .map(|(_, disk_name, _)| disk_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics.get(&("blockdevice".to_string(), disk_list[0].to_string(), "stat_reads_completed_success".to_string())).unwrap().updated_value { return };

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
        let timestamp = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_completed_success".to_string())).unwrap().last_timestamp;
        // reads
        let reads_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_completed_success".to_string())).unwrap().per_second_value;
        let reads_merged = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_merged".to_string())).unwrap().per_second_value;
        let reads_bytes = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_sectors".to_string())).unwrap().per_second_value*512_f64; // convert 512 bytes sector reads to bytes
        let reads_bytes_total = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_sectors".to_string())).unwrap().delta_value*512_f64; // convert 512 bytes sector reads to bytes
        let reads_time_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_reads_time_spent_ms".to_string())).unwrap().per_second_value;
        // writes
        let writes_completed_success = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_completed_success".to_string())).unwrap().per_second_value;
        let writes_merged = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_merged".to_string())).unwrap().per_second_value;
        let writes_bytes = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_sectors".to_string())).unwrap().per_second_value*512_f64; // convert 512 bytes sector reads to bytes
        let writes_bytes_total = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_sectors".to_string())).unwrap().delta_value*512_f64; // convert 512 bytes sector reads to bytes
        let writes_time_ms = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_writes_time_spent_ms".to_string())).unwrap().per_second_value;
        //
        let queue_size = statistics.get(&("blockdevice".to_string(), disk_name.to_string(), "stat_ios_weighted_time_spent_ms".to_string())).unwrap().per_second_value/1000_f64; // convert milliseconds to seconds

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

pub fn create_blockdevice_plot(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    blockdevice_mbps_plot(&mut multi_backend, 0, device_name.clone());
    blockdevice_iops_plot(&mut multi_backend, 1, device_name.clone());
    blockdevice_latency_queuedepth_plot(&mut multi_backend, 2, device_name);
}
pub fn create_blockdevice_psi_plot(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((4, 1));
    blockdevice_mbps_plot(&mut multi_backend, 0, device_name.clone());
    blockdevice_iops_plot(&mut multi_backend, 1, device_name.clone());
    blockdevice_latency_queuedepth_plot(&mut multi_backend, 2, device_name);
    pressure_io_plot(&mut multi_backend, 3);
}

fn blockdevice_mbps_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    multi_backend[backend_number].fill(&WHITE).unwrap();
    // MBPS plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| (blockdevices.reads_bytes + blockdevices.writes_bytes) / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} MBPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("MBPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write MBPS
    // this is a stacked graph, so write MBPS = write + read
    let min_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64)).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_bytes / (1024_f64 * 1024_f64);
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, (blockdevice.writes_bytes + blockdevice.reads_bytes) / (1024_f64 * 1024_f64))), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write", min_write_mbps, max_write_mbps, latest_write_mbps))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read MBPS
    let min_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64)).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_bytes / (1024_f64 * 1024_f64);
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes / (1024_f64 * 1024_f64))), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_mbps, max_read_mbps, latest_read_mbps))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

fn blockdevice_iops_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    multi_backend[backend_number].fill(&WHITE).unwrap();
    //
    // IOPS plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.reads_completed_success + blockdevices.flush_requests_completed_success + blockdevices.discards_completed_success)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[1].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} IOPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("IOPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write IOPS
    // this is a stacked graph, so write IOPS = write + read + discard
    let min_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success)), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write IOPS", min_write_iops, max_write_iops, latest_write_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read IOPS
    let min_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_completed_success + blockdevice.discards_completed_success)), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read IOPS", min_read_iops, max_read_iops, latest_read_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // discards IOPS
    let min_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.discards_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.discards_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.discards_completed_success)), 0.0, PURPLE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "discard IOPS", min_discard_iops, max_discard_iops, latest_discard_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

fn blockdevice_latency_queuedepth_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    multi_backend[backend_number].fill(&WHITE).unwrap();
    //
    // read, write and discard latency and queue depth plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value_latencies: f64 = 0.0;
    let high_value_latencies_read = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.reads_time_spent_ms / blockdevices.reads_completed_success).is_nan() { 0_f64 } else { blockdevices.reads_time_spent_ms / blockdevices.reads_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies_write = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.writes_time_spent_ms / blockdevices.writes_completed_success).is_nan() { 0_f64 } else { blockdevices.writes_time_spent_ms / blockdevices.writes_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies_discard = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.discards_time_spent_ms / blockdevices.discards_completed_success).is_nan() { 0_f64 } else { blockdevices.discards_time_spent_ms / blockdevices.discards_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies = high_value_latencies_read.max(high_value_latencies_write).max(high_value_latencies_discard);
    let low_value_queue_depth = 0_f64;
    let high_value_queue_depth = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.ios_weighted_time_spent_ms / 1000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[2].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} latency and queue depth", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value_latencies..high_value_latencies)
        .unwrap()
        .set_secondary_coord(start_time..end_time, low_value_queue_depth..high_value_queue_depth);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Average latency ms")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("queue depth")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write latency
    // this is a line graph, so no stacking.
    let min_write_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_writes_latency = if (historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_time_spent_ms /
        historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_completed_success).is_nan()
    { 0_f64 }
    else
    { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_time_spent_ms /
        historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_completed_success };
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success) }), RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write latency", min_write_latency, max_write_latency, latest_writes_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // read latency
    // this is a line graph, so no stacking.
    let min_read_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_reads_latency = if (historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_completed_success /
        historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_time_spent_ms).is_nan()
    { 0_f64 }
    else
    { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_time_spent_ms /
        historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_completed_success };
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success) }), GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read latency", min_read_latency, max_read_latency, latest_reads_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    //
    // discard latency
    // this is a line graph, so no stacking.
    let min_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_discard_latency = if (historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_time_spent_ms /
        historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_completed_success).is_nan()
    { 0_f64 }
    else
    { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_time_spent_ms /
      historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_completed_success };
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success) }), PURPLE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "discard latency", min_discard_latency, max_discard_latency, latest_discard_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE.filled()));
    //
    // queue depth
    // this is a line graph, and it is bound to the right-hand y axis!
    let min_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_queue_depth = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().ios_weighted_time_spent_ms / 1000_f64;
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| (blockdevice.timestamp, blockdevice.ios_weighted_time_spent_ms / 1000_f64)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "queue depth", min_queue_depth, max_queue_depth, latest_queue_depth))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
