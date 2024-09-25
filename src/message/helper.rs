use std::time;

use chrono::{Date, DateTime, Local, TimeZone};

pub struct Message {
    pub message: String,
    send_time: DateTime<Local>,
}

impl Message {
    pub fn new(msg: String) -> Self {
        Message {
            message: msg,
            send_time: Local::now(),
        }
    }
}
