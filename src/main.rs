use colored::Colorize;
use std::{
    io::{self, Write},
    process,
    str::FromStr,
};

struct Command {
    command: CommandName,
    args: Vec<String>,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let cmd = split.next().unwrap();

        let args = split.map(|arg| arg.to_string()).collect();
        let command_name = CommandName::from_str(cmd)?;

        Ok(Command::new(command_name, args))
    }
}

impl Command {
    fn new(command: CommandName, args: Vec<String>) -> Self {
        Command { command, args }
    }
}

enum CommandName {
    Exit,
}

impl FromStr for CommandName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(CommandName::Exit),
            _ => Err(()),
        }
    }
}

impl Command {
    fn execute(&self) {
        match self.command {
            CommandName::Exit => {
                let default = String::from("0");

                let first_arg = self.args.first().unwrap_or(&default);
                let Ok(status) = first_arg.parse::<i32>() else {
                    eprintln!("{}: invalid status code", first_arg.red());
                    return;
                };

                process::exit(status);
            }
        }
    }
}

fn main() {
    repl();
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

fn repl() -> ! {
    loop {
        print_prompt();
        let input = read_line();
        let trimmed = input.trim();
        let command = Command::from_str(trimmed);

        match command {
            Ok(cmd) => {
                cmd.execute();
            }
            Err(_) => println!("{}: {} not found", trimmed, "command".red()),
        }
    }
}
