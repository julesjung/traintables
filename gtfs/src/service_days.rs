use std::{collections::HashMap, io::Cursor};

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServiceDay {
    pub service_id: u32,
    pub date: String,
}

pub fn parse_services(data: &Vec<u8>) -> Result<HashMap<u32, Vec<ServiceDay>>> {
    let cursor = Cursor::new(data);
    let mut reader = csv::Reader::from_reader(cursor);

    let mut services: HashMap<u32, Vec<ServiceDay>> = HashMap::new();

    for service_day in reader.deserialize() {
        let service_day: ServiceDay = service_day?;

        services
            .entry(service_day.service_id)
            .or_default()
            .push(service_day);
    }

    Ok(services)
}

pub fn insert_services(
    connection: &mut Connection,
    services: HashMap<u32, Vec<ServiceDay>>,
) -> Result<()> {
    let transaction = connection.transaction()?;

    {
        let mut statement = transaction.prepare("INSERT INTO services (id) VALUES (?1)")?;

        for (service_id, service_days) in services {
            statement.execute(params![service_id])?;

            let mut statement = transaction
                .prepare("INSERT INTO service_days (service_id, date) VALUES (?1, ?2)")?;

            for service_day in service_days {
                statement.execute(params![service_day.service_id, service_day.date])?;
            }
        }
    }

    transaction.commit()?;

    Ok(())
}
