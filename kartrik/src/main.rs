#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
use endpoints::controls::{
    controls::{add_control, get_controls},
    upload::upload,
};
use helpers::{cors::CORS, surrealdb::get_client};

#[launch]
async fn rocket() -> _ {
    let client = get_client().await.unwrap();
    rocket::build()
        .mount("/", routes![add_control, get_controls, upload])
        .attach(CORS)
        .manage(client)
}
