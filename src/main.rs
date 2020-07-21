use std::path::PathBuf;

mod config;
use config::Config;

fn main() {
    let config = Config::new();
    speedmonitor::process(PathBuf::from(config.ingest_dir))
}

