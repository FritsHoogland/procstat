use plotters::backend::{BitMapBackend, RGBPixel};

use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::element::Rectangle;
use plotters::prelude::{*, full_palette::PURPLE};
use plotters::style::colors::full_palette::{RED_100, GREY_100, GREEN_500};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::webserver::pressure::pressure_io_plot;
use crate::ARGS;

pub fn create_blockdevice_plot(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let nr = if device_name == "TOTAL" { 3 } else { 4 };
    let mut multi_backend = backend.split_evenly((nr, 1));
    blockdevice_mbps_plot(&mut multi_backend, 0, device_name.clone());
    blockdevice_iops_plot(&mut multi_backend, 1, device_name.clone());
    blockdevice_latency_queuedepth_plot(&mut multi_backend, 2, device_name.clone());
    if device_name != "TOTAL" { blockdevice_iosize_plot(&mut multi_backend, 3, device_name) };
}

pub fn create_blockdevice_plot_extra(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
    let nr = if device_name == "TOTAL" { 3 } else { 5 };
    let mut multi_backend = backend.split_evenly((nr, 1));
    blockdevice_mbps_plot(&mut multi_backend, 0, device_name.clone());
    blockdevice_iops_plot(&mut multi_backend, 1, device_name.clone());
    blockdevice_latency_queuedepth_plot(&mut multi_backend, 2, device_name.clone());
    if device_name != "TOTAL" { 
        blockdevice_iosize_plot(&mut multi_backend, 3, device_name.clone());
        blockdevice_extra(&mut multi_backend, 4, device_name.clone());
    };
}

