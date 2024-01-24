#![allow(unused_assignments)]

use std::collections::{BTreeSet, HashMap};
use chrono::{DateTime, Local};
use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::chart::{ChartBuilder, LabelAreaPosition, SeriesLabelPosition::UpperLeft};
use plotters::coord::Shift;
use plotters::element::Rectangle;
use plotters::prelude::*;
use serde::{Serialize, Deserialize};
//
use crate::common::{ProcData, single_statistic_u64, Statistic};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};
use crate::{GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkDeviceInfo {
    pub timestamp: DateTime<Local>,
    pub device_name: String,
    pub receive_bytes: f64,
    pub receive_packets: f64,
    pub receive_errors: f64,
    pub receive_drop: f64,
    pub receive_fifo: f64,
    pub receive_frame: f64,
    pub receive_compressed: f64,
    pub receive_multicast: f64,
    pub transmit_bytes: f64,
    pub transmit_packets: f64,
    pub transmit_errors: f64,
    pub transmit_drop: f64,
    pub transmit_fifo: f64,
    pub transmit_collisions: f64,
    pub transmit_carrier: f64,
    pub transmit_compressed: f64,
}

pub async fn process_net_dev_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    for interface in &proc_data.net_dev.interface
    {
        macro_rules! add_net_dev_data_to_statistics {
            ($($field_name:ident),*) => {
                $(
                    single_statistic_u64("net_dev", &interface.name, stringify!($field_name), proc_data.timestamp, interface.$field_name, statistics).await;
                )*
            };
        }
        add_net_dev_data_to_statistics!(receive_bytes, receive_packets, receive_errors, receive_drop, receive_fifo, receive_frame, receive_compressed, receive_multicast, transmit_bytes, transmit_packets, transmit_errors, transmit_drop, transmit_fifo, transmit_collisions, transmit_carrier, transmit_compressed);
    }
}

pub async fn add_networkdevices_to_history(statistics: &HashMap<(String, String, String), Statistic>)
{
    let nic_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "net_dev")
        .map(|(_, nic_name, _)| nic_name)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics.get(&("net_dev".to_string(), nic_list[0].to_string(), "receive_bytes".to_string())).unwrap().updated_value { return; };

    let mut totals = [0_f64; 16];

    let timestamp = statistics.get(&("net_dev".to_string(), nic_list[0].to_string(), "receive_bytes".to_string())).unwrap().last_timestamp;

    for network_interface in nic_list.iter().filter(|interface_name| !interface_name.starts_with("lo"))
    {
        // receive
        let receive_bytes = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_bytes".to_string())).unwrap().per_second_value;
        totals[0] += receive_bytes;
        let receive_packets = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_packets".to_string())).unwrap().per_second_value;
        totals[1] += receive_packets;
        let receive_errors = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_errors".to_string())).unwrap().per_second_value * 512_f64; // convert 512 bytes sector reads to bytes
        totals[2] += receive_errors;
        let receive_drop = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_drop".to_string())).unwrap().per_second_value;
        totals[3] += receive_drop;
        let receive_fifo = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_fifo".to_string())).unwrap().per_second_value;
        totals[4] += receive_fifo;
        let receive_frame = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_frame".to_string())).unwrap().per_second_value;
        totals[5] += receive_frame;
        let receive_compressed = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_compressed".to_string())).unwrap().per_second_value;
        totals[6] += receive_compressed;
        let receive_multicast = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "receive_multicast".to_string())).unwrap().per_second_value;
        totals[7] += receive_multicast;
        // transmit
        let transmit_bytes = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_bytes".to_string())).unwrap().per_second_value;
        totals[8] += transmit_bytes;
        let transmit_packets = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_packets".to_string())).unwrap().per_second_value;
        totals[9] += transmit_packets;
        let transmit_errors = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_errors".to_string())).unwrap().per_second_value * 512_f64; // convert 512 bytes sector reads to bytes
        totals[10] += transmit_errors;
        let transmit_drop = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_drop".to_string())).unwrap().per_second_value;
        totals[11] += transmit_drop;
        let transmit_fifo = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_fifo".to_string())).unwrap().per_second_value;
        totals[12] += transmit_fifo;
        let transmit_collisions = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_collisions".to_string())).unwrap().per_second_value;
        totals[13] += transmit_collisions;
        let transmit_carrier = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_carrier".to_string())).unwrap().per_second_value;
        totals[14] += transmit_carrier;
        let transmit_compressed = statistics.get(&("net_dev".to_string(), network_interface.to_string(), "transmit_compressed".to_string())).unwrap().per_second_value;
        totals[15] += transmit_compressed;

        HISTORY.networkdevices.write().unwrap().push_back(NetworkDeviceInfo {
            timestamp,
            device_name: network_interface.to_string(),
            receive_bytes,
            receive_packets,
            receive_errors,
            receive_drop,
            receive_fifo,
            receive_frame,
            receive_compressed,
            receive_multicast,
            transmit_bytes,
            transmit_packets,
            transmit_errors,
            transmit_drop,
            transmit_fifo,
            transmit_collisions,
            transmit_carrier,
            transmit_compressed,
        });
    }
    HISTORY.networkdevices.write().unwrap().push_back(NetworkDeviceInfo {
        timestamp,
        device_name: "TOTAL".to_string(),
        receive_bytes: totals[0],
        receive_packets: totals[1],
        receive_errors: totals[2],
        receive_drop: totals[3],
        receive_fifo: totals[4],
        receive_frame: totals[5],
        receive_compressed: totals[6],
        receive_multicast: totals[7],
        transmit_bytes: totals[8],
        transmit_packets: totals[9],
        transmit_errors: totals[10],
        transmit_drop: totals[11],
        transmit_fifo: totals[12],
        transmit_collisions: totals[13],
        transmit_carrier: totals[14],
        transmit_compressed: totals[15],
    });
}

