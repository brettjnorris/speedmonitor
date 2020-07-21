use std::env;

pub struct Config {
    pub influxdb_host: String,
    pub influxdb_database: String,
    pub ingest_dir: String
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

        Config { influxdb_host, influxdb_database, ingest_dir }
    }
}