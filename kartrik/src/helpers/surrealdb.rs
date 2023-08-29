use crate::models::control::Control;
use crate::models::tag::Tag;
use crate::models::token::AuthToken;
use crate::models::user::User;
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Id, Thing};
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
    // Should be SELECT * FROM tag WHERE id INSIDE $control_id<-tags.in; with control_id like control:123, but it fails that way
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