pub fn create_blockdevice_psi_plot(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height)).into_drawing_area();
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
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap_or_default();
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| ((blockdevices.reads_bytes + blockdevices.writes_bytes) / (1024_f64 * 1024_f64)) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last();

    // create the plot
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} MBPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("MBPS")
        .y_label_formatter(&|mbps| {
                 if mbps == &0_f64  { format!("{:5.0}", mbps) }
            else if mbps  < &1_f64  { format!("{:5.3}", mbps) }
            else                    { format!("{:5.0}", mbps) }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                                .take(1)
                                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // total MBPS
    // this is a line graph, so total MBPS = read + write bytes.
    // discards to not add to bandwidth.
    let min_total_mbps = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && (blockdevice.writes_bytes + blockdevice.reads_bytes) > 0_f64)
        .map(|blockdevice| (blockdevice.writes_bytes + blockdevice.reads_bytes) / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_mbps = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && (blockdevice.writes_bytes + blockdevice.reads_bytes) > 0_f64)
        .map(|blockdevice| (blockdevice.writes_bytes + blockdevice.reads_bytes) / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, (blockdevice.writes_bytes + blockdevice.reads_bytes) / (1024_f64 * 1024_f64))), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "total", min_total_mbps, max_total_mbps, ((latest.map_or(0_f64, | latest| latest.writes_bytes + latest.reads_bytes)) / (1024_f64 * 1024_f64))))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // write MBPS
    let min_write_mbps = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_bytes > 0_f64)
        .map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_write_mbps = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_bytes > 0_f64)
        .map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_bytes > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.writes_bytes / (1024_f64 * 1024_f64)), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write", min_write_mbps, max_write_mbps, latest.map_or(0_f64, |latest| latest.writes_bytes) / (1024_f64 * 1024_f64)))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // read MBPS
    let min_read_mbps = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_bytes > 0_f64)
        .map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_read_mbps = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_bytes > 0_f64)
        .map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_bytes > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.reads_bytes / (1024_f64 * 1024_f64)), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_mbps, max_read_mbps, latest.map_or(0_f64, |latest| latest.reads_bytes) / (1024_f64 * 1024_f64)))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
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
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap_or_default();
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| (blockdevices.writes_completed_success + blockdevices.reads_completed_success + blockdevices.discards_completed_success) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last();

    // create the plot
    multi_backend[1].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} IOPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("IOPS")
        .y_label_formatter(&|iops| {
                 if iops == &0_f64  { format!("{:5.0}", iops) }
            else if iops  < &10_f64 { format!("{:5.1}", iops) }
            else                    { format!("{:5.0}", iops) }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // total IOPS
    // this is a line graph, so total IOPS = write + read + discard
    let min_total_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && (blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success) > 0_f64)
        .map(|blockdevice| (blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| (blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "total", min_total_iops, max_total_iops, (latest.map_or(0_f64, |latest| latest.writes_completed_success + latest.reads_completed_success + latest.discards_completed_success))))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // write IOPS
    let min_write_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.writes_completed_success)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_write_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.writes_completed_success)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.writes_completed_success), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "writes", min_write_iops, max_write_iops, latest.map_or(0_f64, |latest| latest.writes_completed_success)))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // read IOPS
    let min_read_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.reads_completed_success)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_read_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.reads_completed_success)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.reads_completed_success), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_iops, max_read_iops, latest.map_or(0_f64, |latest| latest.reads_completed_success)))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // discards IOPS
    let min_discard_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.discards_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.discards_completed_success)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_discard_iops = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.discards_completed_success)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.discards_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.discards_completed_success), 2, PURPLE.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "discard", min_discard_iops, max_discard_iops, latest.map_or(0_f64, |latest| latest.discards_completed_success)))
        .legend(move |(x, y)| Circle::new((x, y), 2, PURPLE.filled()));
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
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap_or_default();
    let high_value_latencies_read = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.reads_completed_success == 0_f64 { 0_f64 } else { (blockdevices.reads_time_spent_ms / blockdevices.reads_completed_success) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_latencies_write = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.writes_completed_success == 0_f64 { 0_f64 } else { (blockdevices.writes_time_spent_ms / blockdevices.writes_completed_success) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_latencies_discard = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.discards_completed_success == 0_f64 { 0_f64 } else { (blockdevices.discards_time_spent_ms / blockdevices.discards_completed_success) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_latencies_flush = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.flush_requests_completed_success == 0_f64 { 0_f64 } else { (blockdevices.flush_requests_time_spent_ms / blockdevices.flush_requests_completed_success) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_latencies = high_value_latencies_read.max(high_value_latencies_write).max(high_value_latencies_discard).max(high_value_latencies_flush);
    let high_value_queue_depth = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| (blockdevices.ios_weighted_time_spent_ms / 1000_f64) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_max_queue_size = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.queue_nr_requests * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    // For the TOTAL overview it doesn't make sense to use max_queue_size, because all devices
    // in it can have different max values. So it also doesn't make sense to show the max we find.
    let high_value_queue = if device_name != "TOTAL" {
        high_value_queue_depth.max(high_value_max_queue_size)
    } else {
        high_value_queue_depth
    };

    // create the plot
    multi_backend[2].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} latency and queue depth", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value_latencies)
        .unwrap()
        .set_secondary_coord(start_time..end_time, 0_f64..high_value_queue);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|latency| {
                 if latency == &0_f64    { format!("{:5.0} ms", latency) }
            else if latency  < &0.1_f64  { format!("{:5.3} ms", latency) }
            else if latency  < &10_f64   { format!("{:5.1} ms", latency) }
            else                         { format!("{:5.0} ms", latency) }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("queue depth")
        .y_label_formatter(&|depth| format!("{:5.0}", depth))
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write latency
    // this is a line graph, so no stacking.
    let min_write_latency = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
        .map(|blockdevice| if blockdevice.writes_completed_success == 0_f64 { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_write_latency = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if blockdevice.writes_completed_success == 0_f64 { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_writes_latency = if historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.writes_completed_success) == 0_f64 { 0_f64 } 
    else { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_time_spent_ms / historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_completed_success };
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write", min_write_latency, max_write_latency, latest_writes_latency))
        .legend(move |(x, y)| Circle::new((x , y), 4, RED.filled()));
    //
    // read latency
    // this is a line graph, so no stacking.
    let min_read_latency = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
        .map(|blockdevice| if blockdevice.reads_completed_success == 0_f64 { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_read_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if blockdevice.reads_completed_success == 0_f64 { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_reads_latency = if historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.reads_time_spent_ms) == 0_f64 { 0_f64 } 
    else { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_time_spent_ms / historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_completed_success };
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_latency, max_read_latency, latest_reads_latency))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    //
    // discard latency
    // this is a line graph, so no stacking.
    let min_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.discards_completed_success > 0_f64)
        .map(|blockdevice| if blockdevice.discards_completed_success == 0_f64 { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if blockdevice.discards_completed_success == 0_f64 { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_discard_latency = if historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.discards_completed_success) == 0_f64 { 0_f64 }
    else { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_time_spent_ms / historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_completed_success };
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.discards_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success), 2, PURPLE.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "discard", min_discard_latency, max_discard_latency, latest_discard_latency))
        .legend(move |(x, y)| Circle::new((x , y ), 2, PURPLE.filled()));
    //
    // flush latency
    // this is a line graph, so no stacking.
    let min_flush_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.flush_requests_completed_success > 0_f64)
        .map(|blockdevice| if blockdevice.flush_requests_completed_success == 0_f64 { 0_f64 } else { blockdevice.flush_requests_time_spent_ms / blockdevice.flush_requests_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_flush_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if blockdevice.flush_requests_completed_success == 0_f64 { 0_f64 } else { blockdevice.flush_requests_time_spent_ms / blockdevice.flush_requests_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_flush_latency = if historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.flush_requests_completed_success) == 0_f64 { 0_f64 } 
    else { historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().flush_requests_time_spent_ms / historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().flush_requests_completed_success };
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.flush_requests_completed_success > 0_f64)
                                                .map(|blockdevice| Cross::new((blockdevice.timestamp, blockdevice.flush_requests_time_spent_ms / blockdevice.flush_requests_completed_success), 3, ShapeStyle { color: BLUE.into(), filled: true, stroke_width: 2 })))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "flush", min_flush_latency, max_flush_latency, latest_flush_latency))
        .legend(move |(x, y)| Cross::new((x,y), 3, ShapeStyle { color: BLUE.into(), filled: true, stroke_width: 2 }));
    //
    // queue depth
    let min_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.ios_weighted_time_spent_ms > 0_f64)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_queue_depth = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.ios_weighted_time_spent_ms) / 1000_f64;
    contextarea.draw_secondary_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.ios_weighted_time_spent_ms > 0_f64)
                                                .map(|blockdevice| TriangleMarker::new((blockdevice.timestamp, blockdevice.ios_weighted_time_spent_ms / 1000_f64), 5, BLACK.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "queue depth", min_queue_depth, max_queue_depth, latest_queue_depth))
        .legend(move |(x, y)| TriangleMarker::new((x, y), 5, BLACK.filled()));
    //
    // inflight writes
    let min_inflight_writes = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.inflight_writes > 0_f64)
        .map(|blockdevice| blockdevice.inflight_writes)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_inflight_writes = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.inflight_writes)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_inflight_writes = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last()
        .map_or(0_f64, |latest| latest.inflight_writes);
    contextarea.draw_secondary_series(historical_data_read
            .iter()
            .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.inflight_writes > 0_f64)
            .map(|blockdevice| TriangleMarker::new((blockdevice.timestamp, blockdevice.inflight_writes), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "inflight writes", min_inflight_writes, max_inflight_writes, latest_inflight_writes))
        .legend(move |(x, y)| TriangleMarker::new((x, y), 4, RED.filled()));
    //
    // inflight reads
    let min_inflight_reads = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.inflight_reads > 0_f64)
        .map(|blockdevice| blockdevice.inflight_reads)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_inflight_reads = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.inflight_reads)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest_inflight_reads = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last()
        .map_or(0_f64, |latest| latest.inflight_reads);
    contextarea.draw_secondary_series(historical_data_read
            .iter()
            .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.inflight_reads > 0_f64)
            .map(|blockdevice| TriangleMarker::new((blockdevice.timestamp, blockdevice.inflight_reads), 3, GREEN_500.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "inflight reads", min_inflight_reads, max_inflight_reads, latest_inflight_reads))
        .legend(move |(x, y)| TriangleMarker::new((x, y), 3, GREEN_500.filled()));
    // max queue size
    // It wouldn't make sense to use total, because it combines different blockdevices.
    if device_name != "TOTAL" {
        let latest_queue_nr_requests = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().map_or(0_f64, |latest| latest.queue_nr_requests);
        contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.queue_nr_requests)), RED_100))
        .unwrap()
        .label(format!("{:25} {:10} {:10} {:10.2}", "nr_requests", "", "", latest_queue_nr_requests))
        .legend(|(x, y)| PathElement::new(vec![(x-3,y), (x+3,y)], RED_100.filled()));
    };
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

