use crate::helpers::surrealdb::get_user;
use crate::models::token::Token;
use crate::models::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug)]
pub enum AuthCookieError {
    Missing,
    Invalid,
    // Unauthorized,
}

impl AuthCookieError {
    fn to_http_code(&self) -> Status {
        match self {
            AuthCookieError::Missing => Status::Unauthorized,
            AuthCookieError::Invalid => Status::Unauthorized,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = AuthCookieError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((
                AuthCookieError::Missing.to_http_code(),
                AuthCookieError::Missing,
            )),
            Some(user_supplied_token) => {
                let db = req.guard::<&State<Surreal<Client>>>().await.unwrap();

                let mut token_opt = db
                    .query("SELECT * FROM token WHERE token = $tk")
                    .bind(("tk", user_supplied_token))
                    .await
                    .expect("Can perform a select");

                let token_res: Result<Option<Token>, surrealdb::Error> = token_opt.take(0);

                match token_res {
                    Ok(Some(token)) => {
                        if token.is_expired() {
                            println!("The token is expired");
                            Outcome::Failure((
                                AuthCookieError::Invalid.to_http_code(),
                                AuthCookieError::Invalid,
                            ))
                        } else {
                            let user_res = get_user(&token.username, db).await;
                            if let Ok(Some(user)) = user_res {
                                Outcome::Success(user)
                            } else {
                                println!("Could not retrieve user");
                                Outcome::Failure((
                                    AuthCookieError::Invalid.to_http_code(),
                                    AuthCookieError::Invalid,
                                ))
                            }
                        }
                    }
                    Ok(None) => {
                        println!("Could not find that token in the DB");
                        Outcome::Failure((
                            AuthCookieError::Invalid.to_http_code(),
                            AuthCookieError::Invalid,
                        ))
                    }
                    Err(err) => {
                        println!("Could not query the DB: {err:?}");
                        Outcome::Failure((
                            AuthCookieError::Invalid.to_http_code(),
                            AuthCookieError::Invalid,
                        ))
                    }
                }
            }
        }
    }
}
