use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Deserialize, Serialize)]
pub struct Route {
    #[serde(rename(deserialize = "route_id"))]
    pub id: String,
    #[serde(rename(deserialize = "route_short_name"))]
    pub short_name: String,
    #[serde(rename(deserialize = "route_long_name"))]
    pub long_name: String,
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
