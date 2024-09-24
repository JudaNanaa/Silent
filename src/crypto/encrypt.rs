use crate::message::helper::Message;
use ring::agreement::{self, UnparsedPublicKey};
use ring::rand::SystemRandom;

pub struct Keys {
    private: agreement::EphemeralPrivateKey,
    public: agreement::PublicKey,
}

impl Keys {
    pub fn new() -> Self {
        let rng = SystemRandom::new();
        let private = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).unwrap();
        let public = private.compute_public_key().unwrap();
        Keys { private, public }
    }
}

pub struct Encryptor {
    key: Keys,
    message: Message,
}

impl Encryptor {
    pub fn new(msg: Message) -> Self {
        let key = Keys::new();
        let message = msg;
        Encryptor { key, message }
    }
    pub fn compute_sahred_secret(self, peer_public_key: &[u8]) -> [u8; 32] {
        let peer_pub_key = UnparsedPublicKey::new(&agreement::X25519, peer_public_key);
        let shared_secret = agreement::agree_ephemeral(
            self.key.private,
            &peer_pub_key,
            ring::error::Unspecified,
            |shared_key_material| {
                let mut shared_secret = [0u8; 32];
                shared_secret.copy_from_slice(shared_key_material);
                Ok(shared_secret)
            },
        )
        .unwrap();
        shared_secret
    }
}
