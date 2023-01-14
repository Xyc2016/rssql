use rssql;

fn main() {
    loop {
        rssql::interact::print_prompt();
        let command = rssql::utils::read_line();
        match command.as_str() {
            rssql::constants::command::EXIT => break,
            _ => println!("Unrecognized command '{}'", command),
        }
    }
    println!("Bye!");
}
