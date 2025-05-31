use crate::ARGS;
use chrono::{DateTime, Local};
use ordered_float::OrderedFloat;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::prelude::*;
use std::collections::BTreeSet;

use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, DATA, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE,
};

pub fn create_xfs_plot(
    buffer: &mut [u8],
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let multi_backend = backend.split_evenly((2, 1));
    let mut mbps_graph = multi_backend[0].split_horizontally((60).percent_width());
    xfs_mbps_plot(&mut mbps_graph.0, start_time, end_time);
    xfs_mbps_percentile_plot(&mut mbps_graph.1, start_time, end_time);
    let mut iops_graph = multi_backend[1].split_horizontally((60).percent_width());
    xfs_iops_plot(&mut iops_graph.0, start_time, end_time);
    xfs_iops_percentile_plot(&mut iops_graph.1, start_time, end_time);
}

fn xfs_iops_percentile_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    multi_backend.fill(&WHITE).unwrap();
    let historical_data_read = DATA.xfs.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|b| b.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|b| b.timestamp)
            .max()
            .unwrap_or_default()
    };

    let mut writes_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut reads_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut total_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.xs_write_calls)
        .for_each(|w| {
            writes_set.insert(OrderedFloat(w));
        });
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.xs_read_calls)
        .for_each(|r| {
            reads_set.insert(OrderedFloat(r));
        });
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.xs_write_calls + b.xs_read_calls)
        .for_each(|t| {
            total_set.insert(OrderedFloat(t));
        });
    let sample_interval =
        (final_end_time - final_start_time) / total_set.len().try_into().unwrap_or(1);
    // create the plot
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "XFS IOPS percentiles",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            0..total_set.len(),
            0_f64..total_set.last().unwrap_or(&OrderedFloat(0.)).into_inner() * 1.1,
        )
        .unwrap();
    contextarea
        .configure_mesh()
        .x_label_formatter(&|sample_number| {
            format!(
                "{:3.0}",
                (total_set.len() - sample_number).as_f64() / total_set.len().as_f64() * 100.
            )
        })
        .x_desc(format!(
            "Percentile (avg sample rate {}s)",
            sample_interval.num_seconds()
        ))
        .x_labels(12)
        .y_desc("IOPS")
        .y_label_formatter(&|iops| {
            if iops == &0_f64 {
                format!("{:5.0}", iops)
            } else if iops < &10_f64 {
                format!("{:5.1}", iops)
            } else {
                format!("{:5.0}", iops)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((0, 0_f64)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:6} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
            "ptile", "max", "99.9", "75", "50", "avg", "25", "min"
        ));
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((0, 0_f64)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:6} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0}",
            "nr",
            0,
            total_set.len() as f64 / 100_f64 * 0.1,
            total_set.len() as f64 / 100_f64 * 25.,
            total_set.len() as f64 / 100_f64 * 50.,
            "-",
            total_set.len() as f64 / 100_f64 * 75.,
            total_set.len()
        ));
    contextarea
        .draw_series(LineSeries::new(
            total_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, t)| (nr, t.into_inner())),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "total",
            total_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            total_set.iter().map(|t| t.into_inner()).sum::<f64>() / total_set.len() as f64,
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile
            total_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    contextarea
        .draw_series(LineSeries::new(
            writes_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, w)| (nr, w.into_inner())),
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "write",
            writes_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            writes_set.iter().map(|t| t.into_inner()).sum::<f64>() / writes_set.len() as f64,
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            writes_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    contextarea
        .draw_series(LineSeries::new(
            reads_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, r)| (nr, r.into_inner())),
            GREEN,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "reads",
            reads_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile / median
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            reads_set.iter().map(|t| t.into_inner()).sum::<f64>() / reads_set.len() as f64,
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            reads_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // legend
    contextarea
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
pub fn xfs_iops_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.xfs.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|x| x.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|x| x.timestamp)
            .max()
            .unwrap_or_default()
    };
    let high_value = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| (x.xs_read_calls + x.xs_write_calls) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend.fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("XFS IOPS", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        //.x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%H:%M:%S").to_string())
        .x_desc(format!(
            "Time: {} to {} ({} minutes)",
            final_start_time.format("%Y-%m-%d %H:%M:%S%:z"),
            final_end_time.format("%Y-%m-%d %H:%M:%S%:z"),
            (final_end_time - final_start_time).num_minutes(),
        ))
        .y_desc("IOPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((Local::now(), 0_f64)),
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
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_read_calls + x.xs_write_calls > 0_f64)
        .map(|x| x.xs_read_calls + x.xs_write_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_calls = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| x.xs_read_calls + x.xs_write_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .map(|x| (x.timestamp, x.xs_read_calls + x.xs_write_calls)),
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
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_write_calls > 0_f64)
        .map(|x| x.xs_write_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_write_calls = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| x.xs_write_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .filter(|x| x.xs_write_calls > 0_f64)
                .map(|x| Circle::new((x.timestamp, x.xs_write_calls), 2, RED.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "write", min_xfs_write_calls, max_xfs_write_calls, latest.xs_write_calls
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read calls
    let min_xfs_read_calls = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_read_calls > 0_f64)
        .map(|x| x.xs_read_calls)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_read_calls = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| x.xs_read_calls)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .filter(|x| x.xs_read_calls > 0_f64)
                .map(|x| Circle::new((x.timestamp, x.xs_read_calls), 1, GREEN.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "read", min_xfs_read_calls, max_xfs_read_calls, latest.xs_read_calls
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));

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
fn xfs_mbps_percentile_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    multi_backend.fill(&WHITE).unwrap();
    let historical_data_read = DATA.xfs.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|b| b.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|b| b.timestamp)
            .max()
            .unwrap_or_default()
    };

    let mut writes_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut reads_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut total_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.xs_write_bytes / (1024_f64 * 1024_f64))
        .for_each(|w| {
            writes_set.insert(OrderedFloat(w));
        });
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.xs_read_bytes / (1024_f64 * 1024_f64))
        .for_each(|r| {
            reads_set.insert(OrderedFloat(r));
        });
    historical_data_read
        .iter()
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| (b.xs_write_bytes + b.xs_read_bytes) / (1024_f64 * 1024_f64))
        .for_each(|t| {
            total_set.insert(OrderedFloat(t));
        });
    let sample_interval =
        (final_end_time - final_start_time) / total_set.len().try_into().unwrap_or(1);
    // create the plot
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "XFS MBPS percentiles",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            0..total_set.len(),
            0_f64..total_set.last().unwrap_or(&OrderedFloat(0.)).into_inner() * 1.1,
        )
        .unwrap();
    contextarea
        .configure_mesh()
        .x_label_formatter(&|sample_number| {
            format!(
                "{:3.0}",
                (total_set.len() - sample_number).as_f64() / total_set.len().as_f64() * 100.
            )
        })
        .x_desc(format!(
            "Percentile (avg sample rate {}s)",
            sample_interval.num_seconds()
        ))
        .x_labels(12)
        .y_desc("MBPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((0, 0_f64)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:6} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
            "ptile", "max", "99.9", "75", "50", "avg", "25", "min"
        ));
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((0, 0_f64)),
            ShapeStyle {
                color: TRANSPARENT,
                filled: false,
                stroke_width: 1,
            },
        ))
        .unwrap()
        .label(format!(
            "{:6} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0} {:>9.0}",
            "nr",
            0,
            total_set.len() as f64 / 100_f64 * 0.1,
            total_set.len() as f64 / 100_f64 * 25.,
            total_set.len() as f64 / 100_f64 * 50.,
            "-",
            total_set.len() as f64 / 100_f64 * 75.,
            total_set.len()
        ));
    contextarea
        .draw_series(LineSeries::new(
            total_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, t)| (nr, t.into_inner())),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "total",
            total_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            total_set.iter().map(|t| t.into_inner()).sum::<f64>() / total_set.len() as f64,
            total_set
                .iter()
                .nth((total_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile
            total_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    contextarea
        .draw_series(LineSeries::new(
            writes_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, w)| (nr, w.into_inner())),
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "write",
            writes_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            writes_set.iter().map(|t| t.into_inner()).sum::<f64>() / writes_set.len() as f64,
            writes_set
                .iter()
                .nth((writes_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            writes_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    contextarea
        .draw_series(LineSeries::new(
            reads_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, r)| (nr, r.into_inner())),
            GREEN,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "reads",
            reads_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile / median
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            reads_set.iter().map(|t| t.into_inner()).sum::<f64>() / reads_set.len() as f64,
            reads_set
                .iter()
                .nth((reads_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            reads_set.first().unwrap_or(&OrderedFloat(0.)).into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // legend
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
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.xfs.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|x| x.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|x| x.timestamp)
            .max()
            .unwrap_or_default()
    };
    let high_value = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| ((x.xs_read_bytes + x.xs_write_bytes) / (1024_f64 * 1024_f64)) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend.fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("XFS MBPS", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value)
        .unwrap();
    contextarea
        .configure_mesh()
        //.x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%H:%M:%S").to_string())
        .x_desc(format!(
            "Time: {} to {} ({} minutes)",
            final_start_time.format("%Y-%m-%d %H:%M:%S%:z"),
            final_end_time.format("%Y-%m-%d %H:%M:%S%:z"),
            (final_end_time - final_start_time).num_minutes(),
        ))
        .y_desc("MBPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            std::iter::once((Local::now(), 0_f64)),
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
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_read_bytes + x.xs_write_bytes > 0_f64)
        .map(|x| (x.xs_read_bytes + x.xs_write_bytes) / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_mbps = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .map(|x| (x.xs_read_bytes + x.xs_write_bytes) / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .map(|x| {
                    (
                        x.timestamp,
                        (x.xs_read_bytes + x.xs_write_bytes) / (1024_f64 * 1024_f64),
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
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_write_bytes > 0_f64)
        .map(|x| x.xs_write_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_write_mbps = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_write_bytes > 0_f64)
        .map(|x| x.xs_write_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .filter(|x| x.xs_write_bytes > 0_f64)
                .map(|x| {
                    Circle::new(
                        (x.timestamp, x.xs_write_bytes / (1024_f64 * 1024_f64)),
                        2,
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
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read bytes
    let min_xfs_read_bytes = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_read_bytes > 0_f64)
        .map(|x| x.xs_read_bytes / (1024_f64 * 1024_f64))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_xfs_read_bytes = historical_data_read
        .iter()
        .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
        .filter(|x| x.xs_read_bytes > 0_f64)
        .map(|x| x.xs_read_bytes / (1024_f64 * 1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|x| x.timestamp >= final_start_time && x.timestamp <= final_end_time)
                .filter(|x| x.xs_read_bytes > 0_f64)
                .map(|x| {
                    Circle::new(
                        (x.timestamp, x.xs_read_bytes / (1024_f64 * 1024_f64)),
                        1,
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
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));

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
