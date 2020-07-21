use std::env;

pub struct Config {
    pub influxdb_host: String,
    pub influxdb_database: String,
    pub ingest_dir: String,
    pub sleep_mins: u64
}

impl Config {
    pub fn new() -> Config {
        let influxdb_host = match env::var("INFLUXDB_HOST") {
            Ok(val) => val,
            Err(_e) => String::from("http://localhost:8086")
        };

        let influxdb_database = match env::var("INFLUXDB_DATABASE") {
            Ok(val) => val,
            Err(_e) => String::from("speedmonitor")
        };

        let ingest_dir = match env::var("INGEST_DIR") {
            Ok(val) => val,
            Err(_e) => String::from("./")
        };

        let sleep_mins = match env::var("SLEEP_MINS") {
            Ok(val) => val.parse::<u64>().unwrap(),
            Err(_e) => 15 * 60
        };

        Config { influxdb_host, influxdb_database, ingest_dir, sleep_mins }
    }
}