mod message;

use std::io::stdin;

use local_ip_address::local_ip;
use message::user::*;

fn main() {
    let mut user = User::init_user().unwrap();
	
	std::process::Command::new("clear").status().unwrap();
	loop {
		println!("Silent:\nadd [name] | remove [name] | list | exit");
		let mut input = String::new();
		let _ = stdin().read_line(&mut input);
		let input = input.trim().to_string();
		println!("");
		if input.is_empty() {
			continue;
		}
		let input: Vec<&str> = input.split_whitespace().collect();

		if input.len() == 0 || input.iter().any(|str| str.is_empty()) {
			continue;
		}
		match input[0] {
			"add" => {
				if input.len() != 2 || input[1].trim().is_empty() {
					println!("Add need an argument [name]");
				}else {
					user.add_newcontact(input[1], local_ip().unwrap());
				}
			}
			"remove" => {
				if input.len() != 2 || input[1].trim().is_empty() {
					println!("Remove need an argument [name]");
				} else {
					user.remove_by_name(input[1]);
				}
			}
			"list" => {
				user.print_contacts();
			}
			"exit" => {
				break;
			}
			_ => {
				println!("Unknow command");
			},
		}
		println!("");
	}
}
