use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    speedmonitor::process(current_dir)
}

