use chrono::{DateTime, Local};

pub struct Message {
	message: String,
	send_time: DateTime<Local>,
}

enum MessageErr {
	ReiceverNotFound,
	ReiceverUnreachable,
}

impl Message {
	fn new(msg: String) -> Self
	{
		Message {
			message: msg,
			send_time: Local::now(),
		}
	}
	fn send(msg: self, receiver: User) -> Result<(), MessageErr>
	{
		

	}
}
