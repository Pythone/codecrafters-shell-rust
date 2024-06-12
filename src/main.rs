use std::env;
use std::io::{self, Write};
use std::process;
use std::process::Command;
enum Commands {
	Echo,
	Type,
	Exit,
	Pwd,
}

impl Commands {
	fn from_str(command: &str) -> Option<Commands> {
		match command {
			"echo" => Some(Commands::Echo),
			"type" => Some(Commands::Type),
			"exit" => Some(Commands::Exit),
			"pwd" => Some(Commands::Pwd),
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
	if let Some(_) = Commands::from_str(&exec_command) {
		println!("{exec_command} is a shell builtin");
	} else {
		if let Some(_) = check_path_for_exec(&exec_command){
		} else{
			println!("{exec_command} not found");
		}
	}
}
fn handle_pwd_command(command: &str){
	let current_dir = env::current_dir().display().unwrap();
	println!("{}", current_dir);
	
}

fn check_path_for_exec(executable: &str) -> Option<String> {
	if let Ok(paths) = env::var("PATH") {
		for path in paths.split(':'){
			let path = format!("{path}/{executable}");
			if std::path::Path::new(&path).exists() {
				println!("{executable} is in {path}");
				return Some(path);
			}
		}
	}
	None
}

fn execute_binary(path: &str, arg: &str) {
	let output = Command::new(path)
	    .arg(arg)
	    .output();
	match output {
		Ok(output) => {
			if output.status.success() {
				io::stdout().write_all(&output.stdout).unwrap();
			} else {
				println!("Command failed with exit code: {:?}", output.status.code());
			}
		}
		Err(e)  => {
			eprintln!("Error executing command: {}", e);
		}
	}
}

fn handle_execution_or_unsupported(path: &str) {
	let command: Vec<&str> = path.split_whitespace().collect();
	if command.len() >0 {
		if std::path::Path::new(command[0]).exists() {
			execute_binary(command[0], command[1])
		} else {
			println!("{path}: command not found");
		}
	} else{
		println!("{path}: command not found");
	}
}
fn handle_matching(input: &str) {
	if let Some(command) = Commands::from_str(input.split_whitespace().next().unwrap()){
		match command {
			Commands::Echo => handle_echo_command(&input),
			Commands::Type => handle_type_command(&input),
			Commands::Exit => handle_exit_command(&input),
			Commands::Pwd => handle_pwd_command(&input), 
		}
	}  else {
		handle_execution_or_unsupported(input);
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
