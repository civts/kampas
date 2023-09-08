use super::Record;
use crate::models::tag::Tag;
use ::serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};

pub(crate) async fn add_tag(tag: Tag, db: &Surreal<Client>) -> surrealdb::Result<String> {
    let record: Record = db.create(("tag", &tag.identifier)).content(tag).await?;
    Ok(record.id.to_string())
}

pub(crate) async fn get_tags(db: &Surreal<Client>) -> surrealdb::Result<Vec<Tag>> {
    let tags: Vec<Tag> = db.select("tag").await?;

    Ok(tags)
}

pub(crate) async fn get_tags_for_control(
    control_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Tag>> {
    let tags: Vec<Tag> = db
        // .query("SELECT * FROM tag WHERE id INSIDE ((SELECT id FROM control WHERE identifier = $control_id)[0].id)<-tags.in")
        .query("SELECT * FROM tag WHERE id INSIDE $control_id<-tags.in")
        .bind((
            "control_id",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(tags)
}

pub(crate) async fn tag_control(
    control_id: &str,
    tag_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<()> {
    let response = db
        // .query("RELATE ((SELECT id FROM tag WHERE identifier = $tid)[0].id)->tags->((SELECT id FROM control WHERE identifier = $cid)[0].id)")
        .query("RELATE $tid->tags->$cid")
        .bind((
            "cid",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .bind((
            "tid",
            Thing {
                id: Id::String(tag_id.to_string()),
                tb: "tag".to_string(),
            },
        ))
        .await?;
    response.check().map(|_| ())
}

pub(crate) async fn untag_control(
    control_id: &str,
    tag_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<()> {
    let response = db
        .query("DELETE FROM tags WHERE in=$tid AND out=$cid")
        .bind((
            "tid",
            Thing {
                id: Id::String(tag_id.to_string()),
                tb: "tag".to_string(),
            },
        ))
        .bind((
            "cid",
            Thing {
                id: Id::String(control_id.to_string()),
                tb: "control".to_string(),
            },
        ))
        .await?;
    response.check().map(|_| ())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct _Helper {
    tag: Thing,
    control: Thing,
}

/// Returns a Map in which each tag identifier has associated a Set of control identifiers
pub(crate) async fn get_tag_control_association(
    db: &Surreal<Client>,
) -> surrealdb::Result<HashMap<String, HashSet<String>>> {
    let qres: Vec<_Helper> = db
        .query("SELECT in as tag, out as control FROM tags")
        .await?
        .take(0)?;
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();

    qres.iter().for_each(|r| {
        let tag_id = match &r.tag.id {
            Id::String(id) => id,
            _ => panic!("Tag id was not a String"),
        };
        let control_id = match &r.control.id {
            Id::String(id) => id,
            _ => panic!("Control id was not a String"),
        };
        match res.get_mut(tag_id) {
            None => {
                res.insert(
                    (*tag_id).clone(),
                    HashSet::from_iter(vec![(*control_id).clone()]),
                );
            }
            Some(set) => {
                set.insert((*control_id).clone());
            }
        };
    });

    Ok(res)
}
