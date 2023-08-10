use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::models::control::Control;
use crate::models::token::AuthToken;
use crate::models::user::User;

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
    let _created: Record = db.create("control").content(control).await?;
    Ok(())
}

pub(crate) async fn get_controls(db: &Surreal<Client>) -> surrealdb::Result<Vec<Control>> {
    // Select all records
    let controls: Vec<Control> = db.select("control").await?;

    Ok(controls)
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
