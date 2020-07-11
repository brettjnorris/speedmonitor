use std::env;
use std::{fs, io};
use std::path::PathBuf;
use std::fs::DirEntry;

fn main() {
    let path = env::current_dir().unwrap();
    println!("current_dir: {}", path.display());

    let files = find_files(path).unwrap();
}

fn find_files(path: PathBuf) -> Result<Vec<DirEntry>, &'static str> {
    let mut matches = vec![];

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let query= String::from(".csv");

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

fn read_file(path: PathBuf) ->