fn blockdevice_iosize_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap_or_default();
    let high_value_avg_read_size = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.reads_completed_success == 0_f64 { 0_f64 } else { ((blockdevices.reads_bytes / blockdevices.reads_completed_success) / 1024_f64) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_avg_write_size = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| if blockdevices.writes_completed_success == 0_f64 { 0_f64 } else { ((blockdevices.writes_bytes / blockdevices.writes_completed_success) / 1024_f64) * 1.1_f64 })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_max_io_size = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.queue_max_sectors_kb * 1.1_f64 )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let high_value_size = high_value_avg_read_size.max(high_value_avg_write_size).max(high_value_max_io_size);
    let latest = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last();

    // create the plot
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} IO size", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value_size)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_label_formatter(&|size| format!("{:5.0} kB", size))
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                                .take(1)
                                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    // write
    let min_write_size = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.writes_bytes / blockdevice.writes_completed_success / 1024_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_write_size = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.writes_bytes / blockdevice.writes_completed_success / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.writes_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, (blockdevice.writes_bytes / blockdevice.writes_completed_success) / 1024_f64), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write", min_write_size, max_write_size, if latest.map_or(0_f64, |latest| latest.writes_completed_success) == 0_f64 { 0_f64 } else { (latest.map_or(0_f64, |latest| latest.writes_bytes) / latest.map_or(0_f64, |latest| latest.writes_completed_success)) / 1024_f64 } ))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // read
    let min_read_size = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.reads_bytes / blockdevice.reads_completed_success / 1024_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_read_size = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
        .map(|blockdevice| blockdevice.reads_bytes / blockdevice.reads_completed_success / 1024_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name && blockdevice.reads_completed_success > 0_f64)
                                                .map(|blockdevice| Circle::new((blockdevice.timestamp, (blockdevice.reads_bytes / blockdevice.reads_completed_success) / 1024_f64), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_size, max_read_size, if latest.map_or(0_f64, |latest| latest.reads_completed_success) == 0_f64 { 0_f64 } else { (latest.map_or(0_f64, |latest| latest.reads_bytes) / latest.map_or(0_f64, |latest| latest.reads_completed_success)) / 1024_f64 } ))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // current max IO size line
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                            .filter(|blockdevice| blockdevice.device_name == device_name)
                                            .map(|blockdevice| (blockdevice.timestamp, blockdevice.queue_max_sectors_kb)), BLACK))
    .unwrap()
    .label(format!("{:25} {:10} {:10} {:10.2}", "max_sectors_kb", "", "", latest.map_or(0_f64, |latest| latest.queue_max_sectors_kb)))
    .legend(|(x, y)| PathElement::new(vec![(x-3,y), (x+3,y)], BLACK.filled()));
    // 
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                                .take(1)
                                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:10} {:10} {:10.2}", "max_hw_sectors_kb", "", "", latest.map_or(0_f64, |latest| latest.queue_max_hw_sectors_kb)));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}

