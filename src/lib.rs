use std::{fs};
use std::path::{Path, PathBuf};
use std::error::Error;

use csv::ReaderBuilder;
use tokio::runtime::Runtime;
use influxdb::{Client, Query};
use influxdb::InfluxDbWriteable;

mod config;
use config::Config;

mod models;
use models::Measurement;

pub fn process(dir: PathBuf) {
    let query = String::from(".csv");
    let files = find_files(dir, query).unwrap();

    println!("{:?}", files);

    for file in files {
        match parse_contents(file) {
            Ok(_result) => println!("file processed"),
            Err(e) => println!("Error processing file: {}", e)
        }
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
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(&path)?;

    for result in rdr.records() {
        let record = result?;
        let measurement = Measurement::from_csv(record).unwrap();

        Runtime::new()
            .expect("Failed to create tokio runtime")
            .block_on(write_measurement(measurement.clone()));

        remove_file(path.clone()).unwrap();
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
}

fn remove_file(path_string: String) -> Result<String, Box<dyn Error>> {
    let path = Path::new(&path_string);
    let file_name = path.file_name().unwrap();

    let new_path = path
        .parent()
        .unwrap()
        .join("processed")
        .join(file_name);

    fs::rename(path_string, new_path).unwrap();

    Ok(String::from("file deleted"))
}
