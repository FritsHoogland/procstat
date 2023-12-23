use std::io::Cursor;
use axum::{response::IntoResponse, response::Html};
use image::{DynamicImage, ImageOutputFormat};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::prelude::*;
use crate::HISTORY;
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, MESH_STYLE_FONT_SIZE, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_RIGHT, MESH_STYLE_FONT};

pub async fn root_handler() -> Html<&'static str>
{
    r##"<!doctype html>
 <html>
   <head>
   <style>
    .container { }
    .column_left { width: 5%; float:left; }
    .column_right { width: 95%; height: 3000px; float:right; }
   </style>
  </head>
  <body>
  <div class = "container">
   <div class = "column_left">
    <nav>
     <li><a href="/" target="right">Home</a></li>
     <li><a href="/cpu_all" target="right">CPU total</a></li>
    </nav>
   </div>
   <div class = "column_right">
    <iframe name="right" id="right" width="100%" height="100%">
   </div>
  </div>
  </body>
 </html>
 "##.into()
}

pub async fn cpu_handler_html() -> Html<&'static str>
{
    r#"
    <img src="/cpu_all_plot">
    "#.into()
}

pub async fn cpu_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; 1280 * 900 * 3];
    //generate_plot(&mut buffer);
    create_cpu_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(1280, 900, buffer).unwrap());
    let response_buffer = encode_image(&rgb_image, ImageOutputFormat::Png);
    response_buffer
}

/*
fn generate_plot(buffer: &mut Vec<u8>) {
    let backend = BitMapBackend::with_buffer(buffer, (800, 600)).into_drawing_area();
    backend.fill(&WHITE).unwrap();

    // Draw the plot using Plotters
    let mut chart = ChartBuilder::on(&backend)
        .caption("Example Plot", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..10, 0..10)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(vec![(0, 0), (1, 2), (2, 5)], &RED))
        .unwrap();
}
 */

fn encode_image(image: &DynamicImage, format: ImageOutputFormat) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, format).unwrap();
    buffer.into_inner()
}

pub fn create_cpu_plot(
    buffer: &mut Vec<u8>
)
{
    let historical_data_read = HISTORY.cpu.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .map(|cpustat| cpustat.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .map(|cpustat| cpustat.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value_cpu = historical_data_read
        .iter()
        .map(|cpustat| cpustat.user+cpustat.nice+cpustat.system+cpustat.iowait+cpustat.steal+cpustat.irq+cpustat.softirq+cpustat.idle)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_schedstat = historical_data_read
        .iter()
        .map(|cpustat| cpustat.scheduler_running+cpustat.scheduler_waiting)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value = vec![high_value_cpu, high_value_schedstat].into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest = historical_data_read
        .back()
        .unwrap();

        // create the plot
    let backend = BitMapBackend::with_buffer(buffer, (1280,900)).into_drawing_area();
    backend.fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&backend)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption("CPU usage".to_string(), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(4)
        .x_label_formatter(&|x| x.to_rfc3339())
        .y_desc("CPU per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    // colour picker
    let mut palette99_pick = 1_usize;
    // scheduler waiting = scheduler_waiting + scheduler_running
    let min_scheduler_wait = historical_data_read.iter().map(|cpustat| cpustat.scheduler_waiting).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_scheduler_wait = historical_data_read.iter().map(|cpustat| cpustat.scheduler_waiting).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.scheduler_waiting + cpustat.scheduler_running)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "scheduler wait", min_scheduler_wait, max_scheduler_wait, latest.scheduler_waiting))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // scheduler running
    let min_scheduler_run = historical_data_read.iter().map(|cpustat| cpustat.scheduler_running).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_scheduler_run = historical_data_read.iter().map(|cpustat| cpustat.scheduler_running).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.scheduler_running)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "scheduler run", min_scheduler_run, max_scheduler_run, latest.scheduler_running))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    // cpu states
    // guest_nice = guest_nice + guest_user + softirq + irq + steal + iowait + system + nice + user
    let min_guest_nice = historical_data_read.iter().map(|cpustat| cpustat.guest_nice).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_guest_nice = historical_data_read.iter().map(|cpustat| cpustat.guest_nice).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.guest_nice + cpustat.guest + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "guest_nice", min_guest_nice, max_guest_nice, latest.guest_nice))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // guest_user = guest_user + softirq + irq + steal + iowait + system + nice + user
    let min_guest_user = historical_data_read.iter().map(|cpustat| cpustat.guest).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_guest_user = historical_data_read.iter().map(|cpustat| cpustat.guest).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.guest + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "guest_user", min_guest_user, max_guest_user, latest.guest))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // softirq = softirq + irq + steal + iowait + system + nice + user
    let min_softirq = historical_data_read.iter().map(|cpustat| cpustat.softirq).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_softirq = historical_data_read.iter().map(|cpustat| cpustat.softirq).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "softirq", min_softirq, max_softirq, latest.softirq))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // irq = irq + steal + iowait + system + nice + user
    let min_irq = historical_data_read.iter().map(|cpustat| cpustat.irq).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_irq = historical_data_read.iter().map(|cpustat| cpustat.irq).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "irq", min_irq, max_irq, latest.irq))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // steal = steal + iowait + system + nice + user
    let min_steal = historical_data_read.iter().map(|cpustat| cpustat.steal).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_steal = historical_data_read.iter().map(|cpustat| cpustat.steal).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "steal", min_steal, max_steal, latest.steal))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // iowait = iowait + system + nice + user
    let min_iowait = historical_data_read.iter().map(|cpustat| cpustat.iowait).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_iowait = historical_data_read.iter().map(|cpustat| cpustat.iowait).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "iowait", min_iowait, max_iowait, latest.iowait))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // system = system + nice + user
    let min_system = historical_data_read.iter().map(|cpustat| cpustat.system).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_system = historical_data_read.iter().map(|cpustat| cpustat.system).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.system + cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "system", min_system, max_system, latest.system))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // nice = nice + user
    let min_nice = historical_data_read.iter().map(|cpustat| cpustat.nice).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_nice = historical_data_read.iter().map(|cpustat| cpustat.nice).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.nice + cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "nice", min_nice, max_nice, latest.nice))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    palette99_pick += 1;
    //
    // user
    let min_user = historical_data_read.iter().map(|cpustat| cpustat.user).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_user = historical_data_read.iter().map(|cpustat| cpustat.user).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.user)), 0.0, Palette99::pick(palette99_pick)))
        .unwrap()
        .label(format!("{:25} min: {:10.2}, max: {:10.2}, last: {:10.2}", "user", min_user, max_user, latest.user))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], Palette99::pick(palette99_pick).filled()));
    //
    // draw a line for total cpu
    //contextarea.draw_series(AreaSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, cpustat.idle )),
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().map(|cpustat| (cpustat.timestamp, (cpustat.guest_nice + cpustat.guest + cpustat.idle + cpustat.softirq + cpustat.irq + cpustat.steal + cpustat.iowait + cpustat.system + cpustat.nice + cpustat.user).round())),
                                            ShapeStyle { color: RED.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25}      {:10}       {:10}  last: {:10.2}", "total (v)cpu", "", "", (latest.idle + latest.guest_nice + latest.guest + latest.softirq + latest.irq + latest.steal + latest.iowait + latest.system + latest.nice + latest.user).round()))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
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