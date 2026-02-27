use anyhow::Context as _;
use csv::Writer;
use quick_xml::{Reader, events::BytesStart};
use serde::Serialize;
use std::io::BufRead;
use tracing::info;
use traintables_core::{Parse, get_attribute, parse_tag, read_text};

#[derive(Serialize)]
struct Line {
    id: String,
    name: String,
    code: String,
    color: String,
    text_color: String,
}

impl Parse for Line {
    fn parse<R>(reader: &mut Reader<R>, e: &BytesStart) -> anyhow::Result<Self>
    where
        R: BufRead,
    {
        let id = get_attribute(e, b"id").context("id not found")?;
        let mut name = String::new();
        let mut code = String::new();
        let mut color = String::new();
        let mut text_color = String::new();

        parse_tag(reader, b"Line", |reader, e| {
            match e.name().as_ref() {
                b"Name" => name = read_text(reader, b"Name")?,
                b"PublicCode" => code = read_text(reader, b"PublicCode")?,
                b"Colour" => color = read_text(reader, b"Colour")?,
                b"TextColour" => text_color = read_text(reader, b"TextColour")?,
                _ => (),
            }

            Ok(())
        })?;

        Ok(Line {
            id,
            name,
            code,
            color,
            text_color,
        })
    }
}

pub fn parse<R>(reader: &mut Reader<R>) -> anyhow::Result<()>
where
    R: BufRead,
{
    let mut writer = Writer::from_path("build/lines.csv")?;
    let mut count: u32 = 0;

    parse_tag(reader, b"lines", |reader, e| {
        match e.name().as_ref() {
            b"Line" => {
                let line = Line::parse(reader, e)?;
                writer.serialize(&line)?;
                count += 1;
            }
            _ => (),
        }

        Ok(())
    })?;

    info!("found {} lines", count);

    Ok(())
}
