pub mod constants {
    pub const PROMPT: &str = "rssql> ";
    pub mod command {
        pub const EXIT: &str = ".exit";
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

pub mod interact {
    pub fn print_prompt() {
        print!("{}", super::constants::PROMPT);
    }
    pub fn on_exit() {
        println!("Bye!");
    }
    pub fn repl() {
        loop {
            crate::interact::print_prompt();
            let command = super::utils::read_line();
            match command.as_str() {
                crate::constants::command::EXIT => break,
                _ => println!("Unrecognized command '{}'", command),
            }
        }
    }
}
