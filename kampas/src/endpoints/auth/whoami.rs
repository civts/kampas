use crate::models::user::User;
use rocket::{http::Status, response::status};

#[get("/whoami")]
pub(crate) async fn whoami(useropt: Option<User>) -> status::Custom<String> {
    match useropt {
        Some(user) => status::Custom(Status::Ok, format!("{}", user.username)),
        None => status::Custom(Status::Unauthorized, "You are not authorized".to_string()),
    }
}
