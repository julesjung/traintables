use std::io::Cursor;

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Route {
    #[serde(rename = "route_id")]
    pub id: String,
    #[serde(rename = "route_short_name")]
    pub short_name: String,
    #[serde(rename = "route_long_name")]
    pub long_name: String,
    pub route_type: u8,
    #[serde(rename = "route_color")]
    pub color: Option<String>,
    #[serde(rename = "route_text_color")]
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

pub fn insert_routes(connection: &mut Connection, routes: Vec<Route>) -> Result<()> {
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction.prepare(
            "INSERT INTO routes (id, short_name, long_name, type, color, text_color) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        for route in routes {
            statement.execute(params![
                route.id,
                route.short_name,
                route.long_name,
                route.route_type,
                route.color,
                route.text_color
            ])?;
        }
    }

    transaction.commit()?;

    Ok(())
}
