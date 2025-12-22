use std::{fs::File, io::Write};

use anyhow::Result;
use uuid::Uuid;

pub fn create_version_file(path: &str) -> Result<()> {
    let update_id = Uuid::new_v4().to_string();

    let mut file = File::create(path)?;
    file.write_all(update_id.as_bytes())?;

    Ok(())
}