pub async fn print_net_dev(
    statistics: &HashMap<(String, String, String), Statistic>,
    output: &str,
)
{
    let device_list: Vec<_> = statistics.keys()
        .filter(|(group, _, _)| group == "net_dev")
        .map(|(_, device, _)| device)
        .collect::<BTreeSet<&String>>()
        .into_iter()
        .collect();

    if !statistics.get(&("net_dev".to_string(), device_list[0].to_string(), "receive_bytes".to_string())).unwrap().updated_value { return; };

    match output
    {
        "sar-n-DEV" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                     "Timestamp",
                     "IFACE",
                     "rxpck/s",
                     "txpck/s",
                     "rxMB/s",
                     "txMB/s",
                     "rxcmp/s",
                     "txcmp/s",
                     "rxmcst/s",
            );
        }
        "sar-n-EDEV" => {
            println!("{:10} {:7}    {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
                     "Timestamp",
                     "IFACE",
                     "rxerr/s",
                     "txerr/s",
                     "coll/s",
                     "rxdrop/s",
                     "txdrop/s",
                     "txcarr/s",
                     "rxfram/s",
                     "rxfifo/s",
                     "txfifo/s",
            );
        }
        &_ => todo!(),
    }

    for device in device_list
    {
        let timestamp = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_bytes".to_string())).unwrap().last_timestamp;
        let receive_packets = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_packets".to_string())).unwrap().per_second_value;
        let transmit_packets = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_packets".to_string())).unwrap().per_second_value;
        let receive_bytes = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_bytes".to_string())).unwrap().per_second_value;
        let transmit_bytes = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_bytes".to_string())).unwrap().per_second_value;
        let receive_compressed = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_compressed".to_string())).unwrap().per_second_value;
        let transmit_compressed = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_compressed".to_string())).unwrap().per_second_value;
        let receive_multicast = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_multicast".to_string())).unwrap().per_second_value;
        let receive_errors = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_errors".to_string())).unwrap().per_second_value;
        let transmit_errors = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_errors".to_string())).unwrap().per_second_value;
        let transmit_collisions = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_collisions".to_string())).unwrap().per_second_value;
        let receive_drop = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_drop".to_string())).unwrap().per_second_value;
        let transmit_drop = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_drop".to_string())).unwrap().per_second_value;
        let transmit_carrier = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_carrier".to_string())).unwrap().per_second_value;
        let receive_frame = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_frame".to_string())).unwrap().per_second_value;
        let receive_fifo = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_fifo".to_string())).unwrap().per_second_value;
        let transmit_fifo = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_fifo".to_string())).unwrap().per_second_value;

        match output
        {
            "sar-n-DEV" => {
                println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                         timestamp.format("%H:%M:%S"),
                         device,
                         receive_packets,
                         transmit_packets,
                         receive_bytes / (1024_f64 * 1024_f64),
                         transmit_bytes / (1024_f64 * 1024_f64),
                         receive_compressed,
                         transmit_compressed,
                         receive_multicast,
                );
            }
            "sar-n-EDEV" => {
                println!("{:10} {:7}    {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2} {:10.2}",
                         timestamp.format("%H:%M:%S"),
                         device,
                         receive_errors,
                         transmit_errors,
                         transmit_collisions,
                         receive_drop,
                         transmit_drop,
                         transmit_carrier,
                         receive_frame,
                         receive_fifo,
                         transmit_fifo,
                );
            }
            &_ => todo!(),
        }
    }
}

