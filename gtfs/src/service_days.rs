use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

#[derive(Deserialize, Serialize)]
pub struct ServiceDay {
    pub service_id: u32,
    pub date: String,
}

#[derive(Serialize)]
pub struct Service {
    id: u32,
}

impl Service {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
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

pub fn generate_services_csv(
    services: HashMap<u32, Vec<ServiceDay>>,
    services_path: &str,
    service_days_path: &str,
) -> Result<()> {
    let mut services_writer = csv::Writer::from_path(services_path)?;
    let mut service_days_writer = csv::Writer::from_path(service_days_path)?;

    for (service_id, service_days) in services {
        services_writer.serialize(Service::new(service_id))?;
        for service_day in service_days {
            service_days_writer.serialize(service_day)?;
        }
    }

    services_writer.flush()?;
    service_days_writer.flush()?;

    Ok(())
}