fn blockdevice_extra(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    multi_backend[backend_number].fill(&GREY_100).unwrap();
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    // 
    // CAPTION_STYLE_FONT _SIZE 30
    // MESH_STYLE_FONT _SIZE    17
    // LABELS_STYLE_FONT _SIZE  15
    let latest = historical_data_read
        .iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .last();
    multi_backend[backend_number]
        .draw(&Text::new(format!("device:              {:>10}", device_name), (100,0), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("major:minor:         {:>10}", latest.map_or("".to_string(), |d| d.device_major_minor.clone())), (100,40), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("removable:           {:>10}", latest.map_or(0_f64, |d| d.removable)), (100,60), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("readonly:            {:>10}", latest.map_or(0_f64, |d| d.ro)), (100,80), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("rotational:          {:>10}", latest.map_or(0_f64, |d| d.queue_rotational)), (100,100), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("dax:                 {:>10}", latest.map_or(0_f64, |d| d.queue_dax)), (100,120), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("nr_requests:         {:>10}", latest.map_or(0_f64, |d| d.queue_nr_requests)), (100,140), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("max_sectors_kb:      {:>10}", latest.map_or(0_f64, |d| d.queue_max_sectors_kb)), (100,160), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("max_hw_sectors_kb:   {:>10}", latest.map_or(0_f64, |d| d.queue_max_hw_sectors_kb)), (100,180), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();

    multi_backend[backend_number]
        .draw(&Text::new(format!("hw_sector_size:      {:>10}", latest.map_or(0_f64, |d| d.queue_hw_sector_size)), (600,40), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("logical_block_size   {:>10}", latest.map_or(0_f64, |d| d.queue_logical_block_size)), (600,60), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("nomerges             {:>10}", latest.map_or(0_f64, |d| d.queue_nomerges)), (600,80), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("physical_block_size  {:>10}", latest.map_or(0_f64, |d| d.queue_physical_block_size)), (600,100), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("discard_max_bytes    {:>10}", latest.map_or(0_f64, |d| d.queue_discard_max_bytes)), (600,120), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("discard_max_hw_bytes {:>10}", latest.map_or(0_f64, |d| d.queue_discard_max_hw_bytes)), (600,140), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
    multi_backend[backend_number]
        .draw(&Text::new(format!("read_ahead_kb        {:>10}", latest.map_or(0_f64, |d| d.queue_read_ahead_kb)), (600,160), (MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE).into_font())).unwrap();
}
