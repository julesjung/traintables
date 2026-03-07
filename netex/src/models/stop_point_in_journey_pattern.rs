use anyhow::{Context as _, Result};
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use std::io::BufRead;
use traintables_core::{Parse, get_attribute, parse_tag};

pub fn parse_stop_points_in_journey_pattern<R>(
    reader: &mut Reader<R>,
) -> Result<Vec<NeTExStopPointInJourneyPattern>>
where
    R: BufRead,
{
    let mut stop_points_in_journey_pattern = Vec::new();

    parse_tag(reader, b"pointsInSequence", |reader, e| {
        match e.name().as_ref() {
            b"StopPointInJourneyPattern" => {
                let stop_point_in_journey_pattern =
                    NeTExStopPointInJourneyPattern::parse(reader, e)?;
                stop_points_in_journey_pattern.push(stop_point_in_journey_pattern);
            }
            _ => (),
        }

        Ok(())
    })?;

    Ok(stop_points_in_journey_pattern)
}

pub struct NeTExStopPointInJourneyPattern {
    pub id: String,
    pub scheduled_stop_point_id: String,
}

impl Parse for NeTExStopPointInJourneyPattern {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Self>
    where
        R: BufRead,
    {
        let id = get_attribute(e, b"id").context("id not found")?;
        let mut scheduled_stop_point_id = String::new();

        parse_tag(reader, b"StopPointInJourneyPattern", |_, e| {
            match e.name().as_ref() {
                b"ScheduledStopPointRef" => {
                    scheduled_stop_point_id = get_attribute(e, b"ref").context("ref not found")?
                }
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            id,
            scheduled_stop_point_id,
        })
    }
}

#[derive(Serialize)]
pub struct StopPointInJourneyPattern {
    pub id: String,
    pub journey_pattern_id: String,
    pub scheduled_stop_point_id: String,
}
