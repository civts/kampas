use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct User {
    #[field(validate = len(3..30))]
    pub(crate) username: String,
    #[field(validate = len(3..30))]
    pub(crate) password_hash: String,
    #[field(validate = len(3..30))]
    pub(crate) password_salt: String,
}
