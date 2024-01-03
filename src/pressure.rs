use std::collections::HashMap;
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::coord::Shift;
use plotters::drawing::DrawingArea;
use plotters::element::Rectangle;
use plotters::prelude::*;
use plotters::prelude::full_palette::{BLUE_A100, RED_A100};
use crate::common::{ProcData, Statistic, single_statistic_u64, single_statistic_f64, single_statistic_option_u64, single_statistic_option_f64};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};

#[derive(Debug)]
pub struct PressureInfo {
    pub timestamp: DateTime<Local>,
    pub cpu_some_avg10: f64,
    pub cpu_some_avg60: f64,
    pub cpu_some_avg300: f64,
    pub cpu_some_total: f64,
    pub cpu_full_avg10: f64,
    pub cpu_full_avg60: f64,
    pub cpu_full_avg300: f64,
    pub cpu_full_total: f64,
    pub io_some_avg10: f64,
    pub io_some_avg60: f64,
    pub io_some_avg300: f64,
    pub io_some_total: f64,
    pub io_full_avg10: f64,
    pub io_full_avg60: f64,
    pub io_full_avg300: f64,
    pub io_full_total: f64,
    pub memory_some_avg10: f64,
    pub memory_some_avg60: f64,
    pub memory_some_avg300: f64,
    pub memory_some_total: f64,
    pub memory_full_avg10: f64,
    pub memory_full_avg60: f64,
    pub memory_full_avg300: f64,
    pub memory_full_total: f64,
}

pub async fn process_pressure_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    single_statistic_f64("pressure", "","cpu_some_avg10", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg10, statistics).await;
    single_statistic_f64("pressure", "","cpu_some_avg60", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg60, statistics).await;
    single_statistic_f64("pressure", "","cpu_some_avg300", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_some_avg300, statistics).await;
    single_statistic_u64("pressure", "","cpu_some_total", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_some_total, statistics).await;
    single_statistic_option_f64("pressure", "","cpu_full_avg10", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg10, statistics).await;
    single_statistic_option_f64("pressure", "","cpu_full_avg60", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg60, statistics).await;
    single_statistic_option_f64("pressure", "","cpu_full_avg300", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_full_avg300, statistics).await;
    single_statistic_option_u64("pressure", "","cpu_full_total", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().cpu_full_total, statistics).await;
    single_statistic_f64("pressure", "","io_some_avg10", proc_data.timestamp, proc_data.pressure.psi.as_ref().unwrap().io_some_avg10, statistics).await;
    single_statistic_f64("pressure", "","io_some_avg60", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_some_avg60, statistics).await;
    single_statistic_f64("pressure", "","io_some_avg300", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_some_avg300, statistics).await;
    single_statistic_u64("pressure", "","io_some_total", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_some_total, statistics).await;
    single_statistic_f64("pressure", "","io_full_avg10", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_full_avg10, statistics).await;
    single_statistic_f64("pressure", "","io_full_avg60", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_full_avg60, statistics).await;
    single_statistic_f64("pressure", "","io_full_avg300", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_full_avg300, statistics).await;
    single_statistic_u64("pressure", "","io_full_total", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().io_full_total, statistics).await;
    single_statistic_f64("pressure", "","memory_some_avg10", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_some_avg10, statistics).await;
    single_statistic_f64("pressure", "","memory_some_avg60", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_some_avg60, statistics).await;
    single_statistic_f64("pressure", "","memory_some_avg300", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_some_avg300, statistics).await;
    single_statistic_u64("pressure", "","memory_some_total", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_some_total, statistics).await;
    single_statistic_f64("pressure", "","memory_full_avg10", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_full_avg10, statistics).await;
    single_statistic_f64("pressure", "","memory_full_avg60", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_full_avg60, statistics).await;
    single_statistic_f64("pressure", "","memory_full_avg300", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_full_avg300, statistics).await;
    single_statistic_u64("pressure", "","memory_full_total", proc_data.timestamp,proc_data.pressure.psi.as_ref().unwrap().memory_full_total, statistics).await;
}

