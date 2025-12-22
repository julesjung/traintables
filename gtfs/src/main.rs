mod stops;

use anyhow::Result;
use flate2::{Compression, write::GzEncoder};
use rusqlite::{Connection, params};
use serde::Deserialize;
use std::{
    fs::{File, create_dir_all},
    io::{Cursor, Read},
};
use zip::ZipArchive;

use crate::stops::{Station, StopPoint, parse_stops};

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

async fn fetch_stops(gtfs_url: &str) -> Result<Vec<u8>> {
    let data = reqwest::get(gtfs_url).await?.bytes().await?;

    let reader = Cursor::new(data.to_vec());
    let mut archive = ZipArchive::new(reader)?;

    let mut stops = archive.by_name("stops.txt")?;

    let mut buffer = Vec::with_capacity(stops.size() as usize);
    stops.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn create_database(stations: Vec<Station>, stop_points: Vec<StopPoint>, path: &str) -> Result<()> {
    let mut connection = Connection::open(path)?;

    connection.execute_batch(
        "
        CREATE TABLE stations (
            station_id TEXT PRIMARY KEY,
            station_name TEXT NOT NULL,
            station_latitude REAL,
            station_longitude REAL
        );

        CREATE TABLE stop_points (
            stop_point_id TEXT PRIMARY KEY,
            stop_point_name TEXT NOT NULL,
            station_id TEXT NOT NULL,
            FOREIGN KEY (station_id) REFERENCES stations(station_id)
        );

        CREATE INDEX index_stations_name ON stations(station_name);
        CREATE INDEX index_stop_points_id ON stop_points(stop_point_id);
        ",
    )?;

    {
        let transaction = connection.transaction()?;

        {
            let mut statement = transaction
                .prepare("INSERT INTO stations (station_id, station_name, station_latitude, station_longitude) VALUES (?1, ?2, ?3, ?4)")?;

            for station in stations {
                statement.execute(params![
                    station.id,
                    station.name,
                    station.latitude,
                    station.longitude
                ])?;
            }
        }

        transaction.commit()?;
    }

    {
        let transaction = connection.transaction()?;

        {
            let mut statement = transaction
                .prepare("INSERT INTO stop_points (stop_point_id, stop_point_name, station_id) VALUES (?1, ?2, ?3)")?;

            for stop_point in stop_points {
                statement.execute(params![
                    stop_point.id,
                    stop_point.name,
                    stop_point.station_id
                ])?;
            }
        }

        transaction.commit()?;
    }

    connection.execute_batch("VACUUM; PRAGMA optimize;")?;

    Ok(())
}

fn gunzip(input_path: &str, output_path: &str) -> Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    let mut encoder = GzEncoder::new(&mut output_file, Compression::best());

    std::io::copy(&mut input_file, &mut encoder)?;
    encoder.finish()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let gtfs_url = get_gtfs_url().await?;

    let stops_data = fetch_stops(gtfs_url.as_str()).await?;

    let (stations, stop_points) = parse_stops(stops_data)?;

    create_dir_all("build")?;
    create_database(stations, stop_points, "build/gtfs.sqlite")?;
    gunzip("build/gtfs.sqlite", "build/gtfs.sqlite.gz")?;

    Ok(())
}
