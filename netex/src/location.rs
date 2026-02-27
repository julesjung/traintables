use anyhow::Result;
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use std::io;
use traintables_core::{Parse, parse_tag, read_text};

#[derive(Default, Serialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

impl Parse for Location {
    fn parse<R>(reader: &mut Reader<R>, _: &BytesStart) -> Result<Self>
    where
        R: io::BufRead,
    {
        let mut longitude: f64 = 0.0;
        let mut latitude: f64 = 0.0;

        parse_tag(reader, b"Location", |reader, e| {
            match e.name().as_ref() {
                b"Longitude" => longitude = read_text(reader, b"Longitude")?.parse()?,
                b"Latitude" => latitude = read_text(reader, b"Latitude")?.parse()?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            longitude,
            latitude,
        })
    }
}
