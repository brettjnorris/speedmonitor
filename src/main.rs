use std::path::PathBuf;

mod config;
use config::Config;
use std::{thread, time};

fn main() {
    let config = Config::new();
    loop {
        speedmonitor::process(PathBuf::from(&config.ingest_dir));

        let fifteen_minutes = time::Duration::from_secs(config.sleep_mins);
        thread::sleep(fifteen_minutes);
    }
}

