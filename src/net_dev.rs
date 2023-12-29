use std::collections::{BTreeSet, HashMap};
use chrono::{DateTime, Local};
use plotters::backend::BitMapBackend;
use plotters::chart::{ChartBuilder, LabelAreaPosition};
use plotters::chart::SeriesLabelPosition::UpperLeft;
use plotters::element::Rectangle;
use plotters::prelude::{AreaSeries, BLACK, GREEN, LineSeries, RED, ShapeStyle, TRANSPARENT, WHITE};
use plotters::prelude::full_palette::PURPLE;
use crate::common::{ProcData, single_statistic_u64, Statistic};
use crate::{CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE, HISTORY, LABEL_AREA_SIZE_BOTTOM, LABEL_AREA_SIZE_LEFT, LABEL_AREA_SIZE_RIGHT, LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE, MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE};

#[derive(Debug)]
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

    if !statistics.get(&("net_dev".to_string(), nic_list[0].to_string(), "receive_bytes".to_string())).unwrap().updated_value { return };

    let mut totals = vec![0_f64; 16];

    let timestamp = statistics.get(&("net_dev".to_string(), nic_list[0].to_string(), "receive_bytes".to_string())).unwrap().last_timestamp;

    for network_interface in nic_list.iter().filter(|interface_name| ! interface_name.starts_with("lo") )
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
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
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
            println!("{:10} {:7}    {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}",
                     "Timestamp",
                     "IFACE",
                     "rxerr/s",
                     "txerr/s",
                     "coll/s",
                     "rxdrop/s",
                     "txdrop/s",
                     "txcarr/s",
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
        let receive_fifo = statistics.get(&("net_dev".to_string(), device.to_string(), "receive_fifo".to_string())).unwrap().per_second_value;
        let transmit_fifo = statistics.get(&("net_dev".to_string(), device.to_string(), "transmit_fifo".to_string())).unwrap().per_second_value;

        match output
        {
            "sar-n-DEV" => {
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
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
                println!("{:10} {:7}    {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2} {:9.2}",
                         timestamp.format("%H:%M:%S"),
                         device,
                         receive_errors,
                         transmit_errors,
                         transmit_collisions,
                         receive_drop,
                         transmit_drop,
                         transmit_carrier,
                         receive_fifo,
                         transmit_fifo,
                );
            }
            &_ => todo!(),
        }
    }
}

