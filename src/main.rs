mod message;
extern crate hex;

use std::io::{self, Write};

use message::helper::Message;

mod crypto;
use crypto::encrypt::Encryptor;

fn main() {
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
}
