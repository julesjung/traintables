use crate::models::journey_pattern::NeTExJourneyPattern;
use anyhow::Result;
use csv::Writer;
use quick_xml::Reader;
use std::io::BufRead;
use tracing::info;
use traintables_core::{Parse, parse_tag};

pub fn parse<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/journey_patterns.csv")?;
    let mut stop_points_in_journey_pattern_writer =
        Writer::from_path("build/stop_points_in_journey_pattern.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"journeyPatterns", |reader, e| {
        match e.name().as_ref() {
            b"ServiceJourneyPattern" => {
                let journey_pattern = NeTExJourneyPattern::parse(reader, e)?;
                let (journey_pattern, stop_points_in_journey_pattern) = journey_pattern.into();
                writer.serialize(journey_pattern)?;
                for stop_point_in_journey_pattern in stop_points_in_journey_pattern {
                    stop_points_in_journey_pattern_writer
                        .serialize(stop_point_in_journey_pattern)?;
                }
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} journey patterns", count);

    Ok(())
}
