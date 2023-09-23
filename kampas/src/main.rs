#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
mod models;
use endpoints::{
    auth,
    controls::{
        controls::{
            add_control, get_control, get_control_completion, get_control_completion_b,
            get_controls, get_controls_for_measure, get_measure_control_association_batch,
            get_measures_for_control_count_batch, update_control,
        },
        upload::upload,
    },
    measures::measures::{
        add_measure, associate_measure, get_coverage_for_measure, get_measure, get_measures,
        get_measures_for_control, get_number_controls_batch, update_measure,
    },
    rankings::ranking::{get_ranking, get_rankings, new_ranking},
    tags::{
        tag_control::{add_tag_to_control, remove_tag_from_control},
        tags::{add_tag, get_measure_tag_ids_batch, get_tags_batch, get_tags_for_control},
    },
};
use helpers::{cors::CORS, surrealdb::connection::get_client};

#[get("/")]
pub(crate) fn working() -> &'static str {
    "Yes, kampas is up"
}

#[launch]
async fn rocket() -> _ {
    let client = get_client()
        .await
        .unwrap_or_else(|err| panic!("Error in the databse connection: {err:?}"));
    rocket::build()
        .mount(
            "/api/v1/controls",
            routes![
                add_control,
                get_controls,
                upload,
                get_control,
                get_controls_for_measure,
                get_control_completion,
                get_control_completion_b,
                get_measures_for_control_count_batch,
                get_measure_control_association_batch,
                update_control
            ],
        )
        .mount(
            "/api/v1/tags",
            routes![
                add_tag,
                get_tags_batch,
                get_tags_for_control,
                add_tag_to_control,
                remove_tag_from_control,
                get_measure_tag_ids_batch
            ],
        )
        .mount(
            "/api/v1/measures",
            routes![
                add_measure,
                update_measure,
                get_measures,
                get_measure,
                get_measures_for_control,
                get_coverage_for_measure,
                associate_measure,
                get_number_controls_batch
            ],
        )
        .mount(
            "/api/v1/rankings",
            routes![new_ranking, get_rankings, get_ranking],
        )
        .mount(auth::BASE, auth::routes())
        .mount("/", routes![working])
        .attach(CORS)
        .manage(client)
}
