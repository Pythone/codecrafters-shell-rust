use std::io::{self, Write};
use std::process;


fn handle_exit_command(command: &str) {
	if let Some(last_char) = command.chars().last() {
		match last_char.to_string().parse::<i32>() {
			Ok(status) => process::exit(status),
			Err(e) => println!("Failed to parse the status code: {}", e),
		}
	} else {
		print!("Failed to get last char of exit command(missing status code).");
	}
}

fn command(command &str) {
	let echo_less_command = command.replace("echo", "");
	println!("{echo_less_command}");
}

fn handle_matching(command: &str) {
	match command {
		x if x.to_string().contains("exit")  => handle_exit_command(&command),
		x if x.to_string().contains("echo") => handle_echo_command(&command);
		_ => println!("{command}: command not found"),
	}
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

    // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        handle_matching(input.trim());
    }
}
