use chrono::{DateTime, Days, Utc};
use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand_chacha::ChaCha20Core;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Token {
    pub(crate) token: String,
    pub(crate) user_id: String,
    pub(crate) expiry: DateTime<Utc>,
}

impl Token {
    pub(crate) fn new(user_id: &String) -> Self {
        Token {
            user_id: user_id.clone(),
            token: generate_random_string(64),
            expiry: chrono::offset::Utc::now()
                .checked_add_days(Days::new(7))
                .expect("Can add to current date"),
        }
    }
    pub(crate) fn is_expired(&self) -> bool {
        chrono::offset::Utc::now() > self.expiry
    }
}

fn generate_random_string(len: u16) -> String {
    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng); //Reseeding
    let mut random_string = String::new();

    for _ in 0..(len / 2) {
        let random_byte: u8 = reseeding_rng.next_u32() as u8;
        random_string.push_str(&format!("{:02x}", random_byte));
    }

    random_string
}
