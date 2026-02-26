mod error;
mod routes;
mod service_days;
mod stop_times;
mod stops;
mod trips;

use zip::ZipArchive;

use crate::{
    error::Result,
    routes::{generate_routes_csv, parse_routes},
    service_days::{generate_services_csv, parse_services},
    stop_times::{generate_stop_times_csv, parse_stop_times},
    stops::{generate_stations_csv, generate_stops_csv, parse_stops},
    trips::{generate_trips_csv, parse_trips},
};
use std::{
    collections::HashMap,
    fs::create_dir_all,
    io::{Cursor, Read},
};

async fn unzip(data: Vec<u8>) -> Result<HashMap<String, Vec<u8>>> {
    let reader = Cursor::new(data.to_vec());
    let mut archive = ZipArchive::new(reader)?;

    let mut files = HashMap::new();

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buffer)?;
        files.insert(String::from(file.name()), buffer);
    }

    Ok(files)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url =
        "https://eu.ftp.opendatasoft.com/sncf/plandata/Export_OpenData_SNCF_GTFS_NewTripId.zip";
    let data = reqwest::get(url).await?.bytes().await?.to_vec();
    let files: HashMap<String, Vec<u8>> = unzip(data).await?;

    let (stations, stop_points) = parse_stops(files.get("stops.txt").unwrap())?;
    let routes = parse_routes(files.get("routes.txt").unwrap())?;
    let (stop_times, endpoints) = parse_stop_times(files.get("stop_times.txt").unwrap())?;
    let trips = parse_trips(files.get("trips.txt").unwrap(), endpoints)?;
    let services = parse_services(files.get("calendar_dates.txt").unwrap())?;

    create_dir_all("build")?;

    generate_stations_csv(stations, "build/stations.csv")?;
    generate_stops_csv(stop_points, "build/stops.csv")?;
    generate_routes_csv(routes, "build/routes.csv")?;
    generate_trips_csv(trips, "build/trips.csv")?;
    generate_stop_times_csv(stop_times, "build/stop_times.csv")?;
    generate_services_csv(services, "build/services.csv", "build/service_days.csv")?;

    Ok(())
}
