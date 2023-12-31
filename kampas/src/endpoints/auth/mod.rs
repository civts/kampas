use crate::endpoints::auth::token::generate_token;
use rocket::Route;
pub(crate) mod expire;
pub(crate) mod register;
pub(crate) mod token;
pub(crate) mod token_middleware;
pub(crate) mod user_middleware;
pub(crate) mod whoami;

pub(crate) const BASE: &'static str = "/api/v1/auth";

pub(crate) fn routes() -> Vec<Route> {
    routes![
        register::register,
        generate_token,
        expire::expire,
        whoami::whoami
    ]
}
