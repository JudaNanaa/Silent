use std::{fs::File, net::IpAddr, path::Path};
use chrono::{DateTime, Local};
use local_ip_address::local_ip;
use uuid::Uuid;


#[derive(PartialEq)]
pub struct User {
	name: String,
	id: Uuid,
	ip: IpAddr,
	friends: Vec<User>,
}

enum UserErr {
	UserNotFound,
	UserFileNotFound,
}

pub struct Message {
	message: String,
	send_time: DateTime<Local>,
}

enum MessageErr {
	ReiceverNotFound,
	ReiceverUnreachable,

}

impl User {
	fn new(new_name: String, new_ip: IpAddr) -> Result<Self, UserErr>
	{
		let new_user = User {
			name: new_name,
			id: Uuid::new_v4(),
			ip: new_ip,
			friends: Vec::new(),
		};
		
		Ok(new_user)
	}
	fn addfriend(&mut self, friend: Self) {
		self.friends.push(friend);
	}
	// fn search_by_name(String)
	fn remove_by_name(&mut self, name: String) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.friends.iter() {
			if self.friends[i].name == name {
				self.friends.remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_ip(&mut self, ip: IpAddr) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.friends.iter() {
			if self.friends[i].ip == ip {
				self.friends.remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_id(&mut self, id: Uuid) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.friends.iter() {
			if self.friends[i].id == id {
				self.friends.remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
}

impl Message {
	fn new(msg: String) -> Self {
		Message {
			message: msg,
			send_time: Local::now(),
		}
	}
	// fn send(msg: self, receiver: User) -> Result<(), Me>
}

mod user_file {
    use std::{fs::{write, File}, io::Error, path::Path};

    use super::User;

	static USER_FILE: &str = ".users";

	fn create_userfile() -> Result<File, Error>
	{
		let path = Path::new(USER_FILE);
		if path.exists() {
			return match File::open(path) {
				Ok(file) => Ok(file),
				Err(err) => Err(err),
			}
		} else {
			return match File::create_new(path) {
				Ok(file) => Ok(file),
				Err(err) => Err(err),
			}
		}
	}
	fn write_user(user: User) -> Result<_, Error>
	{
		let path = Path::new(USER_FILE);
		let info = format!(
			"{}:{}:{}:{}",
			user.id,
			user.ip,
			user.friends,
		)
		write(path, contents)

		Ok(())
	}
}
