use std::{fs::File, io::Write, path::PathBuf};

use crate::get_tagcli_dir;

pub fn tag_exists(tag: &str) -> bool {
    get_tagcli_dir().join(tag).is_file()
}

pub fn tag_get_path(tag: &str) -> PathBuf {
    get_tagcli_dir().join(tag)
}

pub fn tag_create(tag: &str) -> Result<(), std::io::Error> {
    assert!(!tag_exists(tag));

    std::fs::File::create_new(tag_get_path(tag))?;

    Ok(())
}

pub fn tag_get_file(tag: &str) -> Result<File, std::io::Error> {
    std::fs::File::options()
        .read(true)
        .write(true)
        .append(true)
        .open(tag_get_path(tag))
}
