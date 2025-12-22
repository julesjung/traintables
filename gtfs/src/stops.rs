use std::io::Cursor;

use anyhow::Result;
use serde::Deserialize;

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

pub struct StopPoint {
    pub id: String,
    pub name: String,
    pub station_id: String,
}

impl StopPoint {
    fn new(stop: Stop) -> Self {
        Self {
            id: stop.stop_id,
            name: stop.stop_name,
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

pub fn parse_stops(data: Vec<u8>) -> Result<(Vec<Station>, Vec<StopPoint>)> {
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
