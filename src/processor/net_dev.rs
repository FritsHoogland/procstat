
use std::collections::{BTreeSet, HashMap};
use log::debug;
use chrono::{DateTime, Local};
use proc_sys_parser::net_dev::ProcNetDev;
use serde::{Serialize, Deserialize};
//
use crate::processor::{ProcData, single_statistic_u64, Statistic};
use crate::HISTORY;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

pub async fn read_netdev_proc_data() -> ProcNetDev {
    let proc_netdev = proc_sys_parser::net_dev::read();
    debug!("{:?}", proc_netdev);
    proc_netdev
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
