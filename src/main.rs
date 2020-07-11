use std::env;
use std::{fs, io};
use std::path::PathBuf;
use std::fs::DirEntry;
use std::error::Error;

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
    ping: u32,
    download_rate: u32,
    upload_rate: u32,
}

fn find_files(path: PathBuf, query: String) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut matches = vec![];

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_dir() {
            let path = path.to_str().unwrap();

            if let true = path.contains(&query) {
                println!("{:?}", path);
                matches.push(entry);
            }
        }
    }

    Ok(matches)
}

fn parse_contents(path: DirEntry) -> Result<Measurement, Box<dyn Error>> {
    Ok(Measurement {
        timestamp: String::from("2020-07-11T17:00:00"),
        server: String::from("foo"),
        ping: 0,
        download_rate: 0,
        upload_rate: 0
    })
}
