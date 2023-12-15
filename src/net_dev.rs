use std::collections::HashMap;
use crate::{ProcData, single_statistic, Statistic};

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