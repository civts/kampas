use crate::helpers::cryptography::generate_random_string;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS, Deserialize)]
#[serde(crate = "rocket::serde")]
#[ts(export, export_to = "../frontend/src/lib/models/bindings/")]
pub(crate) struct Measure {
    pub(crate) identifier: String,
    pub(crate) title: String,
    pub(crate) description: String,
    /// How much this is complete [0-100]
    pub(crate) progress: u8,
    /// How many weeks do we need to implement this company-wide?
    pub(crate) effort: u8,
}

impl Measure {
    pub(crate) fn new(title: &str, description: &str, effort: Option<u8>) -> Self {
        let id = generate_random_string(16);
        Measure {
            identifier: id,
            progress: 0,
            title: title.into(),
            description: description.into(),
            effort: effort.unwrap_or(1),
        }
    }
}
