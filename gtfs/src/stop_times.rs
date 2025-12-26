use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

#[derive(Deserialize, Serialize)]
pub struct StopTime {
    pub trip_id: String,
    pub stop_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_sequence: u8,
}

pub struct Endpoints {
    pub origin: String,
    pub destination: String,
}

impl Endpoints {
    fn new(origin: String) -> Self {
        Self {
            origin: origin.clone(),
            destination: origin,
        }
    }
}

pub fn parse_stop_times(data: &Vec<u8>) -> Result<(Vec<StopTime>, HashMap<String, Endpoints>)> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut stop_times: Vec<StopTime> = Vec::new();
    let mut endpoints: HashMap<String, Endpoints> = HashMap::new();

    for stop_time in reader.deserialize() {
        let stop_time: StopTime = stop_time?;

        endpoints
            .entry(stop_time.trip_id.clone())
            .or_insert(Endpoints::new(stop_time.stop_id.clone()))
            .destination = stop_time.stop_id.clone();

        stop_times.push(stop_time);
    }

    Ok((stop_times, endpoints))
}

pub fn generate_stop_times_csv(stop_times: Vec<StopTime>, path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for stop_time in stop_times {
        writer.serialize(stop_time)?;
    }

    writer.flush()?;

    Ok(())
}
