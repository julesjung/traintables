mod location;
mod service_frame;
mod site_frame;

use anyhow::{Context as _, Result};
use quick_xml::Reader;
use std::fs;
use tracing::{Level, info};
use traintables_core::{download, parse, unzip};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("failed to set default subscriber");

    fs::create_dir_all("build")?;

    download(
        "https://eu.ftp.opendatasoft.com/sncf/plandata/export-opendata-sncf-netex.zip",
        "build/netex.zip",
    )
    .await?;

    info!("downloaded SCNF NeTEx file");

    unzip("build/netex.zip", "build/netex").await?;

    let entries = fs::read_dir("build/netex")?;

    let file_path = entries
        .flatten()
        .find(|entry| {
            let path = entry.path();
            return path
                .extension()
                .map(|extension| extension == "xml")
                .unwrap_or(false);
        })
        .context("could not find SNCF NeTEx file")?
        .path();

    let mut reader = Reader::from_file(file_path)?;
    reader.config_mut().trim_text(true);

    parse(&mut reader, |reader, e| {
        match e.name().as_ref() {
            b"SiteFrame" => {
                site_frame::parse(reader)?;
            }
            b"ServiceFrame" => {
                service_frame::parse(reader)?;
            }
            _ => (),
        }

        Ok(())
    })?;

    Ok(())
}
