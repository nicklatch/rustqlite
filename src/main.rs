use std::{
    fmt::Error,
    io::{stdout, Write},
};

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

#[derive(Clone, Copy)]
pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement(StatementType);

impl Statement {
    pub fn get_type(&self) -> StatementType {
        self.0
    }
}

pub fn do_meta_command(input: &String) -> MetaCommandResult {
    match input.as_str() {
        ".exit" => {
            println!("Exiting...");
            std::process::exit(0);
        }
        _ => {
            println!("Unrecognized command '{:?}'.", input);
            MetaCommandResult::MetaCommandSuccess
        }
    }
}

pub fn prepare_statement(input: &str) -> (PrepareResult, Option<Statement>) {
    match input {
        "insert" => (
            PrepareResult::PrepareSuccess,
            Some(Statement(StatementType::Insert)),
        ),
        "select" => (
            PrepareResult::PrepareSuccess,
            Some(Statement(StatementType::Select)),
        ),
        _ => (PrepareResult::PrepareUnrecognizedStatement, None),
    }
}

pub fn execute_statement(statement: Statement) {
    match statement.get_type() {
        StatementType::Insert => println!("This is where we would do an insert.\n"),
        StatementType::Select => println!("This is where we would do a select.\n"),
    }
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
            match do_meta_command(&input) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command {}", &input);
                    continue;
                }
            }
        }

        let statement: Result<Statement, Error> = match prepare_statement(&input) {
            (PrepareResult::PrepareSuccess, Some(s)) => Ok(s),
            (PrepareResult::PrepareUnrecognizedStatement, None) | (_, _) => {
                eprintln!("Unrecognized keyword at start of {}", &input);
                continue;
            }
        };

        execute_statement(statement.unwrap());

        println!("Completed.")
    }
}
