use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;

use crate::stop_times::Endpoints;

#[derive(Deserialize)]
pub struct GTFSTrip {
    #[serde(rename = "trip_id")]
    pub id: String,
    pub route_id: String,
    pub service_id: u32,
    #[serde(rename = "trip_headsign")]
    pub headsign: u32,
    #[serde(rename = "direction_id")]
    pub direction: Option<u8>,
}

pub struct Trip {
    pub id: String,
    pub route_id: String,
    pub service_id: u32,
    pub headsign: u32,
    pub direction: Option<u8>,
    pub origin: String,
    pub destination: String,
}

impl Trip {
    fn new(gtfs_trip: GTFSTrip, origin: String, destination: String) -> Self {
        Self {
            id: gtfs_trip.id,
            route_id: gtfs_trip.route_id,
            service_id: gtfs_trip.service_id,
            headsign: gtfs_trip.headsign,
            direction: gtfs_trip.direction,
            origin,
            destination,
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

pub fn insert_trips(connection: &mut Connection, trips: Vec<Trip>) -> Result<()> {
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction.prepare(
            "INSERT INTO trips (id, route_id, service_id, headsign, direction, origin, destination) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;

        for trip in trips {
            statement.execute(params![
                trip.id,
                trip.route_id,
                trip.service_id,
                trip.headsign,
                trip.direction,
                trip.origin,
                trip.destination
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}
