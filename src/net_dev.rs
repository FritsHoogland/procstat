use std::collections::{BTreeSet, HashMap};
use crate::common::{ProcData, single_statistic, Statistic};

pub async fn process_net_dev_data(proc_data: &ProcData, statistics: &mut HashMap<(String, String, String), Statistic>)
{
    for interface in &proc_data.net_dev.interface
    {
        macro_rules! add_net_dev_data_to_statistics {
            ($($field_name:ident),*) => {
                $(
                    single_statistic("net_dev", &interface.name, stringify!($field_name), proc_data.timestamp, interface.$field_name, statistics).await;
                )*
            };
        }
        add_net_dev_data_to_statistics!(receive_bytes, receive_packets, receive_errors, receive_drop, receive_fifo, receive_frame, receive_compressed, receive_multicast, transmit_bytes, transmit_packets, transmit_errors, transmit_drop, transmit_fifo, transmit_collisions, transmit_carrier, transmit_compressed);
    }
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

    if !statistics.get(&("net_dev".to_string(), device_list[0].to_string(), "receive_bytes".to_string())).unwrap().updated_value { return };

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
        },
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
        },
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
            },
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
            },
            &_ => todo!(),
        }
    }
}