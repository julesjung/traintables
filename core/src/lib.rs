pub mod error;

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use zip::ZipArchive;

use crate::error::Result;

pub async fn fetch(url: &str) -> Result<Vec<u8>> {
    let data = reqwest::get(url).await?.bytes().await?;
    Ok(data.to_vec())
}

pub async fn unzip(data: Vec<u8>) -> Result<HashMap<String, Vec<u8>>> {
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
