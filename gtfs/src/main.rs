use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;
use std::{fs::File, io::copy};
use zip::ZipArchive;

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

async fn download_gtfs(gtfs_url: &str, gtfs_path: &str) -> Result<()> {
    let data = reqwest::get(gtfs_url).await?.bytes().await?;
    let mut file = File::create(gtfs_path)?;
    copy(&mut data.as_ref(), &mut file)?;

    Ok(())
}

fn extract_file(archive: &mut ZipArchive<File>, file_path: &str) -> Result<()> {
    let mut zip_file = archive.by_name(file_path)?;
    let mut file = File::create(file_path)?;
    std::io::copy(&mut zip_file, &mut file)?;

    Ok(())
}

fn extract_gtfs(gtfs_path: &str) -> Result<()> {
    let file = File::open(gtfs_path)?;
    let mut archive = ZipArchive::new(file)?;

    extract_file(&mut archive, "stops.txt")?;

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Stop {
    stop_id: String,
    stop_name: String,
    location_type: u8,
}

fn parse_stops(path: &str) -> Result<Vec<Stop>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut stops = Vec::new();

    for stop in reader.deserialize() {
        let stop: Stop = stop?;

        if stop.location_type == 1 {
            stops.push(stop);
        }
    }

    Ok(stops)
}

fn create_database(stops: Vec<Stop>, path: &str) -> Result<()> {
    let mut connection = Connection::open(path)?;

    connection.execute_batch(
        "
        PRAGMA journal_mode = OFF;
        PRAGMA synchronous = OFF;
        PRAGMA temp_store = MEMORY;

        CREATE TABLE IF NOT EXISTS stops (
            stop_id TEXT PRIMARY KEY,
            stop_name TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS index_stops_name ON stops(stop_name);
        ",
    )?;

    {
        let transaction = connection.transaction()?;

        {
            let mut statement = transaction
                .prepare("INSERT OR REPLACE INTO stops (stop_id, stop_name) VALUES (?1, ?2)")?;

            for stop in stops {
                statement.execute(params![stop.stop_id, stop.stop_name])?;
            }
        }

        transaction.commit()?;
    }

    connection.execute_batch("VACUUM; PRAGMA optimize;")?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let gtfs_url = get_gtfs_url().await?;
    let gtfs_path = "gtfs.zip";

    download_gtfs(gtfs_url.as_str(), gtfs_path).await?;
    extract_gtfs(gtfs_path)?;

    let stops = parse_stops("stops.txt")?;

    create_database(stops, "gtfs.sqlite")?;

    Ok(())
}
