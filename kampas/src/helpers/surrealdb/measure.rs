use super::Record;
use crate::models::measure::Measure;
use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

pub(crate) async fn add_measure(
    measure: Measure,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new measure with a random id
    let _created: Record = db
        .create(("measure", &measure.identifier))
        .content(measure)
        .await?;
    let s = match _created.id.id {
        Id::String(id_str) => id_str,
        Id::Number(id) => id.to_string(),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    };
    Ok(s)
}

pub(crate) async fn get_measures(db: &Surreal<Client>) -> surrealdb::Result<Vec<Measure>> {
    // Select all records
    let measures: Vec<Measure> = db.select("measure").await?;

    Ok(measures)
}

pub(crate) async fn get_measures_for_control(
    control_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Measure>> {
    let measures: Vec<Measure> = db
        .query("SELECT * FROM measure WHERE id INSIDE $control_id<-satisfies.in")
        .bind((
            "control_id",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(measures)
}

pub(crate) async fn get_measure(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Measure>> {
    let measure: Option<Measure> = db.select(("measure", id)).await?;

    Ok(measure)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper {
    id: Thing,
}
pub(crate) async fn associate_measure(
    measure_id: &str,
    control_id: &str,
    coverage: u8,
    db: &Surreal<Client>,
) -> surrealdb::Result<()> {
    let are_not_associated_already: Option<_Helper> = db
        .query("SELECT id FROM satisfies WHERE in=$mid AND out=$cid")
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
                id: Id::String(measure_id.to_string()),
                tb: "measure".to_string(),
            },
        ))
        .await?
        .take(0)?;
    match are_not_associated_already {
        Some(id) => Err(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest(
                format!("Control and measure are already associated in {id:?}").to_string(),
            ),
        )),
        None => {
            let response = db
                // .query("RELATE ((SELECT id FROM tag WHERE identifier = $tid)[0].id)->tags->((SELECT id FROM control WHERE identifier = $cid)[0].id)")
                .query("RELATE $mid->satisfies->$cid SET coverage = $coverage")
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
                        id: Id::String(measure_id.to_string()),
                        tb: "measure".to_string(),
                    },
                ))
                .bind(("coverage", coverage))
                .await?;
            response.check().map(|_| ())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// Dummy struct to make deserializing from the result in
/// get_measure_control_association easier
struct Helper {
    measure: Thing,
    control: Thing,
    coverage: u8,
}

pub(crate) async fn get_measure_control_association(
    db: &Surreal<Client>,
) -> surrealdb::Result<BTreeMap<String, Vec<(String, u8)>>> {
    let mut response = db
        .query("SELECT in as measure, out as control, coverage FROM satisfies")
        .await?;
    let results: Vec<Helper> = response.take(0)?;
    let mut result: BTreeMap<String, Vec<(String, u8)>> = BTreeMap::new();
    results.iter().for_each(|obj| {
        let key = match &obj.measure.id {
            Id::String(str) => str.into(),
            Id::Number(id) => id.to_string(),
            _ => panic!("A measure's id must be a string"),
        };
        let value = match &obj.control.id {
            Id::String(str) => (str.into(), obj.coverage),
            Id::Number(id) => (id.to_string(), obj.coverage),
            _ => panic!("A measure's id must be a string"),
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

/// Given a measure id, it returns a map that associates the control_ids
/// satisfied by this measure to their coverage
pub(crate) async fn get_coverage_for_measure(
    measure_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<HashMap<String, u8>> {
    let res: Vec<HelperR> = db
        .query("SELECT coverage, out as control FROM $measure_id->satisfies")
        .bind((
            "measure_id",
            Thing {
                id: Id::String(measure_id.to_string()),
                tb: "measure".to_string(),
            },
        ))
        .await?
        .take(0)?;
    Ok(HashMap::from_iter(res.iter().map(|r| {
        let id = match &r.control.id {
            Id::String(s) => s.clone(),
            Id::Number(id) => id.to_string(),
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

/// Given a measure id, it returns a vector with the ids of the tags of
/// all the controls this measure satisfies
pub(crate) async fn get_tags_for_measure(
    measure_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Tag>> {
    let mut res: Vec<Tag> = db
        // .query("SELECT id FROM $measures->satisfies.out<-tags.in")
        // .query("SELECT array::group(id) as ids FROM $measures->satisfies.out<-tags.in.id GROUP ALL")
        .query("SELECT * FROM $measures->satisfies.out<-tags.in.*")
        //You can also use a vec<Thing> and get the aggregate tags of more measures at once
        .bind((
            "measures",
            Thing {
                id: Id::String(measure_id.to_string()),
                tb: "measure".to_string(),
            },
        ))
        .await?
        .take(0)?;

    let mut s = HashSet::<String>::new();
    let mut de_duplicated: Vec<Tag> = Vec::new();
    res.iter_mut().for_each(|tag| {
        if !s.contains(&tag.identifier) {
            s.insert(tag.identifier.clone());
            de_duplicated.push(tag.to_owned());
        }
    });

    Ok(de_duplicated)
}

pub(crate) async fn update_measure(
    measure: Measure,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new measure with a random id
    let _created: Record = db
        .update(("measure", &measure.identifier))
        .content(measure)
        .await?;
    match _created.id.id {
        Id::String(id_str) => Ok(id_str),
        Id::Number(id) => Ok(id.to_string()),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Helper3 {
    progress: u8,
}

/// Returns the progress of all measures
pub(crate) async fn get_number_controls_batch(
    db: &Surreal<Client>,
    ids: Vec<String>,
) -> surrealdb::Result<Vec<u64>> {
    let ids_vec: Vec<Thing> = ids
        .iter()
        .map(|id| Thing {
            tb: "measure".to_string(),
            id: Id::String(id.to_string()),
        })
        .collect();
    let res: Vec<HelperR2> = db
        .query("SELECT id as ids FROM $mid->satisfies")
        .bind(("mid", ids_vec))
        .await?
        .take(0)?;

    let r2 = res.iter().map(|h| h.ids.len() as u64).collect();

    Ok(r2)
}
