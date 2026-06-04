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

fn parse_input(input: &str) -> ShellCommand {
    let builtin_commands: Vec<&str> = vec!["echo", "exit", "type", "pwd", "cd"];
    if input.trim() == "exit" {
        return ShellCommand::Exit;
    }
    let input_list: Vec<&str> = input.trim().split(" ").collect();
    if input_list[0] == "echo" {
        return ShellCommand::Echo(input_list[1..].join(" ").to_string());
    } else if input_list[0] == "type" {
        let command_type: CommandType = if builtin_commands.contains(&input_list[1]) {
            CommandType::Builtin
        } else if is_executable_file(input_list[1]) != "" {
            CommandType::Executable
        } else {
            CommandType::NotExecutable
        };
        return ShellCommand::Type(command_type, input_list[1].to_string());
    } else if input_list[0] == "pwd" {
        return ShellCommand::Pwd;
    } else if input_list[0] == "cd" {
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
                        if Path::new(&path).is_dir() {
                            let _ = env::set_current_dir(Path::new(&path));
                        } else {
                            println!("cd: {}: No such file or directory", path);
                        }
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
