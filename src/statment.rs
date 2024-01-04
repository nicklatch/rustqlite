#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    Insert,
    Select,
    Blank,
    Invalid(String),
}

pub struct Statement(StatementType);

impl Statement {
    pub fn new(statement_type: StatementType) -> Self {
        Statement(statement_type)
    }
}

///Parses user inputed statment
fn parse_statment(input: &String) -> Statement {
    match input.as_str() {
        "insert" => Statement::new(StatementType::Insert),
        "select" => Statement::new(StatementType::Select),
        "" => Statement::new(StatementType::Blank),
        _ => Statement::new(StatementType::Invalid(input.to_string())),
    }
}

///Matches parsed input to return string !INCOMPLETE
fn execute_statement(statement: Statement) -> String {
    match statement.0 {
        StatementType::Blank => String::new(),
        StatementType::Insert => {
            String::from("<This is where we would do an insert>\n\u{2705} Executed.\n")
        }
        StatementType::Select => {
            String::from("<This is where we would do a select>\n\u{2705} Executed.\n")
        }
        StatementType::Invalid(e) => format!("\u{274C} Unrecognized keyword at start of \"{}\"\n", e),
    }
}

pub fn statment_service(input: &String) -> String {
    let parsed = parse_statment(input);
    execute_statement(parsed)
}
