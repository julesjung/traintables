use std::io::BufRead;

use anyhow::Context as _;
use csv::Writer;
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use tracing::info;
use traintables_core::{Parse, get_attribute, parse_tag, read_text};

#[derive(Serialize)]
struct Route {
    id: String,
    distance: u32,
    line_id: String,
    direction_type: String,
}

impl Parse for Route {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> anyhow::Result<Self>
    where
        R: std::io::BufRead,
    {
        let id = get_attribute(e, b"id").context("id not found")?;
        let mut distance = 0;
        let mut line_id = String::new();
        let mut direction_type = String::new();

        parse_tag(reader, b"Route", |reader, e| {
            match e.name().as_ref() {
                b"LineRef" => line_id = get_attribute(e, b"ref").context("line ref not found")?,
                b"Distance" => distance = read_text(reader, b"Distance")?.parse()?,
                b"DirectionType" => direction_type = read_text(reader, b"DirectionType")?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            id,
            distance,
            line_id,
            direction_type,
        })
    }
}

pub fn parse<R>(reader: &mut Reader<R>) -> anyhow::Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/routes.csv")?;
    let mut count = 0;

    parse_tag(reader, b"routes", |reader, e| {
        match e.name().as_ref() {
            b"Route" => {
                let route = Route::parse(reader, e)?;
                writer.serialize(&route)?;
                count += 1;
            }
            _ => (),
        }
        Ok(())
    })?;

    info!("found {} routes", count);

    Ok(())
}
