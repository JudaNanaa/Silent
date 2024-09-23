pub struct Keys {
    private: String,
    public: String,
}

impl Keys {
    pub fn new() -> Self {
        let private = String::new();
        let public = String::new();
        Keys {private, public}
    }
}

pub struct Encryptor {
    key: Keys,
    message: Message,
}

impl Encryptor {
    pub fn new() -> Self {
        let key = Keys::new();
        let message = Message::new();
        Encryptor { key, message }
    }

    pub fn
}
