use crate::helpers::cryptography::generate_random_string;
use chrono::{DateTime, Days, Utc};
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
