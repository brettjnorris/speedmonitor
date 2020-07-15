use std::env;
use std::path::PathBuf;

use speedmonitor::Config;

fn main() {
    let config = Config::new();
    speedmonitor::process(PathBuf::from(config.ingest_dir))
}

