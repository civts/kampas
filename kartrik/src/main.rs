#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
mod models;
use endpoints::{
    auth,
    controls::{
        controls::{add_control, get_control, get_controls},
        upload::upload,
    },
    tags::{
        tag_control::add_tag_to_control,
        tags::{add_tag, get_tags, get_tags_for_control},
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
            routes![add_control, get_controls, upload, get_control],
        )
        .mount(
            "/api/v1/tags",
            routes![add_tag, get_tags, get_tags_for_control, add_tag_to_control],
        )
        .mount(auth::BASE, auth::routes())
        .mount("/", routes![working])
        .attach(CORS)
        .manage(client)
}
