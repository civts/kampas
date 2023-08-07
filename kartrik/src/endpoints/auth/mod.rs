use rocket::form::Form;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::{Flash, Redirect};

use crate::models::user::User;

/// Remove the `user_id` cookie.
#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[derive(FromForm)]
pub(crate) struct LoginFormData {
    user_id: String,
}

#[post("/login", data = "<form_data>")]
pub(crate) fn login(cookies: &CookieJar<'_>, form_data: Form<LoginFormData>) -> Flash<Redirect> {
    let value = form_data.user_id.to_owned();
    cookies.add_private(Cookie::new("user_id", value));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

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
        // match req.headers().get_one("x-api-key") {
        //     None => Outcome::Failure((
        //         AuthCookieError::Missing.to_http_code(),
        //         AuthCookieError::Missing,
        //     )),
        //     Some(key) if is_valid(key) => Outcome::Success(User {}),
        //     Some(_) => Outcome::Failure((
        //         AuthCookieError::Invalid.to_http_code(),
        //         AuthCookieError::Invalid,
        //     )),
        // }
        let cookies = req.cookies();
        if let Some(cookie) = cookies.get_private("user_id") {
            // Auth cookie is set, allow the request to pass
            Outcome::Success(User {
                username: cookie.value().to_string(),
            })
        } else {
            // Auth cookie is not set, return 403 Forbidden
            Outcome::Failure((
                AuthCookieError::Missing.to_http_code(),
                AuthCookieError::Missing,
            ))
        }
    }
}
