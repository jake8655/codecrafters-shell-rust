use colored::Colorize;
use std::{
    io::{self, Write},
    str::FromStr,
};

enum Command {}

impl FromStr for Command {
    type Err = ();

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        // match s {
        /* _ =>  */
        Err(()) /* , */
        // }
    }
}

fn main() {
    print_prompt();

    let input = read_line();
    let trimmed = input.trim();

    let command = Command::from_str(trimmed);

    match command {
        Ok(_cmd) => {}
        Err(_) => println!("{}: {} not found", trimmed, "command".red()),
    }
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
