mod message;

use local_ip_address::local_ip;
use message::user::*;
use std::io::{self, Write};
mod crypto;
use crypto::encrypt::Encryptor;
use message::helper::*;

fn main() {
    std::process::Command::new("clear").status().unwrap();
    print!("Entrez un message a chiffré : ");
    io::stdout().flush().unwrap();
    let mut input_message = String::new();
    io::stdin().read_line(&mut input_message).unwrap();
    input_message = input_message.trim().to_string();

    let shared_secret: [u8; 32] = [0u8; 32];

    let message = Message::new(input_message);
    let encryptor = Encryptor::new();

    let encrypted_message = encryptor.encrypt_message(message.message, &shared_secret);
    println!("Message chiffré: {:?}", encrypted_message);

    let decrypted_message = encryptor.decrypt_message(&shared_secret, &encrypted_message);
    println!("Message déchiffré: {}", decrypted_message);
    let mut user = User::init_user().unwrap();

    loop {
        println!("Silent:\nadd [name] | remove [name] | list | exit");
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).unwrap();
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
                } else {
                    let _ = user.add_newcontact(input[1], local_ip().unwrap());
                }
            }
            "remove" => {
                if input.len() != 2 || input[1].trim().is_empty() {
                    println!("Remove need an argument [name]");
                } else {
                    user.remove_by_name(input[1]).ok();
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
            }
        }
        println!("");
    }
}
