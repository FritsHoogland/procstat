use time::Duration;
use tokio::time;
use std::collections::HashMap;

use procstat::{read_proc_data, process_data, Statistic, cpu};
use cpu::print_per_cpu;

#[tokio::main]
async fn main()
{
    let mut interval = time::interval(Duration::from_millis(500));
    let mut statistics: HashMap<(String, String, String), Statistic> = HashMap::new();
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;
        process_data(data, &mut statistics).await;

        print_per_cpu(&statistics).await;
    }
}
