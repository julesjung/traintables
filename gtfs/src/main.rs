mod database;
mod routes;
mod service_days;
mod stop_times;
mod stops;
mod trips;
mod update;

use anyhow::Result;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::create_dir_all,
    io::{Cursor, Read},
};
use zip::ZipArchive;

use crate::{
    database::create_database, routes::parse_routes, service_days::parse_services,
    stop_times::parse_stop_times, stops::parse_stops, trips::parse_trips,
    update::create_version_file,
};

#[derive(Deserialize)]
struct SNCFOpenDataRecord {
    donnees: String,
    format: String,
    download: String,
}

#[derive(Deserialize)]
struct SNCFOpenDataResponse {
    results: Vec<SNCFOpenDataRecord>,
}

async fn get_gtfs_url() -> Result<String> {
    let response: SNCFOpenDataResponse = reqwest::get(
        "https://ressources.data.sncf.com/api/explore/v2.1/catalog/datasets/horaires-sncf/records",
    )
    .await?
    .json()
    .await?;
    let gtfs_url = response
        .results
        .iter()
        .find(|&record| {
            record.format == "GTFS" && record.donnees == "Horaires des lignes SNCF théorique"
        })
        .unwrap()
        .download
        .clone();

    Ok(gtfs_url)
}

async fn fetch_gtfs_files(gtfs_url: &str) -> Result<HashMap<String, Vec<u8>>> {
    let data = reqwest::get(gtfs_url).await?.bytes().await?;

    let reader = Cursor::new(data.to_vec());
    let mut archive = ZipArchive::new(reader)?;

    let mut data: HashMap<String, Vec<u8>> = HashMap::new();

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
    let gtfs_url = get_gtfs_url().await?;

    let files = fetch_gtfs_files(gtfs_url.as_str()).await?;

    let (stations, stop_points) = parse_stops(files.get("stops.txt").unwrap())?;
    let routes = parse_routes(files.get("routes.txt").unwrap())?;
    let trips = parse_trips(files.get("trips.txt").unwrap())?;
    let stop_times = parse_stop_times(files.get("stop_times.txt").unwrap())?;
    let services = parse_services(files.get("calendar_dates.txt").unwrap())?;

    create_dir_all("build")?;
    create_database(
        stations,
        stop_points,
        routes,
        trips,
        stop_times,
        services,
        "build/gtfs.sqlite",
    )?;

    create_version_file("build/version.txt")?;

    Ok(())
}
