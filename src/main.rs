pub mod meta;
pub mod statment;
use crate::{meta::do_meta_command, statment::statment_service};
use std::io::{stdout, Write};

fn main() {
    loop {
        print!("rustqlite> ");
        let _ = stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // a preceding "." indecates a meta-command
        input = input.trim().to_string();
        if input.starts_with('.') {
            do_meta_command(&input);
            continue;
        }

        print!("{}", statment_service(&input))
    }
}
