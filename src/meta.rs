pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input: &String) -> MetaCommandResult {
    match input.as_str() {
        ".exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command {:?}.", input);
            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    }
}
