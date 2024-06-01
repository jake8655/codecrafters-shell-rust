use colored::Colorize;
use std::env;
use std::io::{self, Write};
use std::str::FromStr;

mod command;
use command::Command;

mod config;
use config::Config;

fn main() {
    let path_arg = env::var("PATH").expect("PATH not set");
    let config = Config::from_str(&path_arg).expect("invalid PATH");

    repl(config);
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn repl(config: Config) -> ! {
    loop {
        print_prompt();
        let input = read_line();
        let trimmed = input.trim();
        let command = Command::from_str(trimmed, &config);

        match command {
            Some(cmd) => {
                cmd.execute(&config);
            }
            None => println!("{}: {} not found", trimmed, "command".red()),
        }
    }
}
