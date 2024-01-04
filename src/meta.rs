pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

// This function can exit the program as a side effect
// TODO: Return something to the hot path to execute an exit to lmit side effects?
pub fn do_meta_command(input: &str) -> MetaCommandResult {
    match input {
        ".exit" => {
            println!("Exiting...See ya! \u{270c}");
            std::process::exit(0);
        }
        _ => {
            println!("\u{274C} Unrecognized command {:?}.", input);
            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    }
}
