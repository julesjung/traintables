use std::io::Cursor;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: u8,
}

pub fn parse_stop_times(data: &Vec<u8>) -> Result<Vec<StopTime>> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut trips: Vec<StopTime> = Vec::new();

    for trip in reader.deserialize() {
        let trip: StopTime = trip?;

        trips.push(trip);
    }

    Ok(trips)
}

pub fn insert_stop_times(connection: &mut Connection, stop_times: Vec<StopTime>) -> Result<()> {
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction
            .prepare("INSERT INTO stop_times (trip_id, arrival_time, departure_time, stop_id, stop_sequence) VALUES (?1, ?2, ?3, ?4, ?5)")?;

        for stop_time in stop_times {
            statement.execute(params![
                stop_time.trip_id,
                stop_time.arrival_time,
                stop_time.departure_time,
                stop_time.stop_id,
                stop_time.stop_sequence
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}
