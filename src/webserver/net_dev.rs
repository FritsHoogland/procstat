#![allow(unused_assignments)]

use crate::ARGS;
use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, DATA, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT,
    MESH_STYLE_FONT_SIZE,
};
use chrono::{DateTime, Local};
use log::debug;
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::element::Rectangle;
use plotters::prelude::*;

pub fn create_networkdevice_plot(
    buffer: &mut [u8],
    device_name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let backend = BitMapBackend::with_buffer(buffer, (ARGS.graph_width, ARGS.graph_height))
        .into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    networkdevice_mbit_plot(
        &mut multi_backend,
        0,
        device_name.clone(),
        start_time,
        end_time,
    );
    networkdevice_packet_plot(
        &mut multi_backend,
        1,
        device_name.clone(),
        start_time,
        end_time,
    );
    networkdevice_error_plot(&mut multi_backend, 2, device_name, start_time, end_time);
}

fn networkdevice_mbit_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
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
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
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
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
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
                        4,
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
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));
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
                        3,
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
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));

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
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
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
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            format!("Networkdevice: {} packets per second", device_name),
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value_packets)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Packets per second")
        .y_label_formatter(&|packets| format!("{:5.0}", packets))
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
                .map(|n| Circle::new((n.timestamp, n.transmit_packets), 4, RED.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "transmit",
            min_transmit_packets,
            max_transmit_packets,
            latest.map_or(0_f64, |l| l.transmit_packets)
        ))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));

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
                .map(|n| Circle::new((n.timestamp, n.receive_packets), 3, GREEN.filled())),
        )
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "receive",
            min_receive_packets,
            max_receive_packets,
            latest.map_or(0_f64, |l| l.receive_packets)
        ))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
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
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
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
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
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
