use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut args = command.trim().split_whitespace(); //FIX:  does not support args with "____"
            let command = args.next().unwrap();
            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                } // command
            } // match
        } // while
        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            let _ = final_command.wait();
        }
    } // loop
} // main
