use std::io::Write;
use std::path::Path;

use crate::tag::{tag_create, tag_exists, tag_get_file};

pub fn add(tag: &str, file: &Path) -> Result<(), std::io::Error> {
    let mut existed = true;
    if !tag_exists(tag) {
        tag_create(tag)?;
        existed = false;
    }

    let mut tag_file = tag_get_file(tag)?;
    if !existed {
        tag_file.write_all("\r\n\r\n".as_bytes())?;
    }
    let txt = format!("\n{}", file.to_str().unwrap_or(""));
    tag_file.write(txt.as_bytes())?;

    Ok(())
}

pub fn remove(tag: &String, file: &Path) {}

pub fn list(tag: &String) {}
