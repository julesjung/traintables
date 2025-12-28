use crate::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Station {
    fn new(stop: Stop) -> Self {
        Self {
            id: stop.stop_id,
            name: stop.stop_name,
            latitude: stop.stop_lat,
            longitude: stop.stop_lon,
        }
    }
}

#[derive(Serialize)]
pub struct StopPoint {
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub station_id: String,
}

impl StopPoint {
    fn new(stop: Stop) -> Self {
        Self {
            id: stop.stop_id,
            name: stop.stop_name,
            latitude: stop.stop_lat,
            longitude: stop.stop_lon,
            station_id: stop.parent_station,
        }
    }
}

#[derive(Deserialize)]
struct Stop {
    stop_id: String,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
    location_type: u8,
    parent_station: String,
}

pub fn parse_stops(data: &Vec<u8>) -> Result<(Vec<Station>, Vec<StopPoint>)> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut stations: Vec<Station> = Vec::new();
    let mut stop_points: Vec<StopPoint> = Vec::new();

    for stop in reader.deserialize() {
        let stop: Stop = stop?;

        if stop.location_type == 0 {
            stop_points.push(StopPoint::new(stop));
        } else if stop.location_type == 1 {
            stations.push(Station::new(stop));
        }
    }

    Ok((stations, stop_points))
}

pub fn generate_stations_csv(stations: Vec<Station>, path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for station in stations {
        writer.serialize(station)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn generate_stops_csv(stops: Vec<StopPoint>, path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for stop in stops {
        writer.serialize(stop)?;
    }

    writer.flush()?;

    Ok(())
}
