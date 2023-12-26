use std::io::Cursor;
use axum::{response::IntoResponse, response::Html};
use image::{DynamicImage, ImageOutputFormat};
use crate::stat::create_cpu_plot;
use crate::meminfo::create_memory_plot;

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
     <li><a href="/memory" target="right">Memory</a></li>
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
    r#"<img src="/cpu_all_plot">"#.into()
}

pub async fn cpu_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; 1280 * 900 * 3];
    create_cpu_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(1280, 900, buffer).unwrap());
    let response_buffer = encode_image(&rgb_image, ImageOutputFormat::Png);
    response_buffer
}

pub async fn memory_handler_html() -> Html<&'static str>
{
    r#"<img src="/memory_plot">"#.into()
}

pub async fn memory_handler_generate() -> impl IntoResponse {
    let mut buffer = vec![0; 1280 * 900 * 3];
    create_memory_plot(&mut buffer);
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(1280, 900, buffer).unwrap());
    let response_buffer = encode_image(&rgb_image, ImageOutputFormat::Png);
    response_buffer
}
fn encode_image(image: &DynamicImage, format: ImageOutputFormat) -> Vec<u8> {
    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, format).unwrap();
    buffer.into_inner()
}

