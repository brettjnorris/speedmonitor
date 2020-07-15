use std::{fs};
use std::path::{PathBuf};
use std::error::Error;
use std::env;

use csv::{StringRecord, ReaderBuilder};
use influxdb::{Client, Query};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};

use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
#[derive(InfluxDbWriteable)]
pub struct Measurement {
    time: DateTime<Utc>,
    ping: f32,
    download_rate: u32,
    upload_rate: u32,
    #[tag] server: String
}

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

pub fn process(dir: PathBuf) {
    let query = String::from(".csv");
    let files = find_files(dir, query).unwrap();

    for file in files {
        parse_contents(file).unwrap();
    }
}

impl Measurement {
    pub fn from_csv(record: StringRecord) -> Result<Measurement, Box<dyn Error>> {
        Ok(Measurement {
            time: Utc::now(),
            server: record[0].to_string(),
            ping: record[2].to_string().parse::<f32>().unwrap(),
            download_rate: record[5].to_string().parse::<u32>().unwrap(),
            upload_rate: record[6].to_string().parse::<u32>().unwrap(),
        })
    }
}

fn find_files(path: PathBuf, query: String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut matches = vec![];

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            let path_str = path.to_str().unwrap();

            if let true = path_str.contains(&query) {
                matches.push(path_str.to_string());
            }
        }
    }

    Ok(matches)
}


fn parse_contents(path: String) -> Result<Vec<Measurement>, Box<dyn Error>> {
    let mut measurements = vec![];
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(path)?;

    for result in rdr.records() {
        let record = result?;
        let measurement = Measurement::from_csv(record).unwrap();

        Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(write_measurement(measurement.clone()));

        measurements.push(measurement);
    }

    Ok(measurements)
}

async fn write_measurement(measurement: Measurement) {
    let config = Config::new();
    let client = Client::new(config.influxdb_host, config.influxdb_database);

    let write_result = client.query(&measurement.into_query("rates")).await;
    assert!(write_result.is_ok(), "Write result was not okay");

    let read_query = Query::raw_read_query("SELECT * FROM rates");
    let read_result = client.query(&read_query).await;
    assert!(read_result.is_ok(), "Read result was not okay");
    println!("{}", read_result.unwrap());
}