use std::collections::HashMap;

use crate::models::control::Control;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

use super::Record;

pub(crate) async fn add_control(control: Control, db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Create a new control with a random id
    let _created: Record = db
        .create(("control", &control.identifier))
        .content(control)
        .await?;
    Ok(())
}

pub(crate) async fn get_controls(db: &Surreal<Client>) -> surrealdb::Result<Vec<Control>> {
    // Select all records
    let controls: Vec<Control> = db.select("control").await?;

    Ok(controls)
}

pub(crate) async fn get_controls_for_measure(
    measure_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Control>> {
    let controls: Vec<Control> = db
        .query("SELECT * FROM control WHERE id INSIDE $measure_id->satisfies.out")
        .bind((
            "measure_id",
            Thing {
                id: Id::String(measure_id.to_string()),
                tb: "measure".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(controls)
}

pub(crate) async fn get_control(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Control>> {
    let control: Option<Control> = db.select(("control", id)).await?;

    Ok(control)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper {
    coverage: u8,
    progress: u8,
}

/// Returns the completion of the given control, which is computed by
/// adding the contributions of all the associated measures.
pub(crate) async fn get_control_completion(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<f64> {
    let query_res: Vec<_Helper> = db
        .query("SELECT coverage, in.progress as progress FROM $cid<-satisfies")
        .bind((
            "cid",
            Thing {
                id: Id::String(id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?
        .take(0)?;
    let mut res = query_res.iter().fold(0f64, |prev, i| {
        prev + (i.progress as f64 * i.coverage as f64 / 100f64)
    });
    res = res.clamp(0f64, 100f64);

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper2 {
    coverage: Vec<u8>,
    progress: Vec<u8>,
}
pub(crate) async fn get_control_completion_batch(
    db: &Surreal<Client>,
    ids: Vec<String>,
) -> surrealdb::Result<Vec<f64>> {
    let query_res: Vec<_Helper2> = db
        .query("SELECT coverage, in.progress as progress FROM $cids<-satisfies")
        .bind((
            "cids",
            Vec::from_iter(ids.iter().map(|id| Thing {
                id: Id::String(id.to_string()),
                tb: "control".to_string(),
            })),
        ))
        .await?
        .take(0)?;
    let res = query_res
        .iter()
        .map(|h| {
            let mut result = 0f64;
            for i in 0..h.coverage.len() {
                let progress = *h.progress.get(i).expect("There should be a progress here");
                let coverage = *h.coverage.get(i).expect("There should be a coverage here");

                result = result + (progress as f64 * coverage as f64 / 100f64);
                result = result.clamp(0f64, 100f64);
            }
            result
        })
        .collect();

    Ok(res)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper3 {
    id: Thing,
    count: u64,
}
pub(crate) async fn get_measures_for_control_count_batch(
    db: &Surreal<Client>,
) -> surrealdb::Result<HashMap<String, u64>> {
    let controls: Vec<_Helper3> = db
        // .query("SELECT id FROM control WHERE id NOT IN (SELECT out FROM satisfies GROUP out).out")
        // .query("SELECT * FROM control WHERE count(<-satisfies) = 0")
        .query("SELECT id, count(<-satisfies) FROM control")
        .await?
        .take(0)?;

    let mut res = HashMap::<String, u64>::new();
    controls.iter().for_each(|h| {
        let id_str = match &h.id.id {
            Id::String(id_str) => id_str.clone(),
            Id::Number(id_numb) => id_numb.to_string(),
            _ => panic!("We don't do that here. We shall only use String IDs"),
        };
        res.insert(id_str, h.count);
    });

    Ok(res)
}
