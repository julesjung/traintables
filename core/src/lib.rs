use std::{
    fs,
    io::{self, BufReader},
    path::Path,
};

use anyhow::Result;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

pub async fn download(url: &str, path: &str) -> Result<()> {
    let response = reqwest::get(url).await?;

    let mut stream = response.bytes_stream();

    let mut file = tokio::fs::File::create(path).await?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}

pub async fn unzip(input_path: &str, output_dir: &str) -> Result<()> {
    let file = fs::File::open(input_path)?;

    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for index in 0..archive.len() {
        let mut file = archive.by_index(index)?;
        let output_path = Path::new(output_dir).join(file.name());

        if file.is_file() {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut output_file = fs::File::create(output_path)?;
            io::copy(&mut file, &mut output_file)?;
        }
    }

    Ok(())
}
