use super::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// A kampas final user
pub(crate) struct User {
    /// The username of the user in kampas. The username shall be unique.
    /// It must consist of 3-30 alphanumeric characters or underscores.
    /// Cannot start nor finish with an underscore.
    pub(crate) username: String,
    /// The argon2 PHC hash of the password of this user
    pub(crate) password_hash: String,
    /// All the roles this user has (e.g., admin)
    pub(crate) roles: Vec<Role>,
}
