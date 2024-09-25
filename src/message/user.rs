use std::{io, net::IpAddr, ops::Index};
use local_ip_address::linux::local_ip;
use uuid::Uuid;


#[derive(Debug, PartialEq, Clone)]
pub struct User {
	pub name: String,
	id: Uuid,
	ip: IpAddr,
	first_user: bool,
	contacts: Option<Vec<User>>,
}

#[derive(PartialEq, Debug)]
#[allow(unused)]
pub enum UserErr {
	UserNotFound,
	FunctionFailed,
	FileNotFound,
	FileCreation,
	FailedRead,
	FailedWrite,
	FailedOpen,
	Other,
}

#[allow(unused)]
impl User {
	pub fn init_user() -> Result<Self, UserErr> {
		let user = match user_file::search_first_user() {
			Ok(mut user) => { user.update_contact(); return Ok(user) },
			Err(UserErr::UserNotFound) => {
				let mut usernamne = String::new();

				loop {
					match io::stdin().read_line(&mut usernamne) {
						Ok(_) => (),
						Err(_) => {
							return Err(UserErr::FunctionFailed);
						}
					}
					if usernamne.trim().is_empty() {
						continue;
					}
				}
				Self {
					name: usernamne.to_string(),
					id: Uuid::new_v4(),
					ip: match local_ip() {
						Ok(ip) => ip,
						Err(_) => {
							return Err(UserErr::FunctionFailed)
						},
					},
					contacts: Some(Vec::new()),
					first_user: true,
				}
			},
			Err(err) => { return Err(err) },
		};
		user_file::write_first_user(user.clone())?;
		Ok(user)
	}
	fn new(&mut self, new_name: &str, new_ip: IpAddr) -> Result<Self, UserErr>
	{
		if self.search_by_name(new_name) != Err(UserErr::UserNotFound) {
			println!("A contact named '{}' already exists\nSelect an option:\n\n{}\n{}\n{}\n",
			new_name,
			"1. Choose an other name",
			"2. Delete and replace the contact",
			"3. Stop this contact creation");

			let mut input = String::new();
			let mut option: i32;
			loop {
				input.clear();
				match io::stdin().read_line(&mut input) {
					Ok(_) => (),
					Err(_) => { return Err(UserErr::FunctionFailed) },
				}
				option = match input.trim().parse() {
					Ok(num) => num,
					Err(_) => { println!("Bad input, retype it"); continue; }
				};
				if [1, 2, 3].contains(&option) == false {
					println!("Option {} doesn't exists, retype it", option);
					continue;
				}
				break;
			}
			match option {
				1 => {
					println!("Enter an other name");
					let mut input = String::new();
					loop {
							input.clear();
							match io::stdin().read_line(&mut input) {
							Ok(_) => (),
							Err(_) => { return Err(UserErr::FunctionFailed) },
						}
						if input.trim().is_empty() {
							continue;
						} else if input.trim() == new_name {
							println!("Same name, enter an other");
							continue;
						}
						break;
					}
					return self.new(input.as_str(), new_ip);
				},
				2 => {
					self.remove_by_name(new_name);
				},
				3 => {
					return (Err(UserErr::UserNotFound));
				},
				_ => (),
			}
		}
		let new_user = User {
			name: new_name.to_string(),
			id: Uuid::new_v4(),
			ip: new_ip,
			contacts: None,
			first_user: false,
		};
		user_file::write_user(new_user.clone())?;
		Ok(new_user)
	}
	fn update_contact(&mut self) -> Result<(), UserErr>{
		let users = user_file::get_contacts()?;
		if users == None {
			return Ok(());
		} else if self.contacts.as_mut().expect("called addcontact on a contact").is_empty()  {
			self.contacts = users;
			return Ok(());
		}
		for user in users.unwrap().iter() {
			match self.search_by_name(user.name.as_str()) {
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
		if self.contacts.as_mut().expect("called addcontact on a contact").is_empty() {
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
	pub fn add_newcontact(&mut self, name: &str, ip: IpAddr) -> Result<(), UserErr>{
		let new_contact = match self.new(name, ip) {
			Ok(user) => user,
			Err(UserErr::UserNotFound) => { return Ok(()) },
			Err(err) => { return Err(err)} ,
		};

		if self.contacts.as_mut().expect("called addcontact on a contact").is_empty() {
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
	pub fn search_by_name(&mut self, name: &str) -> Result<Self, UserErr> {
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
	pub fn search_by_id(&mut self, id: Uuid) -> Result<Self, UserErr> {
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
	pub fn search_by_ip(&mut self, ip: IpAddr) -> Result<Self, UserErr> {
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
	pub fn remove_by_name(&mut self, name: &str) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.name == name {
				self.remove_user(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	pub fn remove_by_ip(&mut self, ip: IpAddr) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.ip == ip {
				self.remove_user(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	pub fn remove_by_id(&mut self, id: Uuid) -> Result<(), UserErr>
	{
		let mut i = 0;

		for user in self.contacts.as_mut().unwrap().iter() {
			if user.id == id
			{
				self.remove_user(i);
				return Ok(());
			}
			i += 1;
		}
		Err(UserErr::UserNotFound)
	}
	fn remove_user(&mut self, index: usize) -> Result<(), UserErr> {
		self.contacts.as_mut().unwrap().remove(index);
		user_file::rewrite_contacts(self)
	}
	pub fn print_contacts(&mut self) {
		for contact in self.contacts.as_mut().unwrap().iter() {
			println!("-{}:\n\t{}", contact.name, contact.ip);
		}
	}
}

mod user_file {

	static USER_FILE: &str = ".users";
	use uuid::Uuid;

use super::{User, UserErr};
    use std::{
		fs::{metadata, read_to_string, File, OpenOptions},
		os::unix::fs::PermissionsExt,
		path::Path,
		io::Write
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

			if perm & 0o400 == 0 { // write permission
				return Err(UserErr::FailedWrite);
			} else if perm & 0o200 == 0 { // read permission
				return Err(UserErr::FailedRead);
			} else {
				return Ok(path);
			}
		}
	}
	pub fn get_contacts() -> Result<Option<Vec<User>>, UserErr> {
		let mut users = Vec::new();
		let file_path = userfile_path()?;
		let content: String = match read_to_string(file_path) {
			Ok(content) => content,
			Err(_) => {return Err(UserErr::FailedRead);},
		};

		let content: Vec<&str> = content.split("\n").collect();

		for str in content {
			let infos: Vec<&str> = str.trim().split(':').collect();
			if infos.iter().any(|str| str.is_empty())
			|| infos.iter().count() != 4 {
				continue ;
			}
			if infos[0] != "@" {
				continue ;
			}
			let new_user = User
			{
				name: infos[1].to_string(),
				id: match Uuid::parse_str(infos[2]) {
					Ok(id) => id,
					Err(_) => { continue ;},
				},
				ip: match infos[3].parse() {
					Ok(ip) => ip,
					Err(_) => {continue ;},
				},
				contacts: None,
				first_user: false,
			};
			users.push(new_user);
		}

		Ok(Some(users))
	}
	pub fn write_user(user: User) -> Result<(), UserErr>
	{
		let path = userfile_path()?;
		if already_writed(user.clone(), path) {
			return Ok(());
		}

		let mut file = match
		OpenOptions::new().append(true)
		.write(true).read(true)
		.create(true).open(path) {
			Ok(file) => file,
			Err(_) => { return Err(UserErr::FailedOpen) },
		}; let info = format!(
			"@:{}:{}:{}",
			user.name,
			user.id,
			user.ip
		);

		match writeln!(file, "{}", info) {
			Ok(()) => Ok(()),
			Err(_) => Err(UserErr::FailedRead),
		}
	}
	fn already_writed(user: User, path: &Path) -> bool
	{
		let content = match read_to_string(path) {
			Ok(c) => c,
			Err(_) => { return false },
		};
		let content: Vec<&str> = content.split("\n").collect();

		for str in content {
			let infos: Vec<&str> = str.trim().split(':').collect();
			if infos.iter().any(|str| str.is_empty())
			|| infos.iter().count() != 4 {
				continue ;
			}else if infos[0] != "@" {
				continue ;
			}
			if infos[1] == user.name {
				return true;
			}
		}
		false
	}
	pub fn write_first_user(user: User) -> Result<(), UserErr>
	{
		let path = userfile_path()?;

		let mut file = match
		OpenOptions::new().append(true)
		.write(true).read(true)
		.create(true).open(path) {
			Ok(file) => file,
			Err(_) => { return Err(UserErr::FailedOpen) },
		}; let info = format!(
			"$:{}:{}:{}",
			user.name,
			user.id,
			user.ip
		);

		match writeln!(file, "{}", info) {
			Ok(()) => Ok(()),
			Err(_) => Err(UserErr::FailedRead),
		}
	}
	pub fn rewrite_contacts(first_user: &mut User) -> Result<(), UserErr> {
		if first_user.first_user == false {
			return Err(UserErr::Other);
		}
		let mut infos: String = String::from(format!("$:{}:{}:{}\n", first_user.name, first_user.id, first_user.ip));
		for user in first_user.contacts.as_mut().unwrap().iter() {
			infos.push_str(format!("@:{}:{}:{}\n", user.name, user.id, user.ip).as_str());
		}
		let mut file = match File::open(userfile_path()?) {
			Ok(file) => file,
			Err(_) => { return Err(UserErr::FailedOpen) },
		};
		write!(file, "{}", infos);
		Ok(())
	}
	pub fn search_first_user() -> Result<User, UserErr> {
		let file_path = userfile_path()?;
		let content: String = match read_to_string(file_path) {
			Ok(content) => content,
			Err(_) => {return Err(UserErr::FailedRead);},
		};
		
		let content: Vec<&str> = content.split("\n").collect();
		
		for str in content {
			let infos: Vec<&str> = str.trim().split(':').collect();
			if infos.iter().any(|str| str.is_empty())
			|| infos.iter().count() != 4 {
				continue ;
			}
			if infos[0] != "$" {
				continue ;
			}
			return Ok(User {
				name: infos[1].to_string(),
				id: match Uuid::parse_str(infos[2]) {
					Ok(id) => id,
					Err(_) => { continue },
				},
				ip: match infos[3].parse() {
					Ok(ip) => ip,
					Err(_) => { continue },
				},
				contacts: Some(Vec::new()),
				first_user: true,
			});
		}
		Err(UserErr::UserNotFound)
	}
}