pub fn create_networkdevice_plot(
    buffer: &mut Vec<u8>,
    device_name: String,
)
{
    /*
    let backend = BitMapBackend::with_buffer(buffer, (1280,900)).into_drawing_area();
    let multi_backend = backend.split_evenly((3,1));

    // MBPS plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| (blockdevices.reads_bytes + blockdevices.writes_bytes) / (1024_f64*1024_f64))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[0].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[0])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} MBPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("MBPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write MBPS
    // this is a stacked graph, so write MBPS = write + read
    let min_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_bytes / (1024_f64 * 1024_f64)).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_write_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_bytes / (1024_f64 * 1024_f64);
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, (blockdevice.writes_bytes + blockdevice.reads_bytes) / (1024_f64 * 1024_f64))), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write", min_write_mbps, max_write_mbps, latest_write_mbps))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read MBPS
    let min_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64)).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_bytes / (1024_f64 * 1024_f64)).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_read_mbps = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_bytes / (1024_f64 * 1024_f64);
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes / (1024_f64 * 1024_f64))), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read", min_read_mbps, max_read_mbps, latest_read_mbps))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
    //
    // IOPS plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value: f64 = 0.0;
    let high_value = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.reads_completed_success + blockdevices.flush_requests_completed_success + blockdevices.discards_completed_success)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[1].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[1])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} IOPS", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value..high_value)
        .unwrap();
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("IOPS")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10} {:>10}", "", "min", "max", "last"));
    //
    // write IOPS
    // this is a stacked graph, so write IOPS = write + read + discard
    let min_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.writes_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_write_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().writes_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.writes_completed_success + blockdevice.reads_completed_success + blockdevice.discards_completed_success)), 0.0, RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "write IOPS", min_write_iops, max_write_iops, latest_write_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    // read IOPS
    let min_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.reads_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_read_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().reads_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_completed_success + blockdevice.discards_completed_success)), 0.0, GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "read IOPS", min_read_iops, max_read_iops, latest_read_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    // discards IOPS
    let min_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.discards_completed_success).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).map(|blockdevice| blockdevice.discards_completed_success).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let latest_discard_iops = historical_data_read.iter().filter(|blockdevice| blockdevice.device_name == device_name).last().unwrap().discards_completed_success;
    contextarea.draw_series(AreaSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| (blockdevice.timestamp, blockdevice.discards_completed_success)), 0.0, PURPLE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2} {:10.2}", "discard IOPS", min_discard_iops, max_discard_iops, latest_discard_iops))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();
    //
    // read, write and discard latency and queue depth plot
    let historical_data_read = HISTORY.blockdevices.read().unwrap();
    let start_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .min()
        .unwrap();
    let end_time = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.timestamp)
        .max()
        .unwrap();
    let low_value_latencies: f64 = 0.0;
    let high_value_latencies_read = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.reads_time_spent_ms / blockdevices.reads_completed_success).is_nan() { 0_f64 } else { blockdevices.reads_time_spent_ms / blockdevices.reads_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies_write = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.writes_time_spent_ms / blockdevices.writes_completed_success).is_nan() { 0_f64 } else { blockdevices.writes_time_spent_ms / blockdevices.writes_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies_discard = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices|  if (blockdevices.discards_time_spent_ms / blockdevices.discards_completed_success).is_nan() { 0_f64 } else { blockdevices.discards_time_spent_ms / blockdevices.discards_completed_success } )
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let high_value_latencies = high_value_latencies_read.max(high_value_latencies_write).max(high_value_latencies_discard);
    let low_value_queue_depth = 0_f64;
    let high_value_queue_depth = historical_data_read
        .iter()
        .filter(|blockdevices| blockdevices.device_name == device_name)
        .map(|blockdevices| blockdevices.ios_weighted_time_spent_ms / 1000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // create the plot
    multi_backend[2].fill(&WHITE).unwrap();
    let mut contextarea = ChartBuilder::on(&multi_backend[2])
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE_BOTTOM)
        .set_label_area_size(LabelAreaPosition::Right, LABEL_AREA_SIZE_RIGHT)
        .caption(format!("Blockdevice: {} latency and queue depth", device_name), (CAPTION_STYLE_FONT, CAPTION_STYLE_FONT_SIZE))
        .build_cartesian_2d(start_time..end_time, low_value_latencies..high_value_latencies)
        .unwrap()
        .set_secondary_coord(start_time..end_time, low_value_queue_depth..high_value_queue_depth);
    contextarea.configure_mesh()
        .x_labels(6)
        .x_label_formatter(&|timestamp| timestamp.format("%Y-%m-%dT%H:%M:%S").to_string())
        .x_desc("Time")
        .y_desc("Average latency ms")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    contextarea.configure_secondary_axes()
        .y_desc("queue depth")
        .label_style((MESH_STYLE_FONT, MESH_STYLE_FONT_SIZE))
        .draw()
        .unwrap();
    //
    // This is a dummy plot for the sole intention to write a header in the legend.
    contextarea.draw_series(LineSeries::new(historical_data_read.iter().take(1).map(|blockdevice| (blockdevice.timestamp, blockdevice.reads_bytes)), ShapeStyle { color: TRANSPARENT.into(), filled: false, stroke_width: 1} ))
        .unwrap()
        .label(format!("{:25} {:>10} {:>10}", "", "min", "max"));
    //
    // write latency
    // this is a line graph, so no stacking.
    let min_write_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_write_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { 0_f64 } else { blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| if (blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.writes_time_spent_ms / blockdevice.writes_completed_success) }), RED))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2}", "avg. write latency", min_write_latency, max_write_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], RED.filled()));
    //
    // read latency
    // this is a line graph, so no stacking.
    let min_read_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_read_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { 0_f64 } else { blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| if (blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.reads_time_spent_ms / blockdevice.reads_completed_success) }), GREEN))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2}", "avg. read latency", min_read_latency, max_read_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], GREEN.filled()));
    //
    // discard latency
    // this is a line graph, so no stacking.
    let min_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_discard_latency = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { 0_f64 } else { blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success })
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_series(LineSeries::new(historical_data_read.iter()
                                                .filter(|blockdevice| blockdevice.device_name == device_name)
                                                .map(|blockdevice| if (blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success).is_nan() { (blockdevice.timestamp, 0_f64) } else { (blockdevice.timestamp, blockdevice.discards_time_spent_ms / blockdevice.discards_completed_success) }), PURPLE))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2}", "avg. discard latency", min_discard_latency, max_discard_latency))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], PURPLE.filled()));
    //
    // queue depth
    // this is a line graph, and it is bound to the right-hand y axis!
    let min_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_queue_depth = historical_data_read.iter()
        .filter(|blockdevice| blockdevice.device_name == device_name)
        .map(|blockdevice| blockdevice.ios_weighted_time_spent_ms / 1000_f64)
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    contextarea.draw_secondary_series(LineSeries::new(historical_data_read.iter()
                                                          .filter(|blockdevice| blockdevice.device_name == device_name)
                                                          .map(|blockdevice| (blockdevice.timestamp, blockdevice.ios_weighted_time_spent_ms / 1000_f64)), BLACK))
        .unwrap()
        .label(format!("{:25} {:10.2} {:10.2}", "avg. queue depth", min_queue_depth, max_queue_depth))
        .legend(move |(x, y)| Rectangle::new([(x - 3, y - 3), (x + 3, y + 3)], BLACK.filled()));
    // legend
    contextarea.configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.7))
        .label_font((LABELS_STYLE_FONT, LABELS_STYLE_FONT_SIZE))
        .position(UpperLeft)
        .draw()
        .unwrap();

     */
}
