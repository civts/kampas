use serde::Deserialize;
use surrealdb::sql::Thing;

pub(crate) mod connection;
pub(crate) mod control;
pub(crate) mod metric;
pub(crate) mod ranking;
pub(crate) mod tag;
pub(crate) mod token;
pub(crate) mod user;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
