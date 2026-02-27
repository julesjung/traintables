use csv::Writer;
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use std::io::BufRead;
use tracing::info;
use traintables_core::{Parse, get_attribute, parse_tag, read_text};

#[derive(Default, Serialize)]
struct RoutePoint {
    id: String,
    latitude: f64,
    longitude: f64,
}

impl Parse for RoutePoint {
    fn parse<R>(reader: &mut quick_xml::Reader<R>, e: &BytesStart) -> anyhow::Result<Self>
    where
        R: std::io::BufRead,
    {
        let id = get_attribute(e, b"id").expect("id not found");
        let mut latitude = 0.0;
        let mut longitude = 0.0;

        parse_tag(reader, b"RoutePoint", |reader, e| {
            match e.name().as_ref() {
                b"Latitude" => latitude = read_text(reader, b"Latitude")?.parse()?,
                b"Longitude" => longitude = read_text(reader, b"Longitude")?.parse()?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            id,
            latitude,
            longitude,
        })
    }
}

pub fn parse<R>(reader: &mut Reader<R>) -> anyhow::Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/route_points.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"routePoints", |reader, e| {
        match e.name().as_ref() {
            b"RoutePoint" => {
                let route_point = RoutePoint::parse(reader, e)?;
                writer.serialize(route_point)?;
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} route points", count);

    Ok(())
}
