use proc_sys_parser;
use time::Duration;
use tokio::time;

use procstat::{ProcData, read_proc_data};
#[tokio::main]
async fn main()
{
    let mut interval = time::interval(Duration::from_secs(2));
    loop
    {
        interval.tick().await;

        let data = read_proc_data().await;

        println!("{:?}", data);
    }
}
