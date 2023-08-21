use super::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// A kartik final user
pub(crate) struct User {
    /// The username of the user in kartik. The username shall be unique.
    /// It must consist of 3-30 alphanumeric characters or underscores.
    /// Cannot start nor finish with an underscore.
    pub(crate) username: String,
    /// The blake3 salted hash of the password of this user
    pub(crate) password_hash: String,
    /// The salt for the blake3 hash
    pub(crate) password_salt: String,
    /// All the roles this user has (e.g., admin)
    pub(crate) roles: Vec<Role>,
}
