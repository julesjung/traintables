mod routes;
mod service_days;
mod stop_times;
mod stops;
mod trips;

use crate::{
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
use traintables_core::error::Result;
use zip::ZipArchive;

async fn fetch_gtfs_files(gtfs_url: &str) -> Result<HashMap<String, Vec<u8>>> {
    let data = reqwest::get(gtfs_url).await?.bytes().await?;

    let reader = Cursor::new(data.to_vec());
    let mut archive = ZipArchive::new(reader)?;

    let mut data = HashMap::new();

    for file_name in 0..archive.len() {
        let mut file = archive.by_index(file_name)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(file.size() as usize);
        file.read_to_end(&mut buffer)?;
        data.insert(String::from(file.name()), buffer);
    }

    Ok(data)
}

#[tokio::main]
async fn main() -> Result<()> {
    let gtfs_url =
        "https://eu.ftp.opendatasoft.com/sncf/plandata/Export_OpenData_SNCF_GTFS_NewTripId.zip";

    let files: HashMap<String, Vec<u8>> = fetch_gtfs_files(gtfs_url).await?;

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
