#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
mod models;
use endpoints::controls::{
    controls::{add_control, get_controls},
    upload::upload,
};
use helpers::{cors::CORS, surrealdb::get_client};

#[launch]
async fn rocket() -> _ {
    let client = get_client().await.unwrap();
    rocket::build()
        .mount(
            "/api/v1/controls",
            routes![add_control, get_controls, upload],
        )
        .attach(CORS)
        .manage(client)
}
