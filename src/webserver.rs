use std::collections::BTreeSet;
use std::io::Cursor;
use std::thread::sleep;
use std::time::Duration;
use axum::{response::IntoResponse, response::Html};
use axum::extract::Path;
use image::{DynamicImage, ImageOutputFormat};
use crate::stat::create_cpu_plot;
use crate::meminfo::{create_memory_plot, create_memory_psi_plot, create_memory_swap_plot, create_memory_swap_inout_plot};
use crate::blockdevice::{create_blockdevice_plot, create_blockdevice_psi_plot, create_blockdevice_plot_extra};
use crate::net_dev::create_networkdevice_plot;
use crate::stat::{create_cpu_load_plot, create_cpu_load_pressure_plot};
use crate::vmstat::{create_memory_alloc_plot, create_memory_alloc_psi_plot};
use crate::{HISTORY, ARGS};
use log::info;
use axum::{Router, routing::get};

pub async fn webserver() {
    let app = Router::new()
        .route("/handler/:plot_1/:plot_2", get(handler_html))
        .route("/plotter/:plot_1/:plot_2", get(handler_plotter))
        .route("/", get(root_handler));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", ARGS.webserver_port)).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

pub async fn root_handler() -> Html<String>
{
    // await blockdevices to appear to be able to make a list of them
    loop {
        if HISTORY.blockdevices.read().unwrap().iter().count() > 0 {
            break
        } else {
            info!("Waiting for blockdevices to become available...");
            sleep(Duration::from_secs(1));
        }
    }
    let unique_blockdevices: Vec<_> = HISTORY.blockdevices.read().unwrap().iter().map(|device| device.device_name.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let mut html_for_blockdevices = String::new();
    for device in &unique_blockdevices {
        html_for_blockdevices += format!(r##"<li><a href="/handler/blockdevice/{}" target="right">Blockdevice {}</a></li>"##, device, device).as_str();
    }
    let mut html_for_blockdevices_psi = String::new();
    for device in &unique_blockdevices {
        html_for_blockdevices_psi += format!(r##"<li><a href="/handler/blockdevice_psi/{}" target="right">Blockdevice-psi {}</a></li>"##, device, device).as_str();
    }
    let mut html_for_blockdevices_extra = String::new();
    for device in unique_blockdevices {
        html_for_blockdevices_extra += format!(r##"<li><a href="/handler/blockdevice_extra/{}" target="right">Blockdevice-extra {}</a></li>"##, device, device).as_str();
    }
    let unique_networkdevices: Vec<_> = HISTORY.networkdevices.read().unwrap().iter().map(|device| device.device_name.clone()).collect::<BTreeSet<String>>().into_iter().collect();
    let mut html_for_networkdevices = String::new();
    for device in unique_networkdevices {
        html_for_networkdevices += format!(r##"<li><a href="/handler/networkdevice/{}" target="right">Networkdevice {}</a></li>"##, device, device).as_str();
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
     <li><a href="/handler/cpu/x" target="right">CPU total</a></li>
     <li><a href="/handler/cpu_load/x" target="right">CPU total-load</a></li>
     <li><a href="/handler/cpu_load_psi/x" target="right">CPU total-load-psi</a></li>
     <li><a href="/handler/memory/x" target="right">Memory</a></li>
     <li><a href="/handler/memory_alloc/x" target="right">Memory-alloc</a></li>
     <li><a href="/handler/memory_psi/x" target="right">Memory-psi</a></li>
     <li><a href="/handler/memory_psi_alloc/x" target="right">Memory-psi-alloc</a></li>
     <li><a href="/handler/memory_swap/x" target="right">Memory-swapspace</a></li>
     <li><a href="/handler/memory_swap_inout/x" target="right">Memory-swapspace-swapio</a></li>
     {html_for_blockdevices}
     {html_for_blockdevices_psi}
     {html_for_blockdevices_extra}
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

pub async fn handler_html(Path((plot_1, plot_2)): Path<(String, String)>) -> Html<String> {
    format!(r#"<img src="/plotter/{}/{}">"#, plot_1, plot_2).into()
}

pub async fn handler_plotter(Path((plot_1, plot_2)): Path<(String, String)>) -> impl IntoResponse {
    let mut buffer = vec![0; (ARGS.graph_width * ARGS.graph_heighth * 3).try_into().unwrap()];
    match plot_1.as_str() {
        "networkdevice" => create_networkdevice_plot(&mut buffer, plot_2),
        "blockdevice" => create_blockdevice_plot(&mut buffer, plot_2),
        "blockdevice_psi" => create_blockdevice_psi_plot(&mut buffer, plot_2),
        "blockdevice_extra" => create_blockdevice_plot_extra(&mut buffer, plot_2),
        "cpu" => create_cpu_plot(&mut buffer),
        "cpu_load" => create_cpu_load_plot(&mut buffer),
        "cpu_load_psi" => create_cpu_load_pressure_plot(&mut buffer),
        "memory" => create_memory_plot(&mut buffer),
        "memory_alloc" => create_memory_alloc_plot(&mut buffer),
        "memory_psi" => create_memory_psi_plot(&mut buffer),
        "memory_psi_alloc" => create_memory_alloc_psi_plot(&mut buffer),
        "memory_swap" => create_memory_swap_plot(&mut buffer),
        "memory_swap_inout" => create_memory_swap_inout_plot(&mut buffer),
        &_ => todo!(),
    }
    let rgb_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(ARGS.graph_width, ARGS.graph_heighth, buffer).unwrap());
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();
    cursor.into_inner()
}