pub async fn add_pressure_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    if !statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_avg10".to_string())).unwrap().updated_value { return };
    let timestamp = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_avg10".to_string())).unwrap().last_timestamp;
    let cpu_some_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_avg10".to_string())).unwrap().last_value;
    let cpu_some_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_avg60".to_string())).unwrap().last_value;
    let cpu_some_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_avg300".to_string())).unwrap().last_value;
    let cpu_some_total = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_some_total".to_string())).unwrap().per_second_value;
    let cpu_full_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg10".to_string())).unwrap().last_value;
    let cpu_full_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg60".to_string())).unwrap().last_value;
    let cpu_full_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_avg300".to_string())).unwrap().last_value;
    let cpu_full_total = statistics.get(&("pressure".to_string(), "".to_string(), "cpu_full_total".to_string())).unwrap().per_second_value;
    let io_some_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "io_some_avg10".to_string())).unwrap().last_value;
    let io_some_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "io_some_avg60".to_string())).unwrap().last_value;
    let io_some_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "io_some_avg300".to_string())).unwrap().last_value;
    let io_some_total = statistics.get(&("pressure".to_string(), "".to_string(), "io_some_total".to_string())).unwrap().per_second_value;
    let io_full_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "io_full_avg10".to_string())).unwrap().last_value;
    let io_full_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "io_full_avg60".to_string())).unwrap().last_value;
    let io_full_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "io_full_avg300".to_string())).unwrap().last_value;
    let io_full_total = statistics.get(&("pressure".to_string(), "".to_string(), "io_full_total".to_string())).unwrap().per_second_value;
    let memory_some_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_some_avg10".to_string())).unwrap().last_value;
    let memory_some_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_some_avg60".to_string())).unwrap().last_value;
    let memory_some_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_some_avg300".to_string())).unwrap().last_value;
    let memory_some_total = statistics.get(&("pressure".to_string(), "".to_string(), "memory_some_total".to_string())).unwrap().per_second_value;
    let memory_full_avg10 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_full_avg10".to_string())).unwrap().last_value;
    let memory_full_avg60 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_full_avg60".to_string())).unwrap().last_value;
    let memory_full_avg300 = statistics.get(&("pressure".to_string(), "".to_string(), "memory_full_avg300".to_string())).unwrap().last_value;
    let memory_full_total = statistics.get(&("pressure".to_string(), "".to_string(), "memory_full_total".to_string())).unwrap().per_second_value;
    HISTORY.pressure.write().unwrap().push_back( PressureInfo {
        timestamp,
        cpu_some_avg10,
        cpu_some_avg60,
        cpu_some_avg300,
        cpu_some_total,
        cpu_full_avg10,
        cpu_full_avg60,
        cpu_full_avg300,
        cpu_full_total,
        io_some_avg10,
        io_some_avg60,
        io_some_avg300,
        io_some_total,
        io_full_avg10,
        io_full_avg60,
        io_full_avg300,
        io_full_total,
        memory_some_avg10,
        memory_some_avg60,
        memory_some_avg300,
        memory_some_total,
        memory_full_avg10,
        memory_full_avg60,
        memory_full_avg300,
        memory_full_total,
    });
}

