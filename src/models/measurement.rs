use std::error::Error;
use std::fmt;

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
        if record.len() < 7 {
            return Err(Box::new(fmt::Error));
        }

        let time = Utc::now();
        let server = record[0].to_string();
        let ping = record[2].to_string().parse::<f32>();
        let download_rate = record[5].to_string().parse::<u32>();
        let upload_rate= record[6].to_string().parse::<u32>();

        match (ping, download_rate, upload_rate)  {
            (Ok(ping), Ok(download_rate), Ok(upload_rate)) => {
                Ok(Measurement {
                    time,
                    server,
                    ping,
                    download_rate,
                    upload_rate,
                })
            },
            _ => Err(Box::new(fmt::Error))
        }
    }
}