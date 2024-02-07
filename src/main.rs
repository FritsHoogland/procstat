use tokio::time::Instant;
use std::process;
use log::info;
use env_logger;
use chrono::Local;

// this is a feature-gated option to perform memory usage analysis.
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use procstat::ARGS;
use procstat::archiver::{archiver, archive, reader};
use procstat::app::app;
use procstat::webserver::webserver;


#[tokio::main]
async fn main() {
    #[cfg(feature = "dhat-heap")]
    let mut _profiler = dhat::Profiler::new_heap(); 

    env_logger::init();
    info!("Start procstat");
    let timer = Instant::now();

    // spawn the ctrlc thead
    ctrlc::set_handler(move || { 
        if ARGS.archiver { archive(Local::now()) }
        info!("End procstat, total time: {:?}", timer.elapsed());
        process::exit(0);
    }).unwrap();

    // spawn the webserver 
    if ARGS.webserver || ARGS.read.is_some() {
        tokio::spawn( async move {
            webserver().await;
        });
    }
    // spawn the archiver; only linux
    if ARGS.archiver { 
        tokio::spawn( async move { 
            archiver().await 
        }); 
    };

    // reader function.
    // execution loops in the reader if called.
    if ARGS.read.is_some() { 
        reader(ARGS.read.as_ref().unwrap().to_string()).await; 
    }

    // run the fetching and CLI output.
    app().await;

    info!("End procstat, total time: {:?}", timer.elapsed());
}
