use crate::message::helper::Message;
use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, CHACHA20_POLY1305};
use ring::agreement::{self, UnparsedPublicKey};
use ring::rand::{SecureRandom, SystemRandom};

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
    pub fn encrypt_message(&self, shared_secret: &[u8; 32]) -> Vec<u8> {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, shared_secret).unwrap();
        let key = LessSafeKey::new(unbound_key);

        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes).unwrap();
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut messag_bytes = self.message.message.clone();
        messag_bytes.extend_from_slice(&[0u8; aead::CHACHA20_POLY1305.tag_len()]);

        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut messag_bytes)
            .unwrap();

        let mut ciphertext = nonce_bytes.to_vec();
        ciphertext.extend_from_slice(&messag_bytes);
        ciphertext
    }
}
