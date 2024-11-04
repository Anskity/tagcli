use std::path::{Path, PathBuf};

pub mod commands;
pub mod tag;

pub fn get_tagcli_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    let tagcli_dir = Path::new(&home).join(".tagcli");
    tagcli_dir
}
