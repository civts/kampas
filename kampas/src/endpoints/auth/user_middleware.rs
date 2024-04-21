use super::token_middleware::AuthTokenError;
use crate::helpers::surrealdb::user::get_user;
use crate::models::token::AuthToken;
use crate::models::user::User;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthTokenError;

    /// Returns the user-if a- to which this request is relative to, using the AuthToken
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token_from_req = req.guard::<AuthToken>().await;
        if token_from_req.is_success() {
            let token = token_from_req.unwrap();
            let db = req.guard::<&State<Surreal<Client>>>().await.unwrap();

            let user_res = get_user(&token.username, db).await;
            match user_res {
                Ok(Some(user)) => Outcome::Success(user),
                Ok(None) => {
                    println!("We have a token relative to an unknown user, TODO: delete the token");
                    Outcome::Error((
                        AuthTokenError::Invalid.to_http_code(),
                        AuthTokenError::Invalid,
                    ))
                }
                Err(err) => {
                    println!("Had a problem retrieving user from database: {err:?}");
                    Outcome::Error((
                        AuthTokenError::InternalError.to_http_code(),
                        AuthTokenError::InternalError,
                    ))
                }
            }
        } else {
            let failure = token_from_req.failed().expect("Should be a failure");
            Outcome::Error(failure)
        }
    }
}
