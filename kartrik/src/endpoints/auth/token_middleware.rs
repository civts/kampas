use crate::helpers::surrealdb::get_token;
use crate::models::token::AuthToken;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug)]
pub enum AuthTokenError {
    Missing,
    Invalid,
    InternalError,
}

impl AuthTokenError {
    pub(crate) fn to_http_code(&self) -> Status {
        match self {
            AuthTokenError::Missing => Status::Unauthorized,
            AuthTokenError::Invalid => Status::Unauthorized,
            AuthTokenError::InternalError => Status::InternalServerError,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = AuthTokenError;

    /// Extracts the AuthToken presen in the request (if any)
    /// This function does check that the token is still valid before returning it
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => {
                println!("No token supplied in the HTTP request");
                Outcome::Failure((
                    AuthTokenError::Missing.to_http_code(),
                    AuthTokenError::Missing,
                ))
            }
            Some(user_supplied_token) => {
                let db = req.guard::<&State<Surreal<Client>>>().await.unwrap();
                let token_res = get_token(user_supplied_token, db).await;
                match token_res {
                    Ok(Some(token)) => {
                        if token.is_expired() {
                            println!("The token is expired");
                            Outcome::Failure((
                                AuthTokenError::Invalid.to_http_code(),
                                AuthTokenError::Invalid,
                            ))
                        } else {
                            println!("A valid token is present in the request");
                            Outcome::Success(token)
                        }
                    }
                    Ok(None) => {
                        println!("Could not find that token in the DB");
                        Outcome::Failure((
                            AuthTokenError::Invalid.to_http_code(),
                            AuthTokenError::Invalid,
                        ))
                    }
                    Err(err) => {
                        println!("Could not query the DB: {err:?}");
                        Outcome::Failure((
                            AuthTokenError::InternalError.to_http_code(),
                            AuthTokenError::InternalError,
                        ))
                    }
                }
            }
        }
    }
}
