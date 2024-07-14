use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, DATA, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT,
    MESH_STYLE_FONT_SIZE,
};
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::full_palette::{BLUE_A100, RED_A100};
use plotters::prelude::*;
use plotters::style::full_palette::{BLUE_100, BLUE_500, BLUE_900, RED_200, RED_900, RED_A400};

#[derive(Debug, Default)]
struct LowValue {
    pub cpu_some_avg10: f64,
    pub cpu_some_avg60: f64,
    pub cpu_some_avg300: f64,
    pub cpu_some_total: f64,
    pub cpu_full_avg10: f64,
    pub cpu_full_avg60: f64,
    pub cpu_full_avg300: f64,
    pub cpu_full_total: f64,
    pub memory_some_avg10: f64,
    pub memory_some_avg60: f64,
    pub memory_some_avg300: f64,
    pub memory_some_total: f64,
    pub memory_full_avg10: f64,
    pub memory_full_avg60: f64,
    pub memory_full_avg300: f64,
    pub memory_full_total: f64,
    pub io_some_avg10: f64,
    pub io_some_avg60: f64,
    pub io_some_avg300: f64,
    pub io_some_total: f64,
    pub io_full_avg10: f64,
    pub io_full_avg60: f64,
    pub io_full_avg300: f64,
    pub io_full_total: f64,
}

#[derive(Debug, Default)]
struct HighValue {
    pub cpu_some_avg10: f64,
    pub cpu_some_avg60: f64,
    pub cpu_some_avg300: f64,
    pub cpu_some_total: f64,
    pub cpu_full_avg10: f64,
    pub cpu_full_avg60: f64,
    pub cpu_full_avg300: f64,
    pub cpu_full_total: f64,
    pub memory_some_avg10: f64,
    pub memory_some_avg60: f64,
    pub memory_some_avg300: f64,
    pub memory_some_total: f64,
    pub memory_full_avg10: f64,
    pub memory_full_avg60: f64,
    pub memory_full_avg300: f64,
    pub memory_full_total: f64,
    pub io_some_avg10: f64,
    pub io_some_avg60: f64,
    pub io_some_avg300: f64,
    pub io_some_total: f64,
    pub io_full_avg10: f64,
    pub io_full_avg60: f64,
    pub io_full_avg300: f64,
    pub io_full_total: f64,
}

