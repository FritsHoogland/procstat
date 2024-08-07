use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_500, GREY_A100};

use crate::{
    CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, DATA, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE,
    LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT,
    MESH_STYLE_FONT_SIZE,
};

#[derive(Debug, Default)]
struct LowValue {
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
}

#[derive(Debug, Default)]
struct HighValue {
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
}
pub fn load_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
) {
    let historical_data_read = DATA.loadavg.read().unwrap();
    let final_start_time = if let Some(final_start_time) = start_time {
        final_start_time
    } else {
        historical_data_read
            .iter()
            .map(|l| l.timestamp)
            .min()
            .unwrap_or_default()
    };
    let final_end_time = if let Some(final_end_time) = end_time {
        final_end_time
    } else {
        historical_data_read
            .iter()
            .map(|l| l.timestamp)
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
                .filter(|l| l.timestamp >= final_start_time && l.timestamp <= final_end_time)
                .map(|l| l.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|l| l.timestamp >= final_start_time && l.timestamp <= final_end_time)
                .map(|l| l.$struct_field_name * 1.1_f64)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_default();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(load_1, load_5, load_15);
    let high_value_all_load = high_value
        .load_1
        .max(high_value.load_5)
        .max(high_value.load_15);
    let latest = historical_data_read.back().unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Load", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(final_start_time..final_end_time, 0_f64..high_value_all_load)
        .unwrap();
    contextarea
        .configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S%z").to_string())
        .x_desc("Time")
        .y_desc("Load")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea
        .draw_series(LineSeries::new(
            historical_data_read
                .iter()
                .take(1)
                .map(|l| (l.timestamp, l.load_1)),
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
    macro_rules! draw_lineseries {
        ($([$struct_field_name:ident, $color:expr]),*) => {
            $(
                contextarea.draw_series(
                    LineSeries::new(
                        historical_data_read
                            .iter()
                            .filter(|l| l.timestamp >= final_start_time && l.timestamp <= final_end_time)
                            .map(|l| (l.timestamp, l.$struct_field_name)),
                        ShapeStyle { color: $color.into(), filled: true, stroke_width: 2 }
                    )
                )
                .unwrap()
                .label(format!("{:25} {:10.2} {:10.2} {:10.2}", stringify!($struct_field_name), low_value.$struct_field_name, high_value.$struct_field_name, latest.$struct_field_name))
                .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], $color.filled()));
            )*
        };
    }
    draw_lineseries!([load_1, BLACK], [load_5, GREY_500], [load_15, GREY_A100]);
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
