use std::io::BufRead;

use anyhow::{Context, Result};
use csv::Writer;
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use tracing::info;
use traintables_core::{Parse, get_attribute, parse_tag, read_text};

#[derive(Default, Serialize)]
struct ScheduledStopPoint {
    id: String,
    name: String,
    code: String,
    longitude: f64,
    latitude: f64,
}

impl Parse for ScheduledStopPoint {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Self>
    where
        R: BufRead,
    {
        let id = get_attribute(e, b"id").context("id not found")?;
        let mut name = String::new();
        let mut code = String::new();
        let mut longitude = 0.0;
        let mut latitude = 0.0;

        parse_tag(reader, b"ScheduledStopPoint", |reader, e| {
            match e.name().as_ref() {
                b"Name" => name = read_text(reader, b"Name")?,
                b"PublicCode" => code = read_text(reader, b"PublicCode")?,
                b"Longitude" => longitude = read_text(reader, b"Longitude")?.parse()?,
                b"Latitude" => latitude = read_text(reader, b"Latitude")?.parse()?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            id,
            name,
            longitude,
            latitude,
            code,
        })
    }
}

pub fn parse<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/scheduled_stop_points.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"scheduledStopPoints", |reader, e| {
        match e.name().as_ref() {
            b"ScheduledStopPoint" => {
                let scheduled_stop_point = ScheduledStopPoint::parse(reader, e)?;
                writer.serialize(scheduled_stop_point)?;
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} scheduled stop points", count);

    writer.flush()?;

    Ok(())
}
