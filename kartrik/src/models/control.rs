use crate::helpers::cryptography::generate_random_string;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS, Deserialize)]
#[serde(crate = "rocket::serde")]
#[ts(export, export_to = "../frontend/src/lib/models/bindings/")]
pub(crate) struct Control {
    pub(crate) identifier: String,
    pub(crate) title: String,
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
