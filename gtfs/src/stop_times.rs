use std::{collections::HashMap, io::Cursor};

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
