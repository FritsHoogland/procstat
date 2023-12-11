//use proc_sys_parser;
use time::Duration;
use tokio::time;
use std::collections::HashMap;

use procstat::{read_proc_data, process_data, Statistic, print_cpu};
#[tokio::main]
async fn main()
{
    let mut interval = time::interval(Duration::from_secs(2));
    let mut statistics: HashMap<(String, String), Statistic> = HashMap::new();
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;
        process_data(data, &mut statistics).await;

        print_cpu(&statistics).await;
    }
}
