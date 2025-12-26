use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

use crate::stop_times::Endpoints;

#[derive(Deserialize)]
pub struct GTFSTrip {
    #[serde(rename = "trip_id")]
    pub id: String,
    pub route_id: String,
    pub service_id: u32,
    #[serde(rename = "trip_headsign")]
    pub headsign: String,
    #[serde(rename = "direction_id")]
    pub direction: Option<u8>,
}

#[derive(Serialize)]
pub struct Trip {
    pub id: String,
    pub route_id: String,
    pub service_id: u32,
    pub headsign: String,
    pub direction: Option<u8>,
    pub origin_id: String,
    pub destination_id: String,
}

impl Trip {
    fn new(gtfs_trip: GTFSTrip, origin_id: String, destination_id: String) -> Self {
        Self {
            id: gtfs_trip.id,
            route_id: gtfs_trip.route_id,
            service_id: gtfs_trip.service_id,
            headsign: gtfs_trip.headsign,
            direction: gtfs_trip.direction,
            origin_id,
            destination_id,
        }
    }
}

pub fn parse_trips(data: &Vec<u8>, endpoints: HashMap<String, Endpoints>) -> Result<Vec<Trip>> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut trips: Vec<Trip> = Vec::new();

    for trip in reader.deserialize() {
        let gtfs_trip: GTFSTrip = trip?;

        let endpoint = endpoints.get(&gtfs_trip.id).unwrap();

        trips.push(Trip::new(
            gtfs_trip,
            endpoint.origin.clone(),
            endpoint.destination.clone(),
        ));
    }

    Ok(trips)
}

pub fn generate_trips_csv(trips: Vec<Trip>, path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for trip in trips {
        writer.serialize(trip)?;
    }

    writer.flush()?;

    Ok(())
}