pub fn pressure_cpu_some_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
)
{
    let historical_data_read = HISTORY.pressure.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let low_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg10)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg10)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg60)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg60)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg300)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_avg300)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_total / 1_000_000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.cpu_some_total / 1_000_000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_all_avg = high_value_some_avg10.max(high_value_some_avg60).max(high_value_some_avg300);
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Pressure stall CPU", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value_some_total)
        .unwrap()
        .set_secondary_coord(start_time..end_time, low_value..high_value_all_avg);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("Percent")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|loadavg| (loadavg.timestamp, loadavg.cpu_some_total)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    // total
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.cpu_some_total / 1_000_000_f64)), 0.0, BLUE_A100))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some cpu total", low_value_some_total, high_value_some_total, latest.cpu_some_total / 1_000_000_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    // colour picker
    let mut palette99_pick = 1_usize;
    // avg 10
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.cpu_some_avg10)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some cpu avg10", low_value_some_avg10, high_value_some_avg10, latest.cpu_some_avg10))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 2; // skip yellow
    // avg 60
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.cpu_some_avg60)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some cpu avg60", low_value_some_avg60, high_value_some_avg60, latest.cpu_some_avg60))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // avg 300
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.cpu_some_avg300)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some cpu avg300", low_value_some_avg300, high_value_some_avg300, latest.cpu_some_avg300))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
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
)
{
    let historical_data_read = HISTORY.pressure.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let low_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg10)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg10)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg60)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg60)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg300)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_avg300)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_total / 1_000_000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_some_total / 1_000_000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg10 = historical_data_read
        .iter()
        .map(|loadavg| loadavg.memory_full_avg10)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_avg10)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_avg60)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_avg60)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_avg300)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_avg300)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_total = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_total / 1_000_000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_total = historical_data_read
        .iter()
        .map(|pressure| pressure.memory_full_total / 1_000_000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_all_avg = [ high_value_some_avg10, high_value_some_avg60, high_value_some_avg300, high_value_full_avg10, high_value_full_avg60, high_value_full_avg300].into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let high_value_all_total = high_value_some_total.max(high_value_full_total);
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Pressure stall memory", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value_all_total)
        .unwrap()
        .set_secondary_coord(start_time..end_time, low_value..high_value_all_avg);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("Percent")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|loadavg| (loadavg.timestamp, loadavg.memory_some_total)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    // some total
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_some_total / 1_000_000_f64)), 0.0, BLUE_A100))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some memory total", low_value_some_total, high_value_some_total, latest.memory_some_total / 1_000_000_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    // colour picker
    let mut palette99_pick = 1_usize;
    // avg 10
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_some_avg10)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some memory avg10", low_value_some_avg10, high_value_some_avg10, latest.memory_some_avg10))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 2; // skip yellow
    // avg 60
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_some_avg60)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some memory avg60", low_value_some_avg60, high_value_some_avg60, latest.memory_some_avg60))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // avg 300
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_some_avg300)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some memory avg300", low_value_some_avg300, high_value_some_avg300, latest.memory_some_avg300))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));

    // full total
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_full_total / 1_000_000_f64)), 0.0, RED_A100))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full memory total", low_value_full_total, high_value_full_total, latest.memory_full_total / 1_000_000_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED_A100.filled()));

    // avg 10
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_full_avg10)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full memory avg10", low_value_full_avg10, high_value_full_avg10, latest.memory_full_avg10))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 2; // skip yellow
    // avg 60
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_full_avg60)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full memory avg60", low_value_full_avg60, high_value_full_avg60, latest.memory_full_avg60))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // avg 300
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.memory_full_avg300)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full memory avg300", low_value_full_avg300, high_value_full_avg300, latest.memory_full_avg300))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
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
)
{
    let historical_data_read = HISTORY.pressure.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|pressure| pressure.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let low_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg10)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg10)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg60)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg60)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg300)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_avg300)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_total / 1_000_000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_some_total = historical_data_read
        .iter()
        .map(|pressure| pressure.io_some_total / 1_000_000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg10 = historical_data_read
        .iter()
        .map(|loadavg| loadavg.io_full_avg10)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg10 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_avg10)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_avg60)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg60 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_avg60)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_avg300)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_avg300 = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_avg300)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let low_value_full_total = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_total / 1_000_000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_full_total = historical_data_read
        .iter()
        .map(|pressure| pressure.io_full_total / 1_000_000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_all_avg = [ high_value_some_avg10, high_value_some_avg60, high_value_some_avg300, high_value_full_avg10, high_value_full_avg60, high_value_full_avg300].into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let high_value_all_total = high_value_some_total.max(high_value_full_total);
    let latest = historical_data_read
        .back()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("Pressure stall io", (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value_all_total)
        .unwrap()
        .set_secondary_coord(start_time..end_time, low_value..high_value_all_avg);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Time per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("Percent")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|loadavg| (loadavg.timestamp, loadavg.io_some_total)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    // some total
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_some_total / 1_000_000_f64)), 0.0, BLUE_A100))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some io total", low_value_some_total, high_value_some_total, latest.io_some_total / 1_000_000_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLUE_A100.filled()));

    // colour picker
    let mut palette99_pick = 1_usize;
    // avg 10
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_some_avg10)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some io avg10", low_value_some_avg10, high_value_some_avg10, latest.io_some_avg10))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 2; // skip yellow
    // avg 60
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_some_avg60)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some io avg60", low_value_some_avg60, high_value_some_avg60, latest.io_some_avg60))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // avg 300
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_some_avg300)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "some io avg300", low_value_some_avg300, high_value_some_avg300, latest.io_some_avg300))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));

    // full total
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_full_total / 1_000_000_f64)), 0.0, RED_A100))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full io total", low_value_full_total, high_value_full_total, latest.io_full_total / 1_000_000_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED_A100.filled()));

    // avg 10
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_full_avg10)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full io avg10", low_value_full_avg10, high_value_full_avg10, latest.io_full_avg10))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 2; // skip yellow
    // avg 60
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_full_avg60)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full io avg60", low_value_full_avg60, high_value_full_avg60, latest.io_full_avg60))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // avg 300
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter().map(|pressure| (pressure.timestamp, pressure.io_full_avg300)), Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "full io avg300", low_value_full_avg300, high_value_full_avg300, latest.io_full_avg300))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //
    // draw the legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
