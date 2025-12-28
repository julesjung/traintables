use crate::Result;
use serde::{Deserialize, Serialize, de};
use std::{collections::HashMap, io::Cursor};

#[derive(Deserialize, Serialize)]
pub struct StopTime {
    pub trip_id: String,
    pub stop_id: String,
    #[serde(
        deserialize_with = "deserialize_time",
        rename(deserialize = "arrival_time")
    )]
    pub arrival_seconds: u32,
    #[serde(
        deserialize_with = "deserialize_time",
        rename(deserialize = "departure_time")
    )]
    pub departure_seconds: u32,
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

fn deserialize_time<'de, D>(deserializer: D) -> std::result::Result<u32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let time: &str = Deserialize::deserialize(deserializer).unwrap();
    let mut time = time.split(":");
    let hours: u32 = time.next().unwrap().parse().map_err(de::Error::custom)?;
    let minutes: u32 = time.next().unwrap().parse().map_err(de::Error::custom)?;
    let seconds: u32 = time.next().unwrap().parse().map_err(de::Error::custom)?;

    Ok(hours * 3600 + minutes * 60 + seconds)
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
