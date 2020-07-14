use std::{fs};
use std::path::{PathBuf};
use std::error::Error;

use csv::{StringRecord, ReaderBuilder};
use influxdb::{Client, Query, Timestamp};
use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};
use futures::executor::block_on;

use tokio::runtime::Runtime;

pub fn process(dir: PathBuf) {
    let query = String::from(".csv");
    let files = find_files(dir, query).unwrap();

    for file in files {
        parse_contents(file).unwrap();
    }
}

#[derive(Clone, Debug)]
#[derive(InfluxDbWriteable)]
pub struct Measurement {
    time: DateTime<Utc>,
    ping: f32,
    download_rate: u32,
    upload_rate: u32,
    #[tag] server: String
}

impl Measurement {
    pub fn from_csv(record: StringRecord) -> Result<Measurement, Box<dyn Error>> {
        let time = Utc::now();
        let server = record[0].to_string();
        let ping = record[2].to_string().parse::<f32>().unwrap();
        let download_rate = record[5].to_string().parse::<u32>().unwrap();
        let upload_rate = record[6].to_string().parse::<u32>().unwrap();

        let measurement = Measurement {
            time,
            server,
            ping,
            download_rate,
            upload_rate,
        };

        println!("{:?}", measurement);

        Ok(measurement)
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
    let client = Client::new("http://localhost:8086", "speedmonitor");

    let write_result = client.query(&measurement.into_query("rates")).await;
    assert!(write_result.is_ok(), "Write result was not okay");

    let read_query = Query::raw_read_query("SELECT * FROM rates");
    let read_result = client.query(&read_query).await;
    assert!(read_result.is_ok(), "Read result was not okay");
    println!("{}", read_result.unwrap());
}