use blake3::Hasher;
use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand_chacha::ChaCha20Core;

pub(crate) fn generate_random_string(len: u16) -> String {
    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng); //Reseeding
    let mut random_string = String::new();

    for _ in 0..(len / 2) {
        let random_byte: u8 = reseeding_rng.next_u32() as u8;
        random_string.push_str(&format!("{:02x}", random_byte));
    }

    random_string
}

/// Returns the hash of a password (with salt) using the Blake3 algorithm
pub(crate) fn hash_salted_password(password: &str, salt: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(salt.as_bytes());
    hasher.update(password.as_bytes());
    let hash = hasher.finalize();

    hash.to_hex().to_string()
}
