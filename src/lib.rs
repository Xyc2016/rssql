pub mod constants {
    pub const PROMPT: &str = "rssql> ";
    pub mod command {
        pub const EXIT: &str = ".exit";
    }
    pub mod statement {
        pub mod tag {
            pub const INSERT: &str = "insert";
            pub const SELECT: &str = "select";
        }
    }
}

pub mod utils {
    use std::io::{self, Write};

    pub fn read_line() -> String {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub mod types {
    pub enum MetaCommandResult {
        MetaCommandSuccess,
        MetaCommandUnrecognizedCommand,
    }

    pub enum StatementType {
        StatementInsert,
        StatementSelect,
    }

    pub struct Statement {
        pub statement_type: StatementType,
    }
}

pub mod statement {
    use crate::types;

    use crate::types::StatementType;

    use crate::types::Statement;

    pub fn excute_statement(statement: &Statement) {
        match statement.statement_type {
            StatementType::StatementInsert => println!("This is where we would do an insert."),
            StatementType::StatementSelect => println!("This is where we would do a select."),
        }
    }

    pub fn prepare_statement(raw_input: &str) -> Option<StatementType> {
        let tag = match raw_input.split_whitespace().next() {
            Some(s) => s,
            None => return None,
        };
        match tag {
            super::constants::statement::tag::INSERT => Some(types::StatementType::StatementInsert),
            super::constants::statement::tag::SELECT => Some(types::StatementType::StatementSelect),
            _ => None,
        }
    }
}

pub mod interact {
    use crate::statement;
    use crate::types::{Statement};

    pub fn print_prompt() {
        print!("{}", super::constants::PROMPT);
    }
    pub fn is_meta_command(command: &str) -> bool {
        command.starts_with(".")
    }
    pub fn do_meta_command(command: &str) -> super::types::MetaCommandResult {
        match command {
            super::constants::command::EXIT => {
                 on_exit();
                std::process::exit(0);
            }
            _ => return super::types::MetaCommandResult::MetaCommandUnrecognizedCommand,
        }
    }
    pub fn on_exit() {
        println!("Bye!");
    }
    pub fn repl() {
        loop {
            crate::interact::print_prompt();
            let _raw_input = super::utils::read_line();
            let raw_input = _raw_input.as_str();
            if is_meta_command(raw_input) {
                match do_meta_command(raw_input) {
                    crate::types::MetaCommandResult::MetaCommandSuccess => continue,
                    crate::types::MetaCommandResult::MetaCommandUnrecognizedCommand => {
                        println!("Unrecognized command '{}'", raw_input);
                        continue;
                    }
                }
            }
            let statement = match statement::prepare_statement(raw_input) {
                Some(v) => Statement { statement_type: v },
                None => {
                    println!("Unrecognized keyword at start of '{}'", raw_input);
                    continue;
                }
            };
            statement::excute_statement(&statement);
        }
    }

}
