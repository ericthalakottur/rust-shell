#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    let BUILTIN_COMMANDS: Vec<&str> = vec!["echo", "exit", "type"];
    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    break;
                }
                let input_list: Vec<&str> = input.trim().split(" ").collect();
                if input_list[0] == "echo" {
                    let output = input_list[1..].join(" ");
                    println!("{}", output);
                } else if input_list[0] == "type" {
                    if BUILTIN_COMMANDS.contains(&input_list[1]) {
                        println!("{} is a shell builtin", input_list[1]);
                    } else {
                        println!("{}: not found", input_list[1]);
                    }
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        };
    }
}