pub fn pressure_cpu_some_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.pressure.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|p| p.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|p| p.timestamp)
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
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(
        cpu_some_avg10,
        cpu_some_avg60,
        cpu_some_avg300,
        cpu_some_total,
        cpu_full_avg10,
        cpu_full_avg60,
        cpu_full_avg300,
        cpu_full_total
    );
    low_value.cpu_some_total /= 1_000_000_f64;
    high_value.cpu_some_total /= 1_000_000_f64;
    low_value.cpu_full_total /= 1_000_000_f64;
    high_value.cpu_full_total /= 1_000_000_f64;

    let high_value_all_avg = high_value
        .cpu_some_avg10
        .max(high_value.cpu_some_avg60)
        .max(high_value.cpu_some_avg300);
    let latest = historical_data_read.back();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "Pressure stall CPU",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            final_start_time..final_end_time,
            0_f64..(high_value.cpu_some_total * 1.1_f64),
        )
        .unwrap()
        .set_secondary_coord(
            final_start_time..final_end_time,
            0_f64..(high_value_all_avg * 1.1_f64),
        );
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .y_label_formatter(&|seconds| {
            if seconds == &0_f64 {
                format!("{:5.0} s", seconds)
            } else if seconds < &0.01_f64 {
                format!("{:5.5} s", seconds)
            } else if seconds < &0.1_f64 {
                format!("{:5.3} s", seconds)
            } else if seconds < &1_f64 {
                format!("{:5.1} s", seconds)
            } else {
                format!("{:5.0} s", seconds)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea
        .configure_secondary_axes()
        .y_desc("Percent")
        .y_label_formatter(&|percent| {
            if percent == &0_f64 {
                format!("{:5.0} %", percent)
            } else if percent < &10_f64 {
                format!("{:5.1} %", percent)
            } else {
                format!("{:5.0} %", percent)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|p| (p.timestamp, p.cpu_some_total)),
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
    //let latest_cpu_some_total = latest.map_or(0_f64, |latest| latest.cpu_some_total);
    // total
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| (p.timestamp, p.cpu_some_total / 1_000_000_f64)),
            0.0,
            BLUE_A100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "cpu_some_total",
            low_value.cpu_some_total,
            high_value.cpu_some_total,
            latest.map_or(0_f64, |latest| latest.cpu_some_total) / 1_000_000_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    macro_rules! draw_lineseries_on_secondary_axes {
        ($([$struct_field_name:ident, $color:expr]),*) => {
            $(
                contextarea.draw_secondary_series(
                    LineSeries::new(
                        historical_data_read
                            .iter()
                            .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                            .map(|p| (p.timestamp, p.$struct_field_name)),
                        ShapeStyle { color: $color.into(), filled: true, stroke_width: 2 }
                    )
                )
                .unwrap()
                .label(format!("{:25} {:10.2} {:10.2} {:10.2}", concat!(stringify!($struct_field_name), " %"), low_value.$struct_field_name, high_value.$struct_field_name, latest.map_or(0_f64, |latest| latest.$struct_field_name)))
                .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], $color.filled()));
            )*
        };
    }
    draw_lineseries_on_secondary_axes!(
        [cpu_some_avg10, BLUE_900],
        [cpu_some_avg60, BLUE_500],
        [cpu_some_avg300, BLUE_100]
    );

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
pub fn pressure_memory_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.pressure.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|pressure| pressure.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|pressure| pressure.timestamp)
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
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(
        memory_some_avg10,
        memory_some_avg60,
        memory_some_avg300,
        memory_some_total,
        memory_full_avg10,
        memory_full_avg60,
        memory_full_avg300,
        memory_full_total
    );
    low_value.memory_some_total /= 1_000_000_f64;
    high_value.memory_some_total /= 1_000_000_f64;
    low_value.memory_full_total /= 1_000_000_f64;
    high_value.memory_full_total /= 1_000_000_f64;

    let high_value_all_avg = [
        high_value.memory_some_avg10,
        high_value.memory_some_avg60,
        high_value.memory_some_avg300,
        high_value.memory_full_avg10,
        high_value.memory_full_avg60,
        high_value.memory_full_avg300,
    ]
    .into_iter()
    .max_by(|a, b| a.partial_cmp(b).unwrap())
    .unwrap();
    let high_value_all_total = high_value
        .memory_some_total
        .max(high_value.memory_full_total);
    let latest = historical_data_read.back();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "Pressure stall memory",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            final_start_time..final_end_time,
            0_f64..(high_value_all_total * 1.1_f64),
        )
        .unwrap()
        .set_secondary_coord(
            final_start_time..final_end_time,
            0_f64..(high_value_all_avg * 1.1_f64),
        );
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .y_label_formatter(&|seconds| {
            if seconds == &0_f64 {
                format!("{:5.0} s", seconds)
            } else if seconds < &0.01_f64 {
                format!("{:5.5} s", seconds)
            } else if seconds < &0.1_f64 {
                format!("{:5.3} s", seconds)
            } else if seconds < &1_f64 {
                format!("{:5.1} s", seconds)
            } else {
                format!("{:5.0} s", seconds)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea
        .configure_secondary_axes()
        .y_desc("Percent")
        .y_label_formatter(&|percent| {
            if percent == &0_f64 {
                format!("{:5.0} %", percent)
            } else if percent < &10_f64 {
                format!("{:5.1} %", percent)
            } else {
                format!("{:5.0} %", percent)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|p| (p.timestamp, p.memory_some_total)),
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
    // some total
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| (p.timestamp, p.memory_some_total / 1_000_000_f64)),
            0.0,
            BLUE_A100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memory_some_total",
            low_value.memory_some_total,
            high_value.memory_some_total,
            latest.map_or(0_f64, |latest| latest.memory_some_total) / 1_000_000_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    macro_rules! draw_lineseries_on_secondary_axes {
        ($([$struct_field_name:ident, $color:expr]),*) => {
            $(
                contextarea.draw_secondary_series(
                    LineSeries::new(
                        historical_data_read
                            .iter()
                            .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                            .map(|p| (p.timestamp, p.$struct_field_name)),
                        ShapeStyle { color: $color.into(), filled: true, stroke_width: 2 }
                    )
                )
                .unwrap()
                .label(format!("{:25} {:10.2} {:10.2} {:10.2}", concat!(stringify!($struct_field_name), " secs %"), low_value.$struct_field_name, high_value.$struct_field_name, latest.map_or(0_f64, |latest| latest.$struct_field_name)))
                .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], $color.filled()));
            )*
        };
    }
    draw_lineseries_on_secondary_axes!(
        [memory_some_avg10, BLUE_900],
        [memory_some_avg60, BLUE_500],
        [memory_some_avg300, BLUE_100]
    );
    // full total
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| (p.timestamp, p.memory_full_total / 1_000_000_f64)),
            0.0,
            RED_A100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "memory_full_total",
            low_value.memory_full_total,
            high_value.memory_full_total,
            latest.map_or(0_f64, |latest| latest.memory_full_total) / 1_000_000_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED_A100.filled()));

    draw_lineseries_on_secondary_axes!(
        [memory_full_avg10, RED_900],
        [memory_full_avg60, RED_A400],
        [memory_full_avg300, RED_200]
    );
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

