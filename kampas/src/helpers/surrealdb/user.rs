use crate::models::user::User;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use super::Record;

pub(crate) async fn add_user(user: User, db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Create a new user with a random id
    let created_opt: Option<Record> = db
        .create(("user", user.username.clone()))
        .content(&user)
        .await?;
    match created_opt {
        Some(_) => Ok(()),
        None => Err(surrealdb::Error::Api(surrealdb::error::Api::InternalError(
            format!("User named {} was not created", &user.username).to_string(),
        ))),
    }
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
