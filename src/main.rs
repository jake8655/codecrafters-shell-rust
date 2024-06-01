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

        let args = split.map(|arg| arg.trim().to_string()).collect();
        let command_name = CommandName::from_str(cmd)?;

        Ok(Command::new(command_name, args))
    }
}

impl Command {
    fn new(command: CommandName, args: Vec<String>) -> Self {
        Command { command, args }
    }
}

#[derive(PartialEq)]
enum CommandName {
    Exit,
    Echo,
    Type,
}

const BUILTINS: [CommandName; 3] = [CommandName::Exit, CommandName::Echo, CommandName::Type];

impl FromStr for CommandName {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(CommandName::Exit),
            "echo" => Ok(CommandName::Echo),
            "type" => Ok(CommandName::Type),
            _ => Err(()),
        }
    }
}

fn is_builtin(cmd: &str) -> bool {
    let Ok(command_name) = CommandName::from_str(cmd) else {
        return false;
    };

    BUILTINS.contains(&command_name)
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

            CommandName::Echo => {
                let text = self.args.join(" ");
                println!("{}", text);
            }

            CommandName::Type => {
                let Some(cmd) = self.args.first() else {
                    eprintln!("{}: type requires a command", "type".red());
                    return;
                };

                if is_builtin(cmd) {
                    println!("{} is a shell {}", cmd.red(), "builtin".red());
                } else {
                    println!("{} not found", cmd);
                }
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
