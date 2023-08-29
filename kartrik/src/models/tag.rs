use crate::helpers::cryptography::generate_random_string;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug)]
#[ts(export, export_to = "../frontend/src/lib/models/bindings/")]
pub(crate) struct Tag {
    pub(crate) identifier: String,
    pub(crate) name: String,
    pub(crate) color_hex: String,
    pub(crate) created_at: String,
}

impl Tag {
    pub(crate) fn new(name: &str, color_hex: &str) -> Self {
        Self {
            name: name.into(),
            color_hex: color_hex.into(),
            created_at: DateTime::<Utc>::default().timestamp_nanos().to_string(),
            identifier: generate_random_string(16),
        }
    }
}
