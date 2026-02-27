use anyhow::Result;
use csv::Writer;
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use serde::Serialize;
use std::io::BufRead;
use tracing::info;
use traintables_core::{get_attribute, parse_tag, read_text};

#[derive(Serialize, Default)]
struct StopPlace {
    id: String,
    name: String,
}

pub fn parse_stop_places<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/stop_places.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"stopPlaces", |reader, e| {
        match e.name().as_ref() {
            b"StopPlace" => {
                let stop_place = parse_stop_place(reader, e)?;
                writer.serialize(stop_place)?;
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} stop places", count);

    Ok(())
}

fn parse_stop_place<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<StopPlace>
where
    R: BufRead,
{
    let mut stop_place = StopPlace {
        id: get_attribute(e, b"id").expect("id not found"),
        ..Default::default()
    };

    parse_tag(reader, b"StopPlace", |reader, e| {
        match e.name().as_ref() {
            b"Name" => stop_place.name = read_text(reader, b"Name")?,
            _ => (),
        }

        Ok(())
    })?;

    Ok(stop_place)
}
