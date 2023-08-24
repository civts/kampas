use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::helpers::cryptography::generate_random_string;

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Control {
    pub(crate) identifier: String,
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
    pub(crate) progress: u8,
    pub(crate) created_at: String,
}

impl Control {
    pub(crate) fn new(title: &str, description: &str) -> Self {
        let created_at = DateTime::<Utc>::default().timestamp_nanos().to_string();
        let id = generate_random_string(16);
        Control {
            created_at,
            identifier: id,
            progress: 0,
            title: title.into(),
            description: description.into(),
        }
    }
}
