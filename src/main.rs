#[allow(unused_imports)]
use std::io::{self, Write, stdin};
use std::{env, fs, os::unix::fs::MetadataExt, path::Path, process::Command};

enum CommandType {
    Builtin,
    Executable,
    NotExecutable,
}

enum ShellCommand {
    Exit,
    Echo(String),
    Type(CommandType, String),
    Pwd,
    Cd(String),
    ExternalCommand(String, Vec<String>),
    InvalidCommand(String),
}

fn parse_string(input: &str) -> Vec<String> {
    let input: Vec<char> = input.chars().collect();
    let mut parsed_string = vec![];
    let mut i = 0;
    while i < input.len() {
        if input[i] != ' ' {
            let mut j = i;
            let delimiter: char = if input[i] == '\'' {
                j += 1;
                '\''
            } else {
                ' '
            };
            let mut s = String::new();
            while j < input.len() && input[j] != delimiter {
                s.push(input[j]);
                j += 1;
            }
            parsed_string.push(s);
            i = j;
        }
        i += 1;
    }
    parsed_string
}

fn parse_input(input: &str) -> ShellCommand {
    let builtin_commands: Vec<&str> = vec!["exit", "echo", "type", "pwd", "cd"];
    if input.trim() == builtin_commands[0] {
        return ShellCommand::Exit;
    }
    let input_list: Vec<String> = parse_string(input.trim());
    if input_list[0] == builtin_commands[1] {
        return ShellCommand::Echo(input_list[1..].join(" ").to_string());
    } else if input_list[0] == builtin_commands[2] {
        let command_type: CommandType = if builtin_commands.contains(&input_list[1].as_str()) {
            CommandType::Builtin
        } else if is_executable_file(input_list[1].as_str()) != "" {
            CommandType::Executable
        } else {
            CommandType::NotExecutable
        };
        return ShellCommand::Type(command_type, input_list[1].to_string());
    } else if input_list[0] == builtin_commands[3] {
        return ShellCommand::Pwd;
    } else if input_list[0] == builtin_commands[4] {
        return ShellCommand::Cd(input_list[1].to_string());
    } else if is_executable_file(&input_list[0]) != "" {
        return ShellCommand::ExternalCommand(
            input_list[0].to_string(),
            input_list[1..].iter().map(|x| x.to_string()).collect(),
        );
    }
    ShellCommand::InvalidCommand(input.trim().to_string())
}

fn is_executable_file(command: &str) -> String {
    let paths = std::env::var("PATH").unwrap();
    for path in paths.split(":") {
        let current_path = Path::new(path).join(command);
        if !current_path.is_file() {
            continue;
        }
        if fs::metadata(&current_path).unwrap().mode() & 0o001 == 1 {
            return String::from(current_path.to_str().unwrap());
        }
    }
    "".to_string()
}

fn main() {
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd: ShellCommand = parse_input(&input);
                match cmd {
                    ShellCommand::Exit => {
                        break;
                    }
                    ShellCommand::Echo(text) => println!("{}", text),
                    ShellCommand::Type(cmd_type, cmd) => match cmd_type {
                        CommandType::Builtin => {
                            println!("{} is a shell builtin", cmd)
                        }
                        CommandType::Executable => {
                            println!("{} is {}", cmd, is_executable_file(&cmd))
                        }
                        CommandType::NotExecutable => {
                            println!("{}: not found", cmd);
                        }
                    },
                    ShellCommand::Pwd => {
                        println!("{}", env::current_dir().unwrap().display());
                    }
                    ShellCommand::Cd(path) => {
                        let path = path.replace("~", std::env::var("HOME").unwrap().as_str());
                        let path = Path::new(&path);
                        env::set_current_dir(path).unwrap_or_else(|_| {
                            println!("cd: {}: No such file or directory", path.display())
                        });
                    }
                    ShellCommand::ExternalCommand(cmd, args) => {
                        let output = Command::new(cmd)
                            .args(args)
                            .output()
                            .expect("failed to execute command");
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    ShellCommand::InvalidCommand(cmd) => {
                        println!("{}: command not found", cmd);
                    }
                };
                io::stdout().flush().unwrap();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        };
    }
}
