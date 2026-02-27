mod stop_places;
mod topographic_places;

use crate::site_frame::{
    stop_places::parse_stop_places, topographic_places::parse_topographic_places,
};
use anyhow::Result;
use quick_xml::Reader;
use std::io::BufRead;
use traintables_core::parse_tag;

pub fn parse<R>(reader: &mut Reader<R>) -> Result<()>
where
    R: BufRead,
{
    parse_tag(reader, b"SiteFrame", |reader, e| {
        match e.name().as_ref() {
            b"stopPlaces" => parse_stop_places(reader)?,
            b"topographicPlaces" => parse_topographic_places(reader)?,
            _ => (),
        };

        Ok(())
    })?;

    Ok(())
}
