#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
mod models;
use endpoints::{
    auth,
    controls::{
        controls::{add_control, get_controls},
        upload::upload,
    },
};
use helpers::{cors::CORS, surrealdb::get_client};

#[get("/")]
pub(crate) fn working() -> &'static str {
    "Yes, we are up"
}

#[launch]
async fn rocket() -> _ {
    let client = get_client().await.unwrap();
    rocket::build()
        .mount(
            "/api/v1/controls",
            routes![add_control, get_controls, upload],
        )
        .mount(auth::BASE, auth::routes())
        .mount("/", routes![working])
        .attach(CORS)
        .manage(client)
}