pub fn pressure_io_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.pressure.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|p| p.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|pressure| pressure.timestamp)
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
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| p.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(
        io_some_avg10,
        io_some_avg60,
        io_some_avg300,
        io_some_total,
        io_full_avg10,
        io_full_avg60,
        io_full_avg300,
        io_full_total
    );
    low_value.io_some_total /= 1_000_000_f64;
    high_value.io_some_total /= 1_000_000_f64;
    low_value.io_full_total /= 1_000_000_f64;
    high_value.io_full_total /= 1_000_000_f64;

    let high_value_all_avg = [
        high_value.io_some_avg10,
        high_value.io_some_avg60,
        high_value.io_some_avg300,
        high_value.io_full_avg10,
        high_value.io_full_avg60,
        high_value.io_full_avg300,
    ]
    .into_iter()
    .max_by(|a, b| a.partial_cmp(b).unwrap())
    .unwrap();
    let high_value_all_total = high_value.io_some_total.max(high_value.io_full_total);
    let latest = historical_data_read.back();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(
            "Pressure stall io",
            (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE),
        )
        .build_cartesian_2d(
            final_start_time..final_end_time,
            0_f64..(high_value_all_total * 1.1_f64),
        )
        .unwrap()
        .set_secondary_coord(
            final_start_time..final_end_time,
            0_f64..(high_value_all_avg * 1.1_f64),
        );
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .y_label_formatter(&|seconds| {
            if seconds == &0_f64 {
                format!("{:5.0} s", seconds)
            } else if seconds < &0.01_f64 {
                format!("{:5.5} s", seconds)
            } else if seconds < &0.1_f64 {
                format!("{:5.3} s", seconds)
            } else if seconds < &1_f64 {
                format!("{:5.1} s", seconds)
            } else {
                format!("{:5.0} s", seconds)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea
        .configure_secondary_axes()
        .y_label_formatter(&|percent| {
            if percent == &0_f64 {
                format!("{:5.0} %", percent)
            } else if percent < &10_f64 {
                format!("{:5.1} %", percent)
            } else {
                format!("{:5.0} %", percent)
            }
        })
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|p| (p.timestamp, p.io_some_total)),
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
    // some total
    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| (p.timestamp, p.io_some_total / 1_000_000_f64)),
            0.0,
            BLUE_A100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "io_some_total",
            low_value.io_some_total,
            high_value.io_some_total,
            latest.map_or(0_f64, |latest| latest.io_some_total) / 1_000_000_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    macro_rules! draw_lineseries_on_secondary_axes {
        ($([$struct_field_name:ident, $color:expr]),*) => {
            $(
                contextarea
                    .draw_secondary_series(
                        LineSeries::new(
                            historical_data_read
                                .iter()
                                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                                .map(|p| (p.timestamp, p.$struct_field_name)),
                            ShapeStyle { color: $color.into(), filled: true, stroke_width: 2 }
                        )
                    )
                    .unwrap()
                    .label(format!("{:25} {:10.2} {:10.2} {:10.2}", concat!(stringify!($struct_field_name), " %"), low_value.$struct_field_name, high_value.$struct_field_name, latest.map_or(0_f64, |latest| latest.$struct_field_name)))
                    .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], $color.filled()));
            )*
        };
    }
    draw_lineseries_on_secondary_axes!(
        [io_some_avg10, BLUE_900],
        [io_some_avg60, BLUE_500],
        [io_some_avg300, BLUE_100]
    );

    contextarea
        .draw_series(AreaSeries::new(
            historical_data_read
                .iter()
                .filter(|p| p.timestamp >= final_start_time && p.timestamp <= final_end_time)
                .map(|p| (p.timestamp, p.io_full_total / 1_000_000_f64)),
            0.0,
            RED_A100,
        ))
        .unwrap()
        .label(format!(
            "{:25} {:10.2} {:10.2} {:10.2}",
            "io_full_total",
            low_value.io_full_total,
            high_value.io_full_total,
            latest.map_or(0_f64, |latest| latest.io_full_total) / 1_000_000_f64
        ))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED_A100.filled()));

    draw_lineseries_on_secondary_axes!(
        [io_full_avg10, RED_900],
        [io_full_avg60, RED_A400],
        [io_full_avg300, RED_200]
    );

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
