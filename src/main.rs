#[allow(unused_variables)]
mod message;

use local_ip_address::local_ip;
use message::user::*;

fn main() {
    let mut new_user = User::init_user("Idrissa").unwrap();
	new_user.add_newcontact("Moussa", local_ip().unwrap()).unwrap();
	new_user.add_newcontact("Imad", local_ip().unwrap()).unwrap();
	new_user.add_newcontact("Moha", local_ip().unwrap()).unwrap();
	println!("{} => {:#?}",new_user.name, new_user);
}
