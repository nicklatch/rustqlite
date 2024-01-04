pub mod meta;
pub mod statment;
use crate::{
    meta::do_meta_command,
    statment::{execute_statement, prepare_statement, Statement},
};
use std::{
    fmt::Error,
    io::{stdout, Write},
};

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

fn main() {
    loop {
        print!("rustqlite -> ");
        let _ = stdout().flush();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = input.trim().to_string();
        if input.as_str().starts_with('.') {
            do_meta_command(&input);
            continue;
        }

        let statement: Result<Statement, Error> = match prepare_statement(&input) {
            (PrepareResult::PrepareSuccess, Some(s)) => Ok(s),
            (PrepareResult::PrepareUnrecognizedStatement, None) => {
                println!("Unrecognized keyword at start of \"{}\"", &input);
                continue;
            }
            _ => continue,
        };

        execute_statement(statement.unwrap());

        println!("Completed.")
    }
}
