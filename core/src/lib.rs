use std::{
    fs,
    io::{self, BufRead, BufReader},
    path::Path,
};

use anyhow::Result;
use futures::StreamExt;
use quick_xml::{
    Reader,
    events::{BytesStart, Event},
    name::QName,
};
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

pub fn get_attribute(e: &BytesStart, key: &[u8]) -> Option<String> {
    e.attributes()
        .filter_map(|attribute| attribute.ok())
        .find(|attribute| attribute.key.as_ref() == key)
        .map(|attribute| String::from_utf8_lossy(&attribute.value).to_string())
}

pub fn parse<R, F>(reader: &mut Reader<R>, mut handle_start: F) -> Result<()>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, &BytesStart) -> Result<()>,
{
    let mut event_buf = Vec::new();

    loop {
        match reader.read_event_into(&mut event_buf)? {
            Event::Start(e) => handle_start(reader, &e)?,
            Event::Empty(e) => handle_start(reader, &e)?,
            Event::Eof => {
                break;
            }
            _ => (),
        }

        event_buf.clear();
    }

    Ok(())
}

pub fn parse_tag<R, F>(reader: &mut Reader<R>, tag: &[u8], mut handle_start: F) -> Result<()>
where
    R: BufRead,
    F: FnMut(&mut Reader<R>, &BytesStart) -> Result<()>,
{
    let mut event_buf = Vec::new();

    loop {
        match reader.read_event_into(&mut event_buf)? {
            Event::Start(e) => handle_start(reader, &e)?,
            Event::Empty(e) => handle_start(reader, &e)?,
            Event::End(e) => {
                if e.name().as_ref() == tag {
                    break;
                }
            }
            _ => (),
        }

        event_buf.clear();
    }

    Ok(())
}

pub fn read_text<R>(reader: &mut Reader<R>, tag: &[u8]) -> Result<String>
where
    R: BufRead,
{
    let mut text_buf = Vec::new();
    let text = reader.read_text_into(QName(tag), &mut text_buf)?;
    Ok(String::from_utf8_lossy(text.as_ref()).to_string())
}

pub trait Parse {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Self>
    where
        R: BufRead,
        Self: Sized;
}
