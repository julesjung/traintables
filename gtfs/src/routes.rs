use anyhow::Result;
use serde::{Deserialize, Serialize, de};
use std::io::Cursor;

fn deserialize_short_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let short_name: &str = Deserialize::deserialize(deserializer).unwrap();
    if short_name == "INCONNU" {
        return Ok(None);
    }
    Ok(Some(short_name.to_string()))
}

#[derive(Deserialize, Serialize)]
pub struct Route {
    #[serde(rename(deserialize = "route_id"))]
    pub id: String,
    #[serde(
        rename(deserialize = "route_short_name"),
        deserialize_with = "deserialize_short_name"
    )]
    pub short_name: Option<String>,
    #[serde(rename(serialize = "type"))]
    pub route_type: u8,
    #[serde(rename(deserialize = "route_color"))]
    pub color: Option<String>,
    #[serde(rename(deserialize = "route_text_color"))]
    pub text_color: Option<String>,
}

pub fn parse_routes(data: &Vec<u8>) -> Result<Vec<Route>> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut routes: Vec<Route> = Vec::new();

    for route in reader.deserialize() {
        let route: Route = route?;

        routes.push(route);
    }

    Ok(routes)
}

pub fn generate_routes_csv(routes: Vec<Route>, path: &str) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for route in routes {
        writer.serialize(route)?;
    }

    writer.flush()?;

    Ok(())
}
