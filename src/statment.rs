use crate::PrepareResult;

#[derive(Clone, Copy)]
pub enum StatementType {
    Insert,
    Select,
    Blank,
}

pub struct Statement(StatementType);

impl Statement {
    pub fn get_type(&self) -> StatementType {
        self.0
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
        "" => (
            PrepareResult::PrepareUnrecognizedStatement,
            Some(Statement(StatementType::Blank)),
        ),
        _ => (PrepareResult::PrepareUnrecognizedStatement, None),
    }
}

pub fn execute_statement(statement: Statement) {
    match statement.get_type() {
        StatementType::Insert => println!("This is where we would do an insert.\n"),
        StatementType::Select => println!("This is where we would do a select.\n"),
        StatementType::Blank => print!(""),
    }
}
