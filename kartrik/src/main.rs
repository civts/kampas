#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
use endpoints::controls::{add_control, get_controls};
use helpers::{cors::CORS, surrealdb::get_client};

#[launch]
async fn rocket() -> _ {
    let client = get_client().await.unwrap();
    rocket::build()
        .mount("/", routes![add_control, get_controls])
        .attach(CORS)
        .manage(client)
}
