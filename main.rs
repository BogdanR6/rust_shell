use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut args = input.trim().split_whitespace();
        let command = args.next().unwrap();
        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
