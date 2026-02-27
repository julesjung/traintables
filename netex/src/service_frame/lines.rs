use std::io::BufRead;

use quick_xml::Reader;
use traintables_core::parse_tag;

pub fn parse<R>(reader: &mut Reader<R>) -> anyhow::Result<()>
where
    R: BufRead,
{
    parse_tag(reader, b"lines", |reader, e| Ok(()))?;

    Ok(())
}
