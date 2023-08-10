use crate::helpers::cryptography::generate_random_string;
use chrono::{DateTime, Days, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct AuthToken {
    pub(crate) token: String,
    pub(crate) username: String,
    pub(crate) expiry: DateTime<Utc>,
}

impl AuthToken {
    pub(crate) fn new(username: &String) -> Self {
        AuthToken {
            username: username.clone(),
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
