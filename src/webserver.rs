use std::collections::BTreeSet;
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
use axum::{response::IntoResponse, response::Html};
use axum::extract::Path;
use image::{DynamicImage, ImageOutputFormat};
use crate::stat::create_cpu_plot;
use crate::meminfo::{create_memory_plot, create_memory_psi_plot};
use crate::blockdevice::{create_blockdevice_plot, create_blockdevice_psi_plot};
use crate::net_dev::create_networkdevice_plot;
use crate::stat::{create_cpu_load_plot, create_cpu_load_pressure_plot};
use crate::HISTORY;
use crate::{GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH};

pub async fn root_handler() -> Html<String>
{
    // await blockdevices to appear to be able to make a list of them
    loop {
        if HISTORY.blockdevices.read().unwrap().iter().count() > 0 {
            break
        } else {
            sleep(Duration::from_secs(1));
        }
    }
    let unique_blockdevices: Vec<_> = HISTORY.blockdevices.read().unwrap().iter().map(|device| device.device_name.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let mut html_for_blockdevices = String::new();
    for device in &unique_blockdevices
    {
        html_for_blockdevices += format!(r##"<li><a href="/blockdevice/{}" target="right">Blockdevice {}</a></li>"##, device, device).as_str();
    }
    let mut html_for_blockdevices_psi = String::new();
    for device in unique_blockdevices
    {
        html_for_blockdevices_psi += format!(r##"<li><a href="/blockdevice_psi/{}" target="right">Blockdevice-psi {}</a></li>"##, device, device).as_str();
    }
    let unique_networkdevices: Vec<_> = HISTORY.networkdevices.read().unwrap().iter().map(|device| device.device_name.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let mut html_for_networkdevices = String::new();
    for device in unique_networkdevices
    {
        html_for_networkdevices += format!(r##"<li><a href="/networkdevice/{}" target="right">Networkdevice {}</a></li>"##, device, device).as_str();
    }

    format!(r##"<!doctype html>
 <html>
   <head>
   <style>
    .container {{ }}
    .column_left {{ width: 10%; float:left; }}
    .column_right {{ width: 90%; height: 3000px; float:right; }}
   </style>
  </head>
  <body>
  <div class = "container">
   <div class = "column_left">
    <nav>
     <li><a href="/" target="right">Home</a></li>
     <li><a href="/cpu_all" target="right">CPU total</a></li>
     <li><a href="/cpu_all_load" target="right">CPU total-load</a></li>
     <li><a href="/cpu_all_load_psi" target="right">CPU total-load-psi</a></li>
     <li><a href="/memory" target="right">Memory</a></li>
     <li><a href="/memory_psi" target="right">Memory-psi</a></li>
     {html_for_blockdevices}
     {html_for_blockdevices_psi}
     {html_for_networkdevices}
    </nav>
   </div>
   <div class = "column_right">
    <iframe name="right" id="right" width="100%" height="100%">
   </div>
  </div>
  </body>
 </html>
 "##).into()
}

pub async fn cpu_handler_html() -> Html<&'static str>
{
    r#"<img src="/cpu_all_plot">"#.into()
}
pub async fn cpu_load_handler_html() -> Html<&'static str>
{
    r#"<img src="/cpu_all_load_plot">"#.into()
}
pub async fn cpu_load_psi_handler_html() -> Html<&'static str>
{
    r#"<img src="/cpu_all_load_psi_plot">"#.into()
}

pub async fn cpu_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_cpu_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
pub async fn cpu_load_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_cpu_load_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
pub async fn cpu_load_psi_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_cpu_load_pressure_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}

pub async fn memory_handler_html() -> Html<&'static str>
{
    r#"<img src="/memory_plot">"#.into()
}

pub async fn memory_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_memory_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
pub async fn memory_psi_handler_html() -> Html<&'static str>
{
    r#"<img src="/memory_psi_plot">"#.into()
}

pub async fn memory_psi_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_memory_psi_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
pub async fn blockdevice_handler_html(Path(device_name): Path<String>) -> Html<String>
{
    format!(r#"<img src="/blockdevice_plot/{}">"#, device_name).into()
}
pub async fn blockdevice_psi_handler_html(Path(device_name): Path<String>) -> Html<String>
{
    format!(r#"<img src="/blockdevice_psi_plot/{}">"#, device_name).into()
}

pub async fn blockdevice_handler_generate(Path(device_name): Path<String>) -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_blockdevice_plot(&mut buffer, device_name);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
pub async fn blockdevice_psi_handler_generate(Path(device_name): Path<String>) -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_blockdevice_psi_plot(&mut buffer, device_name);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}

pub async fn networkdevice_handler_html(Path(device_name): Path<String>) -> Html<String>
{
    format!(r#"<img src="/networkdevice_plot/{}">"#, device_name).into()
}

pub async fn networkdevice_handler_generate(Path(device_name): Path<String>) -> impl IntoResponse {
    let mut buffer = vec![0; (GRAPH_BUFFER_WIDTH * GRAPH_BUFFER_HEIGHTH * 3).try_into().unwrap()];
    create_networkdevice_plot(&mut buffer, device_name);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
