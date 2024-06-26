use colored::Colorize;
use std::{
    fmt,
    io::{self, Write},
    path::PathBuf,
    process,
};

use crate::config::Config;

pub struct Command {
    pub command: CommandName,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(command: CommandName, args: Vec<String>) -> Self {
        Command { command, args }
    }

    pub fn from_str(s: &str, config: &Config) -> Option<Self> {
        let mut split = s.split_whitespace();
        let cmd = split.next().unwrap();

        let args = split.map(|arg| arg.trim().to_string()).collect();
        let command_name = CommandName::from_str(cmd, config)?;

        Some(Command::new(command_name, args))
    }

    pub fn is_builtin(&self) -> bool {
        !matches!(&self.command, CommandName::Other { name: _, path: _ })
    }
}

#[derive(PartialEq)]
pub enum CommandName {
    Exit,
    Echo,
    Type,
    Pwd,
    Cd,
    Other { name: String, path: PathBuf },
}

impl fmt::Display for CommandName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandName::Exit => write!(f, "exit"),
            CommandName::Echo => write!(f, "echo"),
            CommandName::Type => write!(f, "type"),
            CommandName::Pwd => write!(f, "pwd"),
            CommandName::Cd => write!(f, "cd"),
            CommandName::Other { name, path: _ } => write!(f, "{}", name),
        }
    }
}

impl CommandName {
    fn from_str(s: &str, config: &Config) -> Option<Self> {
        match s {
            "exit" => Some(CommandName::Exit),
            "echo" => Some(CommandName::Echo),
            "type" => Some(CommandName::Type),
            "pwd" => Some(CommandName::Pwd),
            "cd" => Some(CommandName::Cd),
            command => {
                let path = config
                    .path
                    .iter()
                    .find_map(|path| path.join(command).exists().then(|| path.join(command)));

                path.map(|path| CommandName::Other {
                    name: command.to_string(),
                    path,
                })
            }
        }
    }
}

impl Command {
    pub fn execute(&self, config: &Config) {
        match &self.command {
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

                let command = Command::from_str(cmd, config);

                match command {
                    Some(cmd) => {
                        if cmd.is_builtin() {
                            println!(
                                "{} is a shell {}",
                                cmd.command.to_string().red(),
                                "builtin".red()
                            );
                        } else {
                            match &cmd.command {
                                CommandName::Other { name, path } => {
                                    println!("{} is {}", name, path.display());
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    None => println!("{}: not found", cmd),
                }
            }

            CommandName::Pwd => {
                println!("{}", std::env::current_dir().unwrap().display());
            }

            CommandName::Other { name: _, path } => {
                let mut command = process::Command::new(path);
                command.args(&self.args);

                let output = command.output().unwrap();
                let stdout = String::from_utf8_lossy(&output.stdout);
                print!("{}", stdout);
                io::stdout().flush().unwrap();
            }

            CommandName::Cd => {
                let home_path_str = config.home.display().to_string();

                let path = self.args.first().unwrap_or(&home_path_str);

                let path = path.trim().replace('~', &home_path_str);

                let path = PathBuf::from(path);

                if path.exists() {
                    std::env::set_current_dir(path).unwrap();
                } else {
                    eprintln!("{}: No such {} or directory", path.display(), "file".red());
                }
            }
        }
    }
}
