#![allow(unused_assignments)]
use std::collections::HashMap;
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_A100, GREY_500};
use crate::common::{ProcData, Statistic, single_statistic_u64, single_statistic_f64};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};

#[derive(Debug)]
pub struct LoadavgInfo {
    pub timestamp: DateTime<Local>,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
    pub current_runnable: f64,
    pub total: f64,
    pub last_pid: f64,
}

pub async fn process_loadavg_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    single_statistic_f64("loadavg", "","load_1", proc_data.timestamp, proc_data.loadavg.load_1, statistics).await;
    single_statistic_f64("loadavg", "","load_5", proc_data.timestamp, proc_data.loadavg.load_5, statistics).await;
    single_statistic_f64("loadavg", "","load_15", proc_data.timestamp, proc_data.loadavg.load_15, statistics).await;
    single_statistic_u64("loadavg", "","current_runnable", proc_data.timestamp, proc_data.loadavg.current_runnable, statistics).await;
    single_statistic_u64("loadavg", "","total", proc_data.timestamp, proc_data.loadavg.total, statistics).await;
    single_statistic_u64("loadavg", "","last_pid", proc_data.timestamp, proc_data.loadavg.last_pid, statistics).await;
}

pub async fn add_loadavg_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().last_timestamp;
    let load_1 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_1".to_string())).unwrap().last_value;
    let load_5 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_5".to_string())).unwrap().last_value;
    let load_15 = statistics.get(&("loadavg".to_string(), "".to_string(), "load_15".to_string())).unwrap().last_value;
    let current_runnable = statistics.get(&("loadavg".to_string(), "".to_string(), "current_runnable".to_string())).unwrap().last_value;
    let total = statistics.get(&("loadavg".to_string(), "".to_string(), "total".to_string())).unwrap().last_value;
    let last_pid = statistics.get(&("loadavg".to_string(), "".to_string(), "last_pid".to_string())).unwrap().last_value;
    HISTORY.loadavg.write().unwrap().push_back( LoadavgInfo {
        timestamp,
        load_1,
        load_5,
        load_15,
        current_runnable,
        total,
        last_pid,
    });
}

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
)
{
    let historical_data_read = HISTORY.loadavg.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|loadavg| loadavg.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|loadavg| loadavg.timestamp)
        .max()
        .unwrap();
    let mut low_value: LowValue = Default::default();
    let mut high_value: HighValue = Default::default();
    macro_rules! read_history_and_set_high_and_low_values {
        ($($struct_field_name:ident),*) => {
            $(
            low_value.$struct_field_name = historical_data_read
                .iter()
                .map(|pressure| pressure.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .map(|pressure| pressure.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(load_1, load_5, load_15);
    let high_value_all_load = high_value.load_1.max(high_value.load_5).max(high_value.load_15);
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Load", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value_all_load)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Load")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|loadavg| (loadavg.timestamp, loadavg.load_1)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    macro_rules! draw_lineseries {
        ($([$struct_field_name:ident, $color:expr]),*) => {
            $(
                contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.$struct_field_name)), ShapeStyle { color: $color.into(), filled: true, stroke_width: 2 }))
                    .unwrap()
                    .label(format!("{:25} {:10.2} {:10.2} {:10.2}", concat!(stringify!($struct_field_name), " secs %"), low_value.$struct_field_name, high_value.$struct_field_name, latest.$struct_field_name))
                    .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], $color.filled()));
            )*
        };
    }
    draw_lineseries!([load_1, BLACK], [load_5, GREY_500], [load_15, GREY_A100]);
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
