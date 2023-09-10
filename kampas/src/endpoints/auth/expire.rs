use crate::{helpers::surrealdb::token::delete_token, models::token::AuthToken};
use rocket::{http::Status, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

/// Remove the provided token.
#[get("/expire")]
pub(crate) async fn expire(token: AuthToken, db: &State<Surreal<Client>>) -> Status {
    let deletion_result = delete_token(&token, db).await;
    match deletion_result {
        Ok(_) => Status::Ok,
        Err(err) => {
            println!("Could not delete the token {token:?}: {err:?}");
            let status = Status::InternalServerError;
            status
        }
    }
}
