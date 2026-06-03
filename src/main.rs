#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    loop {
        match stdin().read_line(&mut input) {
            Ok(_) => {
                println!("{}: command not found", input.trim());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        };
    }
}
