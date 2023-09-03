use crate::models::control::Control;
use crate::models::metric::Metric;
use crate::models::ranking::Ranking;
use crate::models::tag::Tag;
use crate::models::token::AuthToken;
use crate::models::user::User;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Id, Object, Thing, Value};
use surrealdb::Surreal;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/// Returns a new surrealdb client to which we can talk
pub(crate) async fn get_client() -> surrealdb::Result<Surreal<Client>> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8002").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    Ok(db)
}

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

pub(crate) async fn get_control(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Control>> {
    let control: Option<Control> = db.select(("control", id)).await?;

    Ok(control)
}

pub(crate) async fn add_user(user: User, db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Create a new user with a random id
    let _created: Record = db
        .create(("user", user.username.clone()))
        .content(user)
        .await?;
    Ok(())
}

pub(crate) async fn does_user_exist(
    username: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<bool> {
    // Select all records
    let user: Option<User> = db.select(("user", username)).await?;
    Ok(user.is_some())
}

pub(crate) async fn get_user(
    username: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Option<User>> {
    let user: Option<User> = db.select(("user", username)).await?;
    Ok(user)
}

pub(crate) async fn add_token(token: &AuthToken, db: &Surreal<Client>) -> surrealdb::Result<()> {
    let _created: Record = db
        .create(("token", &token.username))
        .content(&token)
        .await?;
    Ok(())
}

pub(crate) async fn get_token_for_user(
    user: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Option<AuthToken>> {
    let token: Option<AuthToken> = db.select(("token", user)).await?;

    Ok(token)
}

pub(crate) async fn get_token(
    token: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Option<AuthToken>> {
    let mut token_opt = db
        .query("SELECT * FROM token WHERE token = $tk")
        .bind(("tk", token))
        .await?;

    let token_res: Result<Option<AuthToken>, surrealdb::Error> = token_opt.take(0);
    token_res
}

pub(crate) async fn delete_token(token: &AuthToken, db: &Surreal<Client>) -> surrealdb::Result<()> {
    db.query("DELETE * FROM token WHERE token = $tk")
        .bind(("tk", &token.token))
        .await?;
    Ok(())
}

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

pub(crate) async fn add_ranking(
    ranking: Ranking,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new ranking with a random id
    let _created: Record = db
        .create(("ranking", &ranking.identifier))
        .content(ranking)
        .await?;
    match _created.id.id {
        Id::String(id_str) => Ok(id_str),
        _ => panic!("We don't do that here. We shall only use String IDs"),
    }
}

pub(crate) async fn get_rankings(db: &Surreal<Client>) -> surrealdb::Result<Vec<Ranking>> {
    // Select all records
    let rankings: Vec<Ranking> = db.select("ranking").await?;

    Ok(rankings)
}

pub(crate) async fn get_ranking(
    id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Option<Ranking>> {
    // Select all records
    let rankings: Option<Ranking> = db.select(("ranking", id)).await?;

    Ok(rankings)
}

// pub(crate) async fn update_control(db: Surreal<Client>) -> surrealdb::Result<Vec<Record>> {
//     // Update a control record with a specific id
//     let updated: Record = db
//         .update(("control", "jaime"))
//         .merge(Responsibility { marketing: true })
//         .await?;
//     dbg!(updated);
// }

//     // Perform a custom advanced query
//     let groups = db
//         .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
//         .bind(("table", "control"))
//         .await?;
//     dbg!(groups);

//     Ok(())
