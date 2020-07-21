use std::error::Error;

use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};
use csv::{StringRecord};

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
        Ok(Measurement {
            time: Utc::now(),
            server: record[0].to_string(),
            ping: record[2].to_string().parse::<f32>().unwrap(),
            download_rate: record[5].to_string().parse::<u32>().unwrap(),
            upload_rate: record[6].to_string().parse::<u32>().unwrap(),
        })
    }
}