use crate::models::user::User;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use super::Record;

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
