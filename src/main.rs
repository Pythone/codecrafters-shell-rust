use std::env;
use std::io::{self, Write};
use std::process;

enum Command  {
	Echo,
	Type,
	Exit,
}

impl Command {
	fn from_str(command: &str) -> Option<Command> {
		match command {
			"echo" => Some(Command::Echo),
			"type" => Some(Command::Type),
			"exit" => Some(Command::Exit),
			_ => None,
		}
	}
}
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

fn handle_echo_command(command: &str) {
	let exec_command = command.replace("echo ", "");
	println!("{exec_command}");
}
fn handle_type_command(command: &str) {
	let exec_command = command.replace("type ", "");
	if let Some(_) = Command::from_str(&exec_command) {
		println!("{exec_command} is a shell builtin");
	} else {
		if !check_path(&exec_command){
			println!("{exec_command} not found");
		}
	}
}

fn check_path(executable: &str) -> bool {
	if let Ok(paths) = env::var("PATH") {
		for path in paths.split(':'){
			let path = format!("{path}/{executable}");
			if std::path::Path::new(&path).exists() {
				println!("{executable} is in {path}");
				return true;
			}
		}
}
false;
fn handle_matching(input: &str) {
	if let Some(command) = Command::from_str(input.split_whitespace().next().unwrap()){
		match command {
			Command::Echo => handle_echo_command(&input),
			Command::Type => handle_type_command(&input),
			Command::Exit => handle_exit_command(&input),
		}
	}  else {
		println!("{input}: command not found");
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
