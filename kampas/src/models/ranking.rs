use crate::helpers::cryptography::generate_random_string;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, TS, Deserialize)]
#[serde(crate = "rocket::serde")]
#[ts(export, export_to = "../frontend/src/lib/models/bindings/")]
pub(crate) struct Ranking {
    /// Identifiers of the measures!
    pub(crate) measures: Vec<String>,
    /// Identifiers of the controls
    pub(crate) controls: Vec<String>,
    /// Username of who created this one
    pub(crate) created_by: String,
    /// Timestamp of when it was created
    pub(crate) created_at: String,
    /// How it was created
    pub(crate) ordering: RankOrdering,
    /// The minimum coverage value a control must
    /// have to be considered complete
    pub(crate) minimum_coverage: u8,
    pub(crate) identifier: String,
    pub(crate) name: String,
}

impl Ranking {
    pub(crate) fn new(
        measures: Vec<String>,
        controls: Vec<String>,
        created_by: &str,
        ordering: RankOrdering,
        name: &str,
        minimum_coverage: u8,
    ) -> Self {
        let created_at = chrono::offset::Utc::now().timestamp_millis().to_string();
        let id = generate_random_string(16);
        Self {
            created_at,
            identifier: id,
            created_by: created_by.to_string(),
            name: name.to_string(),
            measures,
            controls,
            ordering,
            minimum_coverage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS, Eq, Hash)]
#[serde(crate = "rocket::serde")]
#[ts(export, export_to = "../frontend/src/lib/models/bindings/")]
pub(crate) enum RankOrdering {
    /// Generated by the greedy weighted set cover algorithm
    #[serde(rename = "greedy_w_set_cover")]
    GreedyWeightedSetCover,

    /// Someone manually ordered one or more of the measures
    #[serde(rename = "manual")]
    Manual,
}
