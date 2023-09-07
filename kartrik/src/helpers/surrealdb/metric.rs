use super::Record;
use crate::models::metric::Metric;
use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

pub(crate) async fn add_metric(metric: Metric, db: &Surreal<Client>) -> surrealdb::Result<String> {
    // Create a new metric with a random id
    let _created: Record = db
        .create(("metric", &metric.identifier))
        .content(metric)
        .await?;
    match _created.id.id {
        Id::String(id_str) => Ok(id_str),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    }
}

pub(crate) async fn get_metrics(db: &Surreal<Client>) -> surrealdb::Result<Vec<Metric>> {
    // Select all records
    let metrics: Vec<Metric> = db.select("metric").await?;

    Ok(metrics)
}

pub(crate) async fn get_metrics_for_control(
    control_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Metric>> {
    let metrics: Vec<Metric> = db
        .query("SELECT * FROM metric WHERE id INSIDE $control_id<-measures.in")
        .bind((
            "control_id",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(metrics)
}

pub(crate) async fn get_metric(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Metric>> {
    let metric: Option<Metric> = db.select(("metric", id)).await?;

    Ok(metric)
}

pub(crate) async fn associate_metric(
    metric_id: &str,
    control_id: &str,
    coverage: u8,
    db: &Surreal<Client>,
) -> surrealdb::Result<()> {
    let response = db
        // .query("RELATE ((SELECT id FROM tag WHERE identifier = $tid)[0].id)->tags->((SELECT id FROM control WHERE identifier = $cid)[0].id)")
        .query("RELATE $mid->measures->$cid SET coverage = $coverage")
        .bind((
            "cid",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .bind((
            "mid",
            Thing {
                id: Id::String(metric_id.to_string()),
                tb: "metric".to_string(),
            },
        ))
        .bind(("coverage", coverage))
        .await?;
    response.check().map(|_| ())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// Dummy struct to make deserializing from the result in
/// get_metric_control_association easier
struct Helper {
    metric: Thing,
    control: Thing,
    coverage: u8,
}

pub(crate) async fn get_metric_control_association(
    db: &Surreal<Client>,
) -> surrealdb::Result<BTreeMap<String, Vec<(String, u8)>>> {
    let mut response = db
        .query("SELECT in as metric, out as control, coverage FROM measures")
        .await?;
    let results: Vec<Helper> = response.take(0)?;
    let mut result: BTreeMap<String, Vec<(String, u8)>> = BTreeMap::new();
    results.iter().for_each(|obj| {
        let key = match &obj.metric.id {
            Id::String(str) => str.into(),
            _ => panic!("A metric's id must be a string"),
        };
        let value = match &obj.control.id {
            Id::String(str) => (str.into(), obj.coverage),
            _ => panic!("A metric's id must be a string"),
        };
        if let Some(prev_value) = result.get_mut(&key) {
            // If the key is already present, push the value into the existing vector
            prev_value.push(value);
        } else {
            // If the key is not present, insert a new element into the BTreeMap
            let new_value = vec![value];
            result.insert(key, new_value);
        }
    });

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct HelperR {
    control: Thing,
    coverage: u8,
}

/// Given a metric id, it returns a map that associates the control_ids
/// measured by this metric to their coverage
pub(crate) async fn get_coverage_for_metric(
    metric_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<HashMap<String, u8>> {
    let res: Vec<HelperR> = db
        .query("SELECT coverage, out as control FROM $metric_id->measures")
        .bind((
            "metric_id",
            Thing {
                id: Id::String(metric_id.to_string()),
                tb: "metric".to_string(),
            },
        ))
        .await?
        .take(0)?;
    Ok(HashMap::from_iter(res.iter().map(|r| {
        let id = match &r.control.id {
            Id::String(s) => s,
            _ => panic!("We only use string ids for controls"),
        };
        (id.to_owned(), r.coverage)
    })))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct HelperR2 {
    ids: Vec<Thing>,
}

/// Given a metric id, it returns a vector with the ids of the tags of
/// all the controls this metric measures
pub(crate) async fn get_tags_for_metric(
    metric_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Tag>> {
    let res: Vec<Tag> = db
        // .query("SELECT id FROM $metrics->measures.out<-tags.in")
        // .query("SELECT array::group(id) as ids FROM $metrics->measures.out<-tags.in.id GROUP ALL")
        .query("SELECT * FROM $metrics->measures.out<-tags.in.*")
        //You can also use a vec<Thing> and get the aggregate tags of more metrics at once
        .bind((
            "metrics",
            Thing {
                id: Id::String(metric_id.to_string()),
                tb: "metric".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(res)
}
