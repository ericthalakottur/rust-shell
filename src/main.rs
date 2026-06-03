#[allow(unused_imports)]
use std::io::{self, Write, stdin};

fn main() {
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

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
