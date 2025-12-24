use std::collections::HashMap;

use anyhow::Result;
use rusqlite::Connection;

use crate::{
    routes::{Route, insert_routes},
    service_days::{ServiceDay, insert_services},
    stop_times::{StopTime, insert_stop_times},
    stops::{Station, StopPoint, insert_stations, insert_stop_points},
    trips::{Trip, insert_trips},
};

pub fn create_database(
    stations: Vec<Station>,
    stop_points: Vec<StopPoint>,
    routes: Vec<Route>,
    trips: Vec<Trip>,
    stop_times: Vec<StopTime>,
    services: HashMap<u32, Vec<ServiceDay>>,
    path: &str,
) -> Result<()> {
    let mut connection = Connection::open(path)?;

    connection.execute_batch(include_str!("../sql/schema.sql"))?;
    connection.execute_batch(include_str!("../sql/indexes.sql"))?;

    insert_stations(&mut connection, stations)?;
    insert_stop_points(&mut connection, stop_points)?;
    insert_routes(&mut connection, routes)?;
    insert_services(&mut connection, services)?;
    insert_trips(&mut connection, trips)?;
    insert_stop_times(&mut connection, stop_times)?;

    connection.execute_batch(include_str!("../sql/fts.sql"))?;
    connection.execute_batch(include_str!("../sql/cleaning.sql"))?;

    Ok(())
}
