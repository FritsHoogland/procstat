#![allow(unused_assignments)]

use crate::ARGS;
use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, DATA, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE,
};
use chrono::{DateTime, Local};
use log::debug;
use ordered_float::OrderedFloat;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::element::Rectangle;
use plotters::prelude::*;
use std::collections::BTreeSet;

pub fn create_networkdevice_plot(
    buffer: &mut [u8],
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    let mut mbit_graph = multi_backend[0].split_horizontally((60).percent_width());
    networkdevice_mbit_plot(&mut mbit_graph.0, device_name.clone(), start_time, end_time);
    networkdevice_mbit_percentile_plot(
        &mut mbit_graph.1,
        device_name.clone(),
        start_time,
        end_time,
    );
    let mut packet_graph = multi_backend[1].split_horizontally((60).percent_width());
    networkdevice_packet_plot(
        &mut packet_graph.0,
        device_name.clone(),
        start_time,
        end_time,
    );
    networkdevice_packet_percentile_plot(
        &mut packet_graph.1,
        device_name.clone(),
        start_time,
        end_time,
    );
    networkdevice_error_plot(&mut multi_backend, 2, device_name, start_time, end_time);
}

fn networkdevice_mbit_percentile_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    multi_backend.fill(&WHITE).unwrap();
    let historical_data_read = DATA.networkdevices.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .filter(|b| b.device_name == device_name)
            .map(|b| b.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .filter(|b| b.device_name == device_name)
            .map(|b| b.timestamp)
            .max()
            .unwrap_or_default()
    };

    let mut transmit_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut receive_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut total_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| (b.transmit_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .for_each(|t| {
            transmit_set.insert(OrderedFloat(t));
        });
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| (b.receive_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .for_each(|r| {
            receive_set.insert(OrderedFloat(r));
        });
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| ((b.transmit_bytes + b.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)
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
            format!("Networkdevice: {} Mbit per second percentiles", device_name),
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            0..total_set.len(),
            0_f64..total_set.last().unwrap_or(&OrderedFloat(0.)).into_inner() * 1.1,
        )
        .unwrap();
    contextarea
        .configure_mesh()
        //.x_labels(10)
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
        .y_desc("Mbit per second")
        .y_label_formatter(&|mbps| {
            if mbps == &0_f64 {
                format!("{:5.0}", mbps)
            } else if mbps < &1_f64 {
                format!("{:5.3}", mbps)
            } else {
                format!("{:5.0}", mbps)
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
            transmit_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, w)| (nr, w.into_inner())),
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "tx",
            transmit_set
                .last()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // min value
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            transmit_set.iter().map(|t| t.into_inner()).sum::<f64>() / transmit_set.len() as f64,
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            transmit_set
                .first()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    contextarea
        .draw_series(LineSeries::new(
            receive_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, r)| (nr, r.into_inner())),
            GREEN,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
            "rx",
            receive_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile / median
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            receive_set.iter().map(|t| t.into_inner()).sum::<f64>() / receive_set.len() as f64,
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            receive_set
                .first()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // max value
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
fn networkdevice_mbit_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.networkdevices.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .max()
            .unwrap_or_default()
    };
    let high_value_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| ((n.receive_bytes + n.transmit_bytes) / (1024_f64 * 1024_f64)) * 8_f64 * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name)
        .last();
    debug!(
        "mbit plot. start_time: {:?}, final_start_time {:?}",
        start_time, final_start_time
    );

    // create the plot
    multi_backend.fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            format!("Networkdevice: {} Megabit per second", device_name),
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value_mbit)
        .unwrap()
        .set_secondary_coord(
            final_start_time..final_end_time,
            0_f64..(high_value_mbit / 8_f64),
        );
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
        .y_desc("Megabit per second")
        .y_label_formatter(&|size| {
            if size == &0_f64 {
                format!("{:5.0}", size)
            } else if size < &10_f64 {
                format!("{:5.2}", size)
            } else {
                format!("{:5.0}", size)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea
        .configure_secondary_axes()
        .y_desc("Megabyte per second")
        .y_label_formatter(&|size| {
            if size == &0_f64 {
                format!("{:5.0}", size)
            } else if size < &1_f64 {
                format!("{:5.3}", size)
            } else {
                format!("{:5.0}", size)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
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
    //
    // total mbit
    let min_total_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && (n.transmit_bytes + n.receive_bytes) > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| ((n.transmit_bytes + n.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && (n.transmit_bytes + n.receive_bytes) > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| ((n.transmit_bytes + n.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| {
                    (
                        n.timestamp,
                        ((n.transmit_bytes + n.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64,
                    )
                }),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "total",
            min_total_mbit,
            max_total_mbit,
            (latest.map_or(0_f64, |l| l.transmit_bytes + l.receive_bytes) / (1024_f64 * 1024_f64)
                * 8_f64)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));

    // transmit mbit
    let min_transmit_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.transmit_bytes > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| (n.transmit_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_transmit_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.transmit_bytes > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| (n.transmit_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name && n.transmit_bytes > 0_f64)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| {
                    Circle::new(
                        (
                            n.timestamp,
                            n.transmit_bytes / (1024_f64 * 1024_f64) * 8_f64,
                        ),
                        2,
                        RED.filled(),
                    )
                }),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "transmit",
            min_transmit_mbit,
            max_transmit_mbit,
            latest.map_or(0_f64, |l| l.transmit_bytes / (1024_f64 * 1024_f64) * 8_f64)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // receive mbit
    let min_receive_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.receive_bytes > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| (n.receive_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_receive_mbit = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.receive_bytes > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| (n.receive_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name && n.receive_bytes > 0_f64)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| {
                    Circle::new(
                        (n.timestamp, n.receive_bytes / (1024_f64 * 1024_f64) * 8_f64),
                        1,
                        GREEN.filled(),
                    )
                }),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "receive",
            min_receive_mbit,
            max_receive_mbit,
            latest.map_or(0_f64, |l| l.receive_bytes / (1024_f64 * 1024_f64) * 8_f64)
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

fn networkdevice_packet_percentile_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    multi_backend.fill(&WHITE).unwrap();
    let historical_data_read = DATA.networkdevices.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .filter(|b| b.device_name == device_name)
            .map(|b| b.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .filter(|b| b.device_name == device_name)
            .map(|b| b.timestamp)
            .max()
            .unwrap_or_default()
    };

    let mut transmit_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut receive_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    let mut total_set: BTreeSet<OrderedFloat<f64>> = BTreeSet::new();
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.transmit_packets)
        .for_each(|t| {
            transmit_set.insert(OrderedFloat(t));
        });
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.receive_packets)
        .for_each(|r| {
            receive_set.insert(OrderedFloat(r));
        });
    historical_data_read
        .iter()
        .filter(|b| b.device_name == device_name)
        .filter(|b| b.timestamp >= final_start_time && b.timestamp <= final_end_time)
        .map(|b| b.transmit_packets + b.receive_packets)
        .for_each(|t| {
            total_set.insert(OrderedFloat(t));
        });
    let sample_interval =
        (final_end_time - final_start_time) / total_set.len().try_into().unwrap_or(1);
    // create the plot
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .caption(
            format!(
                "Networkdevice: {} packets per second percentiles",
                device_name
            ),
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
        .y_desc("Packets per second")
        .y_label_formatter(&|packets| format!("{:5.0}", packets))
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
            "{:6} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0}",
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
            transmit_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, w)| (nr, w.into_inner())),
            RED,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0}",
            "tx",
            transmit_set
                .last()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // min value
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            transmit_set.iter().map(|t| t.into_inner()).sum::<f64>() / transmit_set.len() as f64,
            transmit_set
                .iter()
                .nth((transmit_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            transmit_set
                .first()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // max value
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    contextarea
        .draw_series(LineSeries::new(
            receive_set
                .iter()
                .rev()
                .enumerate()
                .map(|(nr, r)| (nr, r.into_inner())),
            GREEN,
        ))
        .unwrap()
        .label(format!(
            "{:6} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0} {:9.0}",
            "rx",
            receive_set.last().unwrap_or(&OrderedFloat(0.)).into_inner(), // min value
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 99.9) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 99.9 percentile
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 75.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 75 percentile / median
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 50.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 50 percentile / median
            receive_set.iter().map(|t| t.into_inner()).sum::<f64>() / receive_set.len() as f64,
            receive_set
                .iter()
                .nth((receive_set.len() as f64 / 100_f64 * 25.) as usize)
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // 25 percentile / median
            receive_set
                .first()
                .unwrap_or(&OrderedFloat(0.))
                .into_inner(), // max value
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
fn networkdevice_packet_plot(
    multi_backend: &mut DrawingArea<BitMapBackend<RGBPixel>, Shift>,
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.networkdevices.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .max()
            .unwrap_or_default()
    };
    let high_value_packets = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| (n.receive_packets + n.transmit_packets) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name)
        .last();

    // create the plot
    multi_backend.fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(multi_backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        //.set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            format!("Networkdevice: {} packets per second", device_name),
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value_packets)
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
        .y_desc("Packets per second")
        .y_label_formatter(&|packets| format!("{:5.0}", packets))
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
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
    //
    // total packets
    let min_total_packets = historical_data_read
        .iter()
        .filter(|n| {
            n.device_name == device_name && (n.transmit_packets + n.receive_packets) > 0_f64
        })
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.transmit_packets + n.receive_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_packets = historical_data_read
        .iter()
        .filter(|n| {
            n.device_name == device_name && (n.transmit_packets + n.receive_packets) > 0_f64
        })
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.transmit_packets + n.receive_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| (n.timestamp, n.transmit_packets + n.receive_packets)),
            BLACK,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "total",
            min_total_packets,
            max_total_packets,
            latest.map_or(0_f64, |l| l.transmit_packets + l.receive_packets)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // transmit packets
    let min_transmit_packets = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.transmit_packets > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.transmit_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_transmit_packets = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.transmit_packets > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.transmit_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name && n.transmit_packets > 0_f64)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| Circle::new((n.timestamp, n.transmit_packets), 2, RED.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "transmit",
            min_transmit_packets,
            max_transmit_packets,
            latest.map_or(0_f64, |l| l.transmit_packets)
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));

    // receive packets
    let min_receive_packets = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.receive_packets > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.receive_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_receive_packets = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name && n.receive_packets > 0_f64)
        .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
        .map(|n| n.receive_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea
        .draw_series(
            historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name && n.receive_packets > 0_f64)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| Circle::new((n.timestamp, n.receive_packets), 1, GREEN.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "receive",
            min_receive_packets,
            max_receive_packets,
            latest.map_or(0_f64, |l| l.receive_packets)
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

fn networkdevice_error_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    #[derive(Debug, Default)]
    struct LowValue {
        pub receive_errors: f64,
        pub transmit_errors: f64,
        pub transmit_collisions: f64,
        pub receive_drop: f64,
        pub transmit_drop: f64,
        pub transmit_carrier: f64,
        pub receive_fifo: f64,
        pub transmit_fifo: f64,
    }
    #[derive(Debug, Default)]
    struct HighValue {
        pub receive_errors: f64,
        pub transmit_errors: f64,
        pub transmit_collisions: f64,
        pub receive_drop: f64,
        pub transmit_drop: f64,
        pub transmit_carrier: f64,
        pub receive_fifo: f64,
        pub transmit_fifo: f64,
    }
    let historical_data_read = DATA.networkdevices.read().unwrap();

    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .filter(|n| n.device_name == device_name)
            .map(|n| n.timestamp)
            .max()
            .unwrap_or_default()
    };
    let mut low_value: LowValue = Default::default();
    let mut high_value: HighValue = Default::default();
    macro_rules! read_history_and_set_high_and_low_values {
        ($($struct_field_name:ident),*) => {
            $(
            low_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| n.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|n| n.device_name == device_name)
                .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                .map(|n| n.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(
        receive_errors,
        transmit_errors,
        transmit_collisions,
        receive_drop,
        transmit_drop,
        transmit_carrier,
        receive_fifo,
        transmit_fifo
    );
    let high_value_overall = [
        high_value.receive_errors,
        high_value.transmit_errors,
        high_value.transmit_collisions,
        high_value.receive_drop,
        high_value.transmit_drop,
        high_value.transmit_carrier,
        high_value.receive_fifo,
        high_value.transmit_fifo,
    ]
    .iter()
    .max_by(|a, b| a.partial_cmp(b).unwrap())
    .copied()
    .unwrap();
    let latest = historical_data_read
        .iter()
        .filter(|n| n.device_name == device_name)
        .last();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .caption(
            format!("Networkdevice: {} errors", device_name),
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            final_start_time..final_end_time,
            0_f64..(high_value_overall * 1.1_f64),
        )
        .unwrap();
    contextarea
        .configure_mesh()
        .x_label_formatter(&|timestamp| timestamp.format("%H:%M:%S").to_string())
        .x_desc(format!(
            "Time: {} to {} ({} minutes)",
            final_start_time.format("%Y-%m-%d %H:%M:%S%:z"),
            final_end_time.format("%Y-%m-%d %H:%M:%S%:z"),
            (final_end_time - final_start_time).num_minutes(),
        ))
        .y_desc("Errors per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|blockdevice| (blockdevice.timestamp, blockdevice.transmit_bytes)),
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
    //
    let mut colour_picker = 3_usize;
    macro_rules! draw_lineseries {
        ($($struct_field_name:ident),*) => {
            $(
                contextarea.draw_series(historical_data_read.iter()
                                                            .filter(|n| n.device_name == device_name && n.$struct_field_name > 0_f64)
                                                            .filter(|n| n.timestamp >= final_start_time && n.timestamp <= final_end_time)
                                                            .map(|n| Circle::new((n.timestamp, n.$struct_field_name), 4, Palette99::pick(colour_picker).filled())))
                .unwrap()
                .label(format!("{:25} {:10.2} {:10.2} {:10.2}", stringify!($struct_field_name), low_value.$struct_field_name, high_value.$struct_field_name, latest.map_or(0_f64, |l| l.$struct_field_name)))
                .legend(move |(x, y)| Circle::new((x, y), 4, Palette99::pick(colour_picker).filled()));

                colour_picker += 1;
            )*
        };
    }
    draw_lineseries!(
        receive_errors,
        transmit_errors,
        transmit_collisions,
        receive_drop,
        transmit_drop,
        transmit_carrier,
        receive_fifo,
        transmit_fifo
    );
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
