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

pub mod traits {
    pub trait Executable {
        fn execute(&self);
    }
}

pub mod entity {
    use crate::types::Entity;
    use std::{fmt};

    impl TryFrom<Vec<String>> for Entity {
        type Error = &'static str;

        fn try_from(words: Vec<String>) -> Result<Self, Self::Error> {
            let id = match words.get(0) {
                Some(s) => match s.parse::<u32>() {
                    Ok(v) => v,
                    Err(_) => return Err("id must be a number"),
                },
                None => return Err("id is required"),
            };
            let username = match words.get(1) {
                Some(s) => s,
                None => return Err("username is required"),
            };
            let email = match words.get(2) {
                Some(s) => s,
                None => return Err("email is required"),
            };
            Ok(Entity {
                id,
                username: username.to_string(),
                email: email.to_string(),
            })
        }
    }

    // impl Into<Vec<u8>> for Entity {
    //     fn into(self) -> Vec<u8> {
    //         let mut bytes = Vec::new();
    //         bytes.extend(self.id.to_be_bytes().iter());
    //         bytes.extend(self.username.as_bytes());
    //         bytes.extend(self.email.as_bytes());
    //         bytes.as_slice()
    //     }
    // }

    impl fmt::Display for Entity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}, {})", self.id, self.username, self.email)
        }
    }
}

pub mod types {

    pub enum MetaCommandResult {
        MetaCommandSuccess,
        MetaCommandUnrecognizedCommand,
    }

    #[derive(Debug)]
    pub struct Entity {
        pub id: u32,
        pub username: String,
        pub email: String,
    }

    pub enum Statement {
        StatementInsert(Entity),
        StatementSelect,
    }
}

pub mod statement {

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
}

pub mod interact {
    use crate::{statement, traits::Executable};

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
                Ok(v) => v,
                Err(e) => {
                    println!("Parse failed: {}", e);
                    continue;
                }
            };
            statement.execute();
        }
    }
}
