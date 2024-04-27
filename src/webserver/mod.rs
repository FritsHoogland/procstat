pub mod blockdevice;
pub mod loadavg;
pub mod meminfo;
pub mod net_dev;
pub mod pressure;
pub mod stat;
pub mod vmstat;

use crate::webserver::meminfo::{
    create_memory_plot, create_memory_psi_plot, create_memory_swap_inout_plot,
    create_memory_swap_plot,
};
use crate::webserver::net_dev::create_networkdevice_plot;
use crate::webserver::stat::create_cpu_plot;
use crate::webserver::stat::{create_cpu_load_plot, create_cpu_load_pressure_plot};
use crate::webserver::vmstat::{create_memory_alloc_plot, create_memory_alloc_psi_plot};
use crate::webserver::{
    blockdevice::{
        create_blockdevice_plot, create_blockdevice_plot_extra, create_blockdevice_psi_plot,
    },
    meminfo::create_memory_commit,
};
use crate::{ARGS, HISTORY};
use axum::{extract::Path, response::Html, response::IntoResponse, routing::get, Router};
use image::{DynamicImage, ImageFormat};
//use image::DynamicImage;
use log::info;
use std::fmt::Write;
use std::{collections::BTreeSet, io::Cursor, thread::sleep, time::Duration};

use self::meminfo::create_memory_active_inactive_plot;

pub async fn webserver() {
    let app = Router::new()
        .route("/handler/:plot_1/:plot_2", get(handler_html))
        .route("/plotter/:plot_1/:plot_2", get(handler_plotter))
        .route("/", get(root_handler));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", ARGS.webserver_port))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn root_handler() -> Html<String> {
    // await blockdevices to appear to be able to make a list of them
    loop {
        if HISTORY.blockdevices.read().unwrap().iter().count() > 0 {
            break;
        } else {
            info!("Waiting for blockdevices to become available...");
            sleep(Duration::from_secs(1));
        }
    }

    let html_for_blockdevices = HISTORY
        .blockdevices
        .read()
        .unwrap()
        .iter()
        .map(|d| d.device_name.clone())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>()
        .iter()
        .fold(String::new(), |mut output, d| {
            let _ = write!(
                output,
                r##"<li><a href="/handler/blockdevice/{}" target="right">Blockdevice {}</a>"##,
                d, d
            );
            output
        });

    let html_for_blockdevices_psi = HISTORY.blockdevices
        .read()
        .unwrap()
        .iter()
        .map(|d| d.device_name.clone())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>()
        .iter()
        .fold(String::new(), |mut output, d| {
            let _ = write!(
                output,
                r##"<li><a href="/handler/blockdevice_psi/{}" target="right">Blockdevice-psi {}</a>"##, d, d
            );
            output
        });

    let html_for_blockdevices_extra = HISTORY.blockdevices
        .read()
        .unwrap()
        .iter()
        .map(|d| d.device_name.clone())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>()
        .iter()
        .fold(String::new(), |mut output, d| {
            let _ = write!(output,
               r##"<li><a href="/handler/blockdevice_extra/{}" target="right">Blockdevice-extra {}</a>"##, d, d
            );
            output
        });

    let html_for_networkdevices = HISTORY
        .networkdevices
        .read()
        .unwrap()
        .iter()
        .map(|d| d.device_name.clone())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>()
        .iter()
        .fold(String::new(), |mut output, d| {
            let _ = write!(
                output,
                r##"<li><a href="/handler/networkdevice/{}" target="right">Networkdevice {}</a>"##,
                d, d
            );
            output
        });

    format!(
        r##"<!doctype html>
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
     <li><a href="/handler/memory_commit/x" target="right">Memory-committed</a></li>
     <li><a href="/handler/memory_psi/x" target="right">Memory-psi</a></li>
     <li><a href="/handler/memory_psi_alloc/x" target="right">Memory-psi-alloc</a></li>
     <li><a href="/handler/memory_swap/x" target="right">Memory-swapspace</a></li>
     <li><a href="/handler/memory_swap_inout/x" target="right">Memory-swapspace-swapio</a></li>
     <li><a href="/handler/memory_act_inact/x" target="right">Memory-active-inactive</a></li>
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
 "##
    )
    .into()
}

pub async fn handler_html(Path((plot_1, plot_2)): Path<(String, String)>) -> Html<String> {
    format!(r#"<img src="/plotter/{}/{}">"#, plot_1, plot_2).into()
}

pub async fn handler_plotter(Path((plot_1, plot_2)): Path<(String, String)>) -> impl IntoResponse {
    let mut buffer = vec![
        0;
        (ARGS.graph_width * ARGS.graph_height * 3)
            .try_into()
            .unwrap()
    ];
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
        "memory_commit" => create_memory_commit(&mut buffer),
        "memory_psi" => create_memory_psi_plot(&mut buffer),
        "memory_psi_alloc" => create_memory_alloc_psi_plot(&mut buffer),
        "memory_swap" => create_memory_swap_plot(&mut buffer),
        "memory_swap_inout" => create_memory_swap_inout_plot(&mut buffer),
        "memory_act_inact" => create_memory_active_inactive_plot(&mut buffer),
        &_ => todo!(),
    }
    let rgb_image = DynamicImage::ImageRgb8(
        image::RgbImage::from_raw(ARGS.graph_width, ARGS.graph_height, buffer).unwrap(),
    );
    let mut cursor = Cursor::new(Vec::new());
    rgb_image.write_to(&mut cursor, ImageFormat::Png).unwrap();
    cursor.into_inner()
}
