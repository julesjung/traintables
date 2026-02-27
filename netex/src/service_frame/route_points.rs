use crate::location::Location;
use csv::Writer;
use quick_xml::{Reader, events::BytesStart};
use serde::{Serialize, Serializer, ser::SerializeStruct};
use std::io::BufRead;
use tracing::info;
use traintables_core::{Parse, get_attribute, parse_tag};

#[derive(Default)]
struct RoutePoint {
    id: String,
    location: Location,
}

impl Serialize for RoutePoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RoutePoint", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("longitude", &self.location.longitude)?;
        state.serialize_field("latitude", &self.location.latitude)?;
        state.end()
    }
}

impl Parse for RoutePoint {
    fn parse<R>(reader: &mut quick_xml::Reader<R>, e: &BytesStart) -> anyhow::Result<Self>
    where
        R: std::io::BufRead,
        Self: Sized,
    {
        let id = get_attribute(e, b"id").expect("id not found");
        let mut location = Location::default();

        parse_tag(reader, b"RoutePoint", |reader, e| {
            match e.name().as_ref() {
                b"Location" => location = Location::parse(reader, e)?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self { id, location })
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
