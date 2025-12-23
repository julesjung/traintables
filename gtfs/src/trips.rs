use std::io::Cursor;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Trip {
    #[serde(rename = "trip_id")]
    pub id: String,
    pub route_id: String,
    #[serde(rename = "trip_headsign")]
    pub headsign: u32,
    #[serde(rename = "direction_id")]
    pub direction: Option<u8>,
}

pub fn parse_trips(data: &Vec<u8>) -> Result<Vec<Trip>> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut trips: Vec<Trip> = Vec::new();

    for trip in reader.deserialize() {
        let trip: Trip = trip?;

        trips.push(trip);
    }

    Ok(trips)
}

pub fn insert_trips(connection: &mut Connection, trips: Vec<Trip>) -> Result<()> {
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction.prepare(
            "INSERT INTO trips (id, route_id, headsign, direction) VALUES (?1, ?2, ?3, ?4)",
        )?;

        for trip in trips {
            statement.execute(params![
                trip.id,
                trip.route_id,
                trip.headsign,
                trip.direction,
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}
