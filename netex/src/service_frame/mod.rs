mod lines;
mod route_points;
mod routes;
mod scheduled_stop_points;

use anyhow::Result;
use quick_xml::Reader;
use std::io::BufRead;
use traintables_core::parse_tag;

pub fn parse<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    parse_tag(reader, b"ServiceFrame", |reader, e| {
        match e.name().as_ref() {
            b"routePoints" => route_points::parse(reader)?,
            b"routes" => routes::parse(reader)?,
            b"lines" => lines::parse(reader)?,
            b"scheduledStopPoints" => scheduled_stop_points::parse(reader)?,
            _ => (),
        }

        Ok(())
    })?;

    Ok(())
}
