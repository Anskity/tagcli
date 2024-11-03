fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 2);

    let command = &args[1];

    match command.as_str() {
        "add" => {}
        "list" => {}
        "remove" => {}
        _ => todo!(),
    }
}