pub fn create_networkdevice_plot(
    buffer: &mut [u8],
    device_name: String,
)
{
    let backend = BitMapBackend::with_buffer(buffer, (GRAPH_BUFFER_WIDTH, GRAPH_BUFFER_HEIGHTH)).into_drawing_area();
    let mut multi_backend = backend.split_evenly((3, 1));
    networkdevice_mbit_plot(&mut multi_backend, 0, device_name.clone());
    networkdevice_packet_plot(&mut multi_backend, 1, device_name.clone());
    networkdevice_error_plot(&mut multi_backend, 2, device_name);
}

fn networkdevice_mbit_plot(
    multi_backend: &mut [DrawingArea<BitMapBackend<RGBPixel>, Shift>],
    backend_number: usize,
    device_name: String,
)
{
    let historical_data_read = HISTORY.networkdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .max()
        .unwrap();
    let high_value_mbit = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| ((networkdevices.receive_bytes + networkdevices.transmit_bytes) / (1024_f64 * 1024_f64)) * 8_f64 * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let latest = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name)
        .last()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Networkdevice: {} Megabit per second", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value_mbit)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Megabit per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                                .take(1)
                                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.transmit_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // total mbit
    let min_total_mbit = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && (networkdevice.transmit_bytes + networkdevice.receive_bytes) > 0_f64)
        .map(|networkdevice| ((networkdevice.transmit_bytes + networkdevice.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_mbit= historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && (networkdevice.transmit_bytes + networkdevice.receive_bytes) > 0_f64)
        .map(|networkdevice| ((networkdevice.transmit_bytes + networkdevice.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name)
                                                .map(|networkdevice| (networkdevice.timestamp, ((networkdevice.transmit_bytes + networkdevice.receive_bytes) / (1024_f64 * 1024_f64)) * 8_f64)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "total", min_total_mbit, max_total_mbit, (latest.transmit_bytes + latest.receive_bytes) / (1024_f64 * 1024_f64) * 8_f64))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    
    // transmit mbit
    let min_transmit_mbit = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.transmit_bytes > 0_f64)
        .map(|networkdevice| (networkdevice.transmit_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_transmit_mbit = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.transmit_bytes > 0_f64)
        .map(|networkdevice| (networkdevice.transmit_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.transmit_bytes > 0_f64)
                                                .map(|networkdevice| Circle::new((networkdevice.timestamp, networkdevice.transmit_bytes / (1024_f64 * 1024_f64) * 8_f64), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "transmit", min_transmit_mbit, max_transmit_mbit, latest.transmit_bytes / (1024_f64 * 1024_f64) * 8_f64))
        .legend(move |(x, y)| Circle::new((x , y), 4, RED.filled()));
    //
    // receive mbit
    let min_receive_mbit = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.receive_bytes > 0_f64)
        .map(|networkdevice| (networkdevice.receive_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_receive_mbit = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.receive_bytes > 0_f64)
        .map(|networkdevice| (networkdevice.receive_bytes / (1024_f64 * 1024_f64)) * 8_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.receive_bytes > 0_f64)
                                                .map(|networkdevice| Circle::new((networkdevice.timestamp, networkdevice.receive_bytes / (1024_f64 * 1024_f64) * 8_f64), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "receive", min_receive_mbit, max_receive_mbit, latest.receive_bytes / (1024_f64 * 1024_f64) * 8_f64))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));

    // legend
    contextarea.configure_series_labels()
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
)
{
    let historical_data_read = HISTORY.networkdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .min()
        .unwrap_or_default();
    let end_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .max()
        .unwrap_or_default();
    let high_value_packets = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| (networkdevices.receive_packets + networkdevices.transmit_packets) * 1.1_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let latest = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name)
        .last()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Networkdevice: {} packets per second", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..high_value_packets)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Packets per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .take(1)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.transmit_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // total packets
    let min_total_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && (networkdevice.transmit_packets + networkdevice.receive_packets) > 0_f64)
        .map(|networkdevice| networkdevice.transmit_packets + networkdevice.receive_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_total_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && (networkdevice.transmit_packets + networkdevice.receive_packets) > 0_f64)
        .map(|networkdevice| networkdevice.transmit_packets + networkdevice.receive_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name)
                                                .map(|networkdevice| (networkdevice.timestamp, networkdevice.transmit_packets + networkdevice.receive_packets)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "transmit", min_total_packets, max_total_packets, (latest.transmit_packets + latest.receive_packets)))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // transmit packets
    let min_transmit_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.transmit_packets > 0_f64)
        .map(|networkdevice| networkdevice.transmit_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_transmit_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.transmit_packets > 0_f64)
        .map(|networkdevice| networkdevice.transmit_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name)
                                                .map(|networkdevice| Circle::new((networkdevice.timestamp, networkdevice.transmit_packets), 4, RED.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "transmit", min_transmit_packets, max_transmit_packets, latest.transmit_packets))
        .legend(move |(x, y)| Circle::new((x, y), 4, RED.filled()));

    // receive packets
    let min_receive_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.receive_packets > 0_f64)
        .map(|networkdevice| networkdevice.receive_packets)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    let max_receive_packets = historical_data_read
        .iter()
        .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.receive_packets > 0_f64)
        .map(|networkdevice| networkdevice.receive_packets)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or_default();
    contextarea.draw_series(historical_data_read.iter()
                                                .filter(|networkdevice| networkdevice.device_name == device_name)
                                                .map(|networkdevice| Circle::new((networkdevice.timestamp, networkdevice.receive_packets), 3, GREEN.filled())))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "receive", min_receive_packets, max_receive_packets, latest.receive_packets))
        .legend(move |(x, y)| Circle::new((x, y), 3, GREEN.filled()));
    // legend
    contextarea.configure_series_labels()
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
)
{
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
    let historical_data_read = HISTORY.networkdevices.read().unwrap();

    let start_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|networkdevices| networkdevices.device_name == device_name)
        .map(|networkdevices| networkdevices.timestamp)
        .max()
        .unwrap();
    let mut low_value: LowValue = Default::default();
    let mut high_value: HighValue = Default::default();
    macro_rules! read_history_and_set_high_and_low_values {
        ($($struct_field_name:ident),*) => {
            $(
            low_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|networkdevices| networkdevices.device_name == device_name)
                .map(|pressure| pressure.$struct_field_name)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            high_value.$struct_field_name = historical_data_read
                .iter()
                .filter(|networkdevices| networkdevices.device_name == device_name)
                .map(|pressure| pressure.$struct_field_name)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            )*
        };
    }
    read_history_and_set_high_and_low_values!(receive_errors, transmit_errors, transmit_collisions, receive_drop, transmit_drop, transmit_carrier, receive_fifo, transmit_fifo);
    let high_value_overall = [high_value.receive_errors, high_value.transmit_errors, high_value.transmit_collisions, high_value.receive_drop, high_value.transmit_drop, high_value.transmit_carrier, high_value.receive_fifo, high_value.transmit_fifo]
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .copied()
        .unwrap();
    let latest = historical_data_read.iter()
        .filter(|networkdevice| networkdevice.device_name == device_name)
        .last()
        .unwrap();

    // create the plot
    multi_backend[backend_number].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[backend_number])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Networkdevice: {} errors per second", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, 0_f64..(high_value_overall * 1.1_f64))
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Errors per second")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read
                                                .iter()
                                                .take(1)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.transmit_bytes)), ShapeStyle { color: TRANSPARENT, filled: false, stroke_width: 1 }))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    let mut colour_picker = 3_usize;
    macro_rules! draw_lineseries {
        ($($struct_field_name:ident),*) => {
            $(
                contextarea.draw_series(historical_data_read.iter()
                                                            .filter(|networkdevice| networkdevice.device_name == device_name && networkdevice.$struct_field_name > 0_f64)
                                                            .map(|networkdevice| Circle::new((networkdevice.timestamp, networkdevice.$struct_field_name), 4, Palette99::pick(colour_picker).filled())))
                .unwrap()
                .label(format!("{:25} {:10.2} {:10.2} {:10.2}", stringify!($struct_field_name), low_value.$struct_field_name, high_value.$struct_field_name, latest.$struct_field_name))
                .legend(move |(x, y)| Circle::new((x, y), 4, Palette99::pick(colour_picker).filled()));

                colour_picker += 1;
            )*
        };
    }
    draw_lineseries!(receive_errors, transmit_errors, transmit_collisions, receive_drop, transmit_drop, transmit_carrier, receive_fifo, transmit_fifo);
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
}
