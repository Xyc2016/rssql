use crate::types::{Entity, Statement};

pub fn prepare_statement(raw_input: &str) -> Result<Statement, &str> {
    let words: Vec<String> = raw_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let first = words.get(0);
    let tag = match first {
        Some(s) => s,
        None => return Err("Empty input, please input something"),
    };

    match tag.as_str() {
        super::constants::statement::tag::INSERT => {
            let data_words = words[1..].to_vec();
            let entity = match Entity::try_from(data_words) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            Ok(Statement::StatementInsert(entity))
        }
        super::constants::statement::tag::SELECT => Ok(Statement::StatementSelect),
        _ => Err("Unrecognized tag"),
    }
}

impl crate::traits::Executable for Statement {
    fn execute(&self) {
        match self {
            Statement::StatementInsert(v) => println!("got {}", v),
            Statement::StatementSelect => println!("This is where we would do a select."),
        }
    }
}
