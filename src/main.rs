use std::path::Path;
use tagcli::commands::{add, list, remove};

fn main() {
    let home = std::env::var("HOME").unwrap();
    let tagcli_dir = Path::new(&home).join(".tagcli");

    if !tagcli_dir.exists() {
        std::fs::create_dir(tagcli_dir).unwrap();
    }

    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 2);

    let command = &args[1];

    match command.as_str() {
        "add" => {
            let tag = &args[2];
            let file = Path::new(&args[3]);

            add(tag, file).unwrap();
        }
        "list" => {
            let tag = &args[2];
            list(tag)
        }
        "remove" => {
            let tag = &args[2];
            let file = Path::new(&args[3]);

            remove(tag, file);
        }
        _ => todo!(),
    }
}
