use anyhow::Result;
use csv::Writer;
use quick_xml::Reader;
use quick_xml::events::BytesStart;
use serde::Serialize;
use std::io::BufRead;
use tracing::info;
use traintables_core::{get_attribute, parse_tag, read_text};

#[derive(Serialize, Default)]
struct TopographicPlace {
    id: String,
    name: String,
}

pub fn parse_topographic_places<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/topographic_places.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"topographicPlaces", |reader, e| {
        match e.name().as_ref() {
            b"TopographicPlace" => {
                let topographic_place = parse_topographic_place(reader, e)?;
                writer.serialize(topographic_place)?;
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} topographic places", count);

    Ok(())
}

fn parse_topographic_place<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<TopographicPlace>
where
    R: BufRead,
{
    let mut topographic_place = TopographicPlace {
        id: get_attribute(e, b"id").expect("id not found"),
        ..Default::default()
    };

    parse_tag(reader, b"TopographicPlace", |reader, e| {
        match e.name().as_ref() {
            b"Name" => topographic_place.name = read_text(reader, b"Name")?,
            _ => (),
        };

        Ok(())
    })?;

    Ok(topographic_place)
}
