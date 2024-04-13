use anyhow::Result;
use chrono::Local;
use env_logger;
use log::info;
use std::process;
use tokio::time::Instant;

// this is a feature-gated option to perform memory usage analysis.
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use procstat::app::app;
use procstat::archiver::{archive, archiver, reader};
use procstat::webserver::webserver;
use procstat::ARGS;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let mut _profiler = dhat::Profiler::new_heap();

    env_logger::init();
    info!("Start procstat");
    let timer = Instant::now();

    // spawn the ctrlc thead
    ctrlc::set_handler(move || {
        println!("SIGINT received, terminating.");
        let mut return_value = 0;
        if ARGS.archiver {
            // this performs an "emergency write".
            // because the time is not exactly matched to the interval,
            // it is likely to archive data that is archived before.
            // the reader function does not complain about reading already read
            // rows.
            match archive(Local::now()) {
                Ok(_) => {}
                Err(error) => {
                    return_value = 1;
                    eprintln!("{:?}", error);
                }
            }
        }
        info!("End procstat, total time: {:?}", timer.elapsed());
        process::exit(return_value);
    })
    .unwrap();

    // spawn the webserver
    if ARGS.webserver || ARGS.read.is_some() {
        println!("Webserver is started at port: {}", ARGS.webserver_port);
        tokio::spawn(async move {
            webserver().await;
        });
    }
    // spawn the archiver; only linux
    if ARGS.archiver {
        tokio::spawn(async move {
            match archiver().await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{:?}", error);
                    process::exit(1);
                }
            }
        });
    };

    // reader function.
    // execution loops in the reader if called.
    if ARGS.read.is_some() {
        reader(ARGS.read.as_ref().unwrap().to_string()).await;
    }

    // run the fetching and CLI output.
    // in deamon mode, the cli output is skipped and only the data data is fetched.
    app().await?;

    info!("End procstat, total time: {:?}", timer.elapsed());

    Ok(())
}
