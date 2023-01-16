use crate::{statement, traits::Executable};
use std::{io, io::Write};

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

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
