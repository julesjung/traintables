use std::io::BufRead;

use anyhow::Result;
use quick_xml::{
    Reader,
    events::{Event, attributes::Attributes},
};
use serde::Serialize;
use tracing::info;

#[derive(Serialize, Default)]
struct StopPlace {
    id: String,
    name: String,
}

#[derive(Serialize, Default)]
struct TopographicPlace {
    id: String,
    name: String,
}

pub fn parse_site_frame<R: BufRead>(
    reader: &mut Reader<R>,
    stop_places_path: &str,
    topographic_places_path: &str,
) -> Result<()> {
    let mut buf = Vec::new();

    let mut stop_places_count: u32 = 0;
    let mut topographic_places_count: u32 = 0;

    let mut stop_places_writer = csv::Writer::from_path(stop_places_path)?;
    let mut topographic_places_writer = csv::Writer::from_path(topographic_places_path)?;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"StopPlace" => {
                    let stop_place = parse_stop_place(reader, e.attributes())?;
                    stop_places_writer.serialize(&stop_place)?;
                    stop_places_count += 1;
                }
                b"TopographicPlace" => {
                    let topographic_place = parse_topographic_place(reader, e.attributes())?;
                    topographic_places_writer.serialize(&topographic_place)?;
                    topographic_places_count += 1;
                }
                _ => (),
            },
            Event::End(e) => {
                if e.name().as_ref() == b"SiteFrame" {
                    break;
                }
            }
            _ => (),
        }

        buf.clear();
    }

    info!("found {} stop places", stop_places_count);
    info!("found {} topographic places", topographic_places_count);

    stop_places_writer.flush()?;
    topographic_places_writer.flush()?;

    Ok(())
}

fn parse_stop_place<R: BufRead>(
    reader: &mut Reader<R>,
    attributes: Attributes,
) -> Result<StopPlace> {
    let mut stop_place = StopPlace::default();
    let mut buf = Vec::new();
    let mut text = Vec::new();

    stop_place.id = attributes
        .filter_map(|attribute| attribute.ok())
        .find(|attribute| attribute.key.as_ref() == b"id")
        .map(|attribute| String::from_utf8_lossy(&attribute.value).to_string())
        .expect("id not found");

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"Name" => {
                    stop_place.name = String::from_utf8_lossy(
                        &reader.read_text_into(e.name(), &mut text)?.as_ref(),
                    )
                    .to_string()
                }
                _ => (),
            },
            Event::End(e) => {
                if e.name().as_ref() == b"StopPlace" {
                    return Ok(stop_place);
                }
            }
            _ => (),
        }

        buf.clear();
    }
}

fn parse_topographic_place<R: BufRead>(
    reader: &mut Reader<R>,
    attributes: Attributes,
) -> Result<TopographicPlace> {
    let mut topographic_place = TopographicPlace::default();
    let mut buf = Vec::new();
    let mut text = Vec::new();

    topographic_place.id = attributes
        .filter_map(|attribute| attribute.ok())
        .find(|attribute| attribute.key.as_ref() == b"id")
        .map(|attribute| String::from_utf8_lossy(&attribute.value).to_string())
        .expect("id not found");

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"Name" => {
                    topographic_place.name = String::from_utf8_lossy(
                        &reader.read_text_into(e.name(), &mut text)?.as_ref(),
                    )
                    .to_string()
                }
                _ => (),
            },
            Event::End(e) => {
                if e.name().as_ref() == b"TopographicPlace" {
                    return Ok(topographic_place);
                }
            }
            _ => (),
        }

        buf.clear();
    }
}
