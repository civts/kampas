use super::Record;
use crate::models::token::AuthToken;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub(crate) async fn add_token(token: &AuthToken, db: &Surreal<Client>) -> surrealdb::Result<()> {
    let _deleted = db.delete(("token", &token.username)).await?;
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
