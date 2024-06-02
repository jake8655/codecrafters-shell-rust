use colored::Colorize;
use std::env;
use std::io::{self, Write};

mod command;
use command::Command;

mod config;
use config::Config;

fn main() {
    let path_env = env::var("PATH").expect("PATH not set");
    let home_env = env::var("HOME").expect("HOME not set");
    let config = Config::from_str(&path_env, &home_env);

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
            None => eprintln!("{}: {} not found", trimmed, "command".red()),
        }
    }
}
