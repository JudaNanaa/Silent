use ring::aead::{self, Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, CHACHA20_POLY1305};
use ring::agreement::{self, UnparsedPublicKey};
use ring::rand::{SecureRandom, SystemRandom};
extern crate hex;

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
}

impl Encryptor {
    pub fn new() -> Self {
        let key = Keys::new();
        Encryptor { key }
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
    pub fn encrypt_message(&self, msg: String, shared_secret: &[u8; 32]) -> String {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, shared_secret).unwrap();
        let key = LessSafeKey::new(unbound_key);

        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes).unwrap();
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut messag_bytes = msg.clone().into_bytes();

        let tag_len = aead::CHACHA20_POLY1305.tag_len();
        messag_bytes.extend(vec![0u8; tag_len]);

        key.seal_in_place_append_tag(nonce, Aad::empty(), &mut messag_bytes)
            .unwrap();

        let mut ciphertext = nonce_bytes.to_vec();
        ciphertext.extend_from_slice(&messag_bytes);
        hex::encode(ciphertext)
    }
    pub fn decrypt_message(&self, shared_secret: &[u8; 32], ciphertext_hex: &str) -> String {
        let ciphertext = hex::decode(ciphertext_hex).unwrap();
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, shared_secret).unwrap();
        let key = LessSafeKey::new(unbound_key);

        let (nonce_bytes, encrypted_message) = ciphertext.split_at(12);
        let nonce = Nonce::assume_unique_for_key(<[u8; 12]>::try_from(nonce_bytes).unwrap());
        let mut decrypted_message = encrypted_message.to_vec();

        key.open_in_place(nonce, Aad::empty(), &mut decrypted_message)
            .unwrap();

        let tag_len = aead::CHACHA20_POLY1305.tag_len();
        let original_msg_len = decrypted_message.len() - tag_len;
        decrypted_message.truncate(original_msg_len);

        let message = String::from_utf8(decrypted_message).unwrap();
        message
    }
}
