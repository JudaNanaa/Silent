use std::net::IpAddr;
use uuid::Uuid;

#[derive(PartialEq)]
pub struct User {
	name: String,
	id: Uuid,
	ip: IpAddr,
	contacts: Option<Vec<User>>,
}

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
	fn addfriend(&mut self, contact: Self) {
		if self.contacts == None {
			self.contacts = Some(Vec::new()).unwrap().push(contact);
		} else {
			self.contacts.unwrap().push(contact);
		}
	}
	fn search_by_name(name: String) -> Result<Self, UserErr> {
		let user_file = user_file::userfile_path()?;


	}
	fn remove_by_name(&mut self, name: String) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.contacts.iter() {
			if self.contacts[i].name == name {
				self.contacts.remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_ip(&mut self, ip: IpAddr) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.contacts.iter() {
			if self.contacts[i].ip == ip {
				self.contacts.remove(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_by_id(&mut self, id: Uuid) -> Result<(), UserErr>
	{
		let mut i = 0;

		for _ in self.contacts.iter()
		{
			if self.contacts[i].id == id
			{
				self.contacts.remove(i);
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
	pub fn get_users() -> Result<Option<Vec<User>>, UserErr> {
		let mut users = Vec::new();
		let file_path = userfile_path()?;
		let content: String = match fs::read_to_string(file_path) {
			Ok(content) => content,
			Err(_) => {return Err(UserErr::FailedRead);},
		};

		let content = content.split("\n");

		for str in content {
			let mut infos = str.trim().trim_matches('@').split(':');
			if infos.clone().any(|str| str.is_empty()) || infos.clone().count() != 3 {
				continue ;
			}
			let new_user = User
			{
				name: infos.nth(0).unwrap().to_string(),
				id: match Uuid::parse_str(infos.nth(1).unwrap()) {
					Ok(id) => id,
					Err(_) => { continue ;},
				},
				ip: match infos.nth(2).unwrap().parse() {
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
