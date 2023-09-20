use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::{rngs::adapter::ReseedingRng, RngCore, SeedableRng};
use rand_chacha::ChaCha20Core;

pub(crate) fn generate_random_string(len: u16) -> String {
    let prng = ChaCha20Core::from_entropy();
    //TODO: cache this value and make it reseed periodically
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng); //Reseeding
    let mut random_string = String::new();

    for _ in 0..(len / 2) {
        let random_byte: u8 = reseeding_rng.next_u32() as u8;
        random_string.push_str(&format!("{:02x}", random_byte));
    }

    random_string
}

/// Returns the hash of a password using the Argon2id algorithm
pub(crate) fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Can hash the password")
        .to_string();

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash).expect("Is a valid password hash");
    assert!(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());

    password_hash
}
