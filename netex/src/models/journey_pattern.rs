use std::io::BufRead;

use crate::models::stop_point_in_journey_pattern::{
    NeTExStopPointInJourneyPattern, StopPointInJourneyPattern, parse_stop_points_in_journey_pattern,
};
use anyhow::{Context as _, Result};
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use traintables_core::{Parse, get_attribute, parse_tag};

pub struct NeTExJourneyPattern {
    pub id: String,
    pub route_id: String,
    pub stop_points_in_journey_pattern: Vec<NeTExStopPointInJourneyPattern>,
}

impl Parse for NeTExJourneyPattern {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Self>
    where
        R: BufRead,
    {
        let id = get_attribute(e, b"id").context("id not found")?;
        let mut route_id = String::new();
        let mut stop_points_in_journey_pattern = Vec::new();

        parse_tag(reader, b"ServiceJourneyPattern", |reader, e| {
            match e.name().as_ref() {
                b"RouteRef" => route_id = get_attribute(e, b"ref").context("ref not found")?,
                b"pointsInSequence" => {
                    stop_points_in_journey_pattern = parse_stop_points_in_journey_pattern(reader)?
                }
                _ => (),
            }

            Ok(())
        })?;

        Ok(Self {
            id,
            route_id,
            stop_points_in_journey_pattern,
        })
    }
}

#[derive(Serialize)]
pub struct JourneyPattern {
    pub id: String,
    pub route_id: String,
}

impl From<NeTExJourneyPattern> for (JourneyPattern, Vec<StopPointInJourneyPattern>) {
    fn from(value: NeTExJourneyPattern) -> Self {
        let journey_pattern = JourneyPattern {
            id: value.id.clone(),
            route_id: value.route_id,
        };

        let stop_points_in_journey_pattern = value
            .stop_points_in_journey_pattern
            .into_iter()
            .map(
                |stop_point_in_journey_pattern: NeTExStopPointInJourneyPattern| {
                    StopPointInJourneyPattern {
                        id: stop_point_in_journey_pattern.id,
                        journey_pattern_id: value.id.clone(),
                        scheduled_stop_point_id: stop_point_in_journey_pattern
                            .scheduled_stop_point_id,
                    }
                },
            )
            .collect();

        (journey_pattern, stop_points_in_journey_pattern)
    }
}
