use crate::ARGS;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
//use plotters::element::Rectangle;
use plotters::prelude::*;
//use plotters::style::full_palette::{GREY_500, GREY_A100};

use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABELS_STYLE_FONT,
    LABELS_STYLE_FONT_SIZE, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT,
    MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE,
};

pub fn create_xfs_plot(buffer: &mut [u8]) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((2, 1));
    xfs_iops_plot(&mut multi_backend, 0);
    xfs_mbps_plot(&mut multi_backend, 1);
}

pub fn xfs_iops_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.xfs.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|r| r.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .map(|r| r.timestamp)
        .max()
        .unwrap_or_default();
    let high_value = historical_data_read
        .iter()
        .map(|r| (r.xs_read_calls + r.xs_write_calls) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("XFS IOPS", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("IOPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|r| (r.timestamp, r.xs_read_calls)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min", "max", "last"
        ));

    let min_total_calls = historical_data_read
        .iter()
        .filter(|r| r.xs_read_calls + r.xs_write_calls > 0_f64)
        .map(|r| r.xs_read_calls + r.xs_write_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_calls = historical_data_read
        .iter()
        .map(|r| r.xs_read_calls + r.xs_write_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .map(|r| (r.timestamp, r.xs_read_calls + r.xs_write_calls)),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "total",
            min_total_calls,
            max_total_calls,
            (latest.xs_read_calls + latest.xs_write_calls)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // write calls
    let min_xfs_write_calls = historical_data_read
        .iter()
        .filter(|r| r.xs_write_calls > 0_f64)
        .map(|r| r.xs_write_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_write_calls = historical_data_read
        .iter()
        .map(|r| r.xs_write_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|r| r.xs_write_calls > 0_f64)
                .map(|r| Circle::new((r.timestamp, r.xs_write_calls), 4, RED.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "write", min_xfs_write_calls, max_xfs_write_calls, latest.xs_write_calls
        ))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // read calls
    let min_xfs_read_calls = historical_data_read
        .iter()
        .filter(|r| r.xs_read_calls > 0_f64)
        .map(|r| r.xs_read_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_read_calls = historical_data_read
        .iter()
        .map(|r| r.xs_read_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|r| r.xs_read_calls > 0_f64)
                .map(|r| Circle::new((r.timestamp, r.xs_read_calls), 3, GREEN.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "read", min_xfs_read_calls, max_xfs_read_calls, latest.xs_read_calls
        ))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));

    // draw the legend
    contextarea
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
pub fn xfs_mbps_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
) {
    let historical_data_read = HISTORY.xfs.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|r| r.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .map(|r| r.timestamp)
        .max()
        .unwrap_or_default();
    let high_value = historical_data_read
        .iter()
        .map(|r| ((r.xs_read_bytes + r.xs_write_bytes) / (1024_f64 * 1024_f64)) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("XFS MBPS", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("MBPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|r| (r.timestamp, r.xs_read_bytes)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:25} {:>10} {:>10} {:>10}",
            "", "min", "max", "last"
        ));

    let min_total_mbps = historical_data_read
        .iter()
        .filter(|r| r.xs_read_bytes + r.xs_write_bytes > 0_f64)
        .map(|r| (r.xs_read_bytes + r.xs_write_bytes) / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_mbps = historical_data_read
        .iter()
        .map(|r| (r.xs_read_bytes + r.xs_write_bytes) / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read.iter().map(|r| {
                (
                    r.timestamp,
                    (r.xs_read_bytes + r.xs_write_bytes) / (1024_f64 * 1024_f64),
                )
            }),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "total",
            min_total_mbps,
            max_total_mbps,
            ((latest.xs_read_calls + latest.xs_write_calls) / (1024_f64 * 1024_f64))
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // write bytes
    let min_xfs_write_mbps = historical_data_read
        .iter()
        .filter(|r| r.xs_write_bytes > 0_f64)
        .map(|r| r.xs_write_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_write_mbps = historical_data_read
        .iter()
        .filter(|r| r.xs_write_bytes > 0_f64)
        .map(|r| r.xs_write_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|r| r.xs_write_bytes > 0_f64)
                .map(|r| {
                    Circle::new(
                        (r.timestamp, r.xs_write_bytes / (1024_f64 * 1024_f64)),
                        4,
                        RED.filled(),
                    )
                }),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "write",
            min_xfs_write_mbps,
            max_xfs_write_mbps,
            latest.xs_write_bytes / (1024_f64 * 1024_f64)
        ))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
    // read bytes
    let min_xfs_read_bytes = historical_data_read
        .iter()
        .filter(|r| r.xs_read_bytes > 0_f64)
        .map(|r| r.xs_read_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_read_bytes = historical_data_read
        .iter()
        .filter(|r| r.xs_read_bytes > 0_f64)
        .map(|r| r.xs_read_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|r| r.xs_read_bytes > 0_f64)
                .map(|r| {
                    Circle::new(
                        (r.timestamp, r.xs_read_bytes / (1024_f64 * 1024_f64)),
                        3,
                        GREEN.filled(),
                    )
                }),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "read",
            min_xfs_read_bytes,
            max_xfs_read_bytes,
            latest.xs_read_bytes / (1024_f64 * 1024_f64)
        ))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));

    // draw the legend
    contextarea
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
