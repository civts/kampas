use super::Record;
use crate::models::enabler::Enabler;
use crate::models::tag::Tag;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

pub(crate) async fn add_enabler(
    enabler: Enabler,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new enabler with a random id
    let _created: Record = db
        .create(("enabler", &enabler.identifier))
        .content(enabler)
        .await?;
    match _created.id.id {
        Id::String(id_str) => Ok(id_str),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    }
}

pub(crate) async fn get_enablers(db: &Surreal<Client>) -> surrealdb::Result<Vec<Enabler>> {
    // Select all records
    let enablers: Vec<Enabler> = db.select("enabler").await?;

    Ok(enablers)
}

pub(crate) async fn get_enablers_for_control(
    control_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Enabler>> {
    let enablers: Vec<Enabler> = db
        .query("SELECT * FROM enabler WHERE id INSIDE $control_id<-satisfies.in")
        .bind((
            "control_id",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(enablers)
}

pub(crate) async fn get_enabler(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Enabler>> {
    let enabler: Option<Enabler> = db.select(("enabler", id)).await?;

    Ok(enabler)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper {
    id: Thing,
}
pub(crate) async fn associate_enabler(
    enabler_id: &str,
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
                id: Id::String(enabler_id.to_string()),
                tb: "enabler".to_string(),
            },
        ))
        .await?
        .take(0)?;
    match are_not_associated_already {
        Some(id) => Err(surrealdb::Error::Api(
            surrealdb::error::Api::InvalidRequest(
                format!("Control and enabler are already associated in {id:?}").to_string(),
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
                        id: Id::String(enabler_id.to_string()),
                        tb: "enabler".to_string(),
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
/// get_enabler_control_association easier
struct Helper {
    enabler: Thing,
    control: Thing,
    coverage: u8,
}

pub(crate) async fn get_enabler_control_association(
    db: &Surreal<Client>,
) -> surrealdb::Result<BTreeMap<String, Vec<(String, u8)>>> {
    let mut response = db
        .query("SELECT in as enabler, out as control, coverage FROM satisfies")
        .await?;
    let results: Vec<Helper> = response.take(0)?;
    let mut result: BTreeMap<String, Vec<(String, u8)>> = BTreeMap::new();
    results.iter().for_each(|obj| {
        let key = match &obj.enabler.id {
            Id::String(str) => str.into(),
            _ => panic!("A enabler's id must be a string"),
        };
        let value = match &obj.control.id {
            Id::String(str) => (str.into(), obj.coverage),
            _ => panic!("A enabler's id must be a string"),
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

/// Given a enabler id, it returns a map that associates the control_ids
/// satisfied by this enabler to their coverage
pub(crate) async fn get_coverage_for_enabler(
    enabler_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<HashMap<String, u8>> {
    let res: Vec<HelperR> = db
        .query("SELECT coverage, out as control FROM $enabler_id->satisfies")
        .bind((
            "enabler_id",
            Thing {
                id: Id::String(enabler_id.to_string()),
                tb: "enabler".to_string(),
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

/// Given a enabler id, it returns a vector with the ids of the tags of
/// all the controls this enabler satisfies
pub(crate) async fn get_tags_for_enabler(
    enabler_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Tag>> {
    let mut res: Vec<Tag> = db
        // .query("SELECT id FROM $enablers->satisfies.out<-tags.in")
        // .query("SELECT array::group(id) as ids FROM $enablers->satisfies.out<-tags.in.id GROUP ALL")
        .query("SELECT * FROM $enablers->satisfies.out<-tags.in.*")
        //You can also use a vec<Thing> and get the aggregate tags of more enablers at once
        .bind((
            "enablers",
            Thing {
                id: Id::String(enabler_id.to_string()),
                tb: "enabler".to_string(),
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

pub(crate) async fn update_enabler(
    enabler: Enabler,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new enabler with a random id
    let _created: Record = db
        .update(("enabler", &enabler.identifier))
        .content(enabler)
        .await?;
    match _created.id.id {
        Id::String(id_str) => Ok(id_str),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Helper3 {
    progress: u8,
}

/// Returns the progress of all enablers
pub(crate) async fn get_number_controls_batch(
    db: &Surreal<Client>,
    ids: Vec<String>,
) -> surrealdb::Result<Vec<u64>> {
    let ids_vec: Vec<Thing> = ids
        .iter()
        .map(|id| Thing {
            tb: "enabler".to_string(),
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
