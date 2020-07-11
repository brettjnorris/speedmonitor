use std::env;
use std::{fs};
use std::path::{PathBuf};
use std::error::Error;

use csv::{StringRecord, ReaderBuilder};

fn main() {
    let path = env::current_dir().unwrap();
    let query = String::from(".csv");

    println!("current directory: {}", path.display());

    let files = find_files(path, query).unwrap();

    for file in files {
        let measurement = parse_contents(file).unwrap();

        println!("{:?}", measurement);
    }
}

#[derive(Debug)]
struct Measurement {
    timestamp: String,
    server: String,
    ping: f32,
    download_rate: u32,
    upload_rate: u32,
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
        let measurement = parse_record(record)?;

        measurements.push(measurement);
    }

    Ok(measurements)
}

fn parse_record(record: StringRecord) -> Result<Measurement, Box<dyn Error>> {
    let timestamp = String::from("2020-07-11T17:00:00");
    let server = record[0].to_string();
    let ping = record[2].to_string().parse::<f32>().unwrap();
    let download_rate = record[5].to_string().parse::<u32>().unwrap();
    let upload_rate = record[6].to_string().parse::<u32>().unwrap();

    let measurement = Measurement {
        timestamp,
        server,
        ping,
        download_rate,
        upload_rate,
    };

    Ok(measurement)
}
