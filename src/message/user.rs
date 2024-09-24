use std::net::IpAddr;
use uuid::Uuid;

#[derive(PartialEq, Clone)]
pub struct User {
	name: String,
	id: Uuid,
	ip: IpAddr,
	contacts: Option<Vec<User>>,
}

#[derive(PartialEq)]
enum UserErr {
	UserNotFound,
	FileNotFound,
	FileCreation,
	FilePermission,
	FailedRead,
}

impl User {
	fn new(new_name: String, new_ip: IpAddr) -> Result<Self, UserErr>
	{
		let new_user = User {
			name: new_name,
			id: Uuid::new_v4(),
			ip: new_ip,
			contacts: Some(Vec::new()),
		};
		
		Ok(new_user)
	}
	fn update_contact(&mut self) -> Result<(), UserErr>{
		let users = user_file::get_contacts()?;
		if users == None {
			return Ok(());
		} else if self.contacts == None {
			self.contacts = users;
			return Ok(());
		}
		for user in users.unwrap().iter() {
			match self.search_by_id(user.id) {
				Ok(_) => (),
				Err(UserErr::UserNotFound) => {
					self.contacts.as_mut().unwrap().push((*user).clone())
				},
				Err(err) => { return Err(err) },
			}
		}
		Ok(())
	}
	fn addcontact(&mut self, new_contact: Self) -> Result<(), UserErr>{
		if self.contacts == None {
			self.contacts = Some(Vec::new());
			self.contacts.as_mut().unwrap().push(new_contact);
			return Ok(());
		}
	
		match self.search_by_id(new_contact.id) {
			Ok(_) => Ok(()),
			Err(UserErr::UserNotFound) => {
				self.contacts.as_mut().unwrap().push(new_contact);
				Ok(())
			},
			Err(err) => Err(err),
		}
	}
	fn search_by_name(&mut self, name: String) -> Result<Self, UserErr> {
		let contact = self.contacts.as_mut();

		if contact == None {
			return Err(UserErr::UserNotFound);
		} else {
			for user in contact.unwrap().iter() {
				if user.name == name {
					return Ok((*user).clone());
				}
			}
			return Err(UserErr::UserNotFound);
		}
	}
	fn search_by_id(&mut self, id: Uuid) -> Result<Self, UserErr> {
		let contact = self.contacts.as_mut();

		if contact == None {
			return Err(UserErr::UserNotFound);
		} else {
			for user in contact.unwrap().iter() {
				if user.id == id {
					return Ok((*user).clone());
				}
			}
			return Err(UserErr::UserNotFound);
		}
	}
	fn search_by_ip(&mut self, ip: IpAddr) -> Result<Self, UserErr> {
		let user_file = user_file::userfile_path()?;
		let contact = self.contacts.as_mut();

		if contact == None {
			return Err(UserErr::UserNotFound);
		} else {
			for user in contact.unwrap().iter() {
				if user.ip == ip {
					return Ok((*user).clone());
				}
			}
			return Err(UserErr::UserNotFound);
		}
	}
	fn remove_by_name(&mut self, name: String) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.name == name {
				self.contacts.as_mut().unwrap().remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_ip(&mut self, ip: IpAddr) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.ip == ip {
				self.contacts.as_mut().unwrap().remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_id(&mut self, id: Uuid) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.id == id
			{
				self.contacts.as_mut().unwrap().remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
}

mod user_file {

	static USER_FILE: &str = ".users";
	use uuid::Uuid;

use super::{User, UserErr};
    use std::{
		fs::{self, metadata, write, File},
		io::{self, Error, ErrorKind},
		os::unix::fs::PermissionsExt,
		path::Path
	};

	pub fn userfile_path() -> Result<&'static Path, UserErr>
	{
		let path = Path::new(USER_FILE);

		if path.exists() == false {
			let _ = match File::create_new(path) {
				Ok(_) => (),
				Err(_) => { return Err(UserErr::FileCreation); },
			};
			return Ok(path);
		} else {
			let perm = metadata(path).expect("Failed to open Users file").permissions().mode();

			if perm & 0o400 == 0 {
				return Err(UserErr::FilePermission);
			} else if perm & 0o200 == 0 {
				return Err(UserErr::FilePermission);
			} else {
				return Ok(path);
			}
		}
	}
	pub fn get_contacts() -> Result<Option<Vec<User>>, UserErr> {
		let mut users = Vec::new();
		let file_path = userfile_path()?;
		let content: String = match fs::read_to_string(file_path) {
			Ok(content) => content,
			Err(_) => {return Err(UserErr::FailedRead);},
		};

		let content = content.split("\n");

		for str in content {
			let mut infos = str.trim().split(':');
			if infos.clone().any(|str| str.is_empty()) || infos.clone().count() != 4 {
				continue ;
			}
			if infos.nth(0).unwrap() != "@" {
				continue ;
			}
			let new_user = User
			{
				name: infos.nth(1).unwrap().to_string(),
				id: match Uuid::parse_str(infos.nth(2).unwrap()) {
					Ok(id) => id,
					Err(_) => { continue ;},
				},
				ip: match infos.nth(3).unwrap().parse() {
					Ok(ip) => ip,
					Err(_) => {continue ;},
				},
				contacts: None,
			};
			users.push(new_user);
		}

		Ok(Some(users))
	}
	pub fn write_user(user: User) -> Result<(), Error>
	{
		let path = Path::new(USER_FILE);
		if path.exists() == false {
			return Err(
				io::Error::new(
					ErrorKind::NotFound,
					Error::other("Users File not found")));
		}
		let info = format!(
			"@:{}:{}:{}:@",
			user.name,
			user.id,
			user.ip,
		);
		write(path, info)?;
		Ok(())
	}
}
