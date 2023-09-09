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
            get_controls, get_controls_for_metric,
        },
        upload::upload,
    },
    metrics::metrics::{
        add_metric, associate_metric, get_coverage_for_metric, get_metric, get_metrics,
        get_metrics_for_control, get_metrics_progess, get_number_controls_batch,
        get_tags_for_metric, update_metric,
    },
    rankings::ranking::{get_ranking, get_rankings, new_ranking},
    tags::{
        tag_control::{add_tag_to_control, remove_tag_from_control},
        tags::{add_tag, get_tags, get_tags_batch, get_tags_for_control},
    },
};
use helpers::{cors::CORS, surrealdb::connection::get_client};

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
            routes![
                add_control,
                get_controls,
                upload,
                get_control,
                get_controls_for_metric,
                get_control_completion,
                get_control_completion_b
            ],
        )
        .mount(
            "/api/v1/tags",
            routes![
                add_tag,
                get_tags,
                get_tags_batch,
                get_tags_for_control,
                add_tag_to_control,
                remove_tag_from_control
            ],
        )
        .mount(
            "/api/v1/metrics",
            routes![
                add_metric,
                update_metric,
                get_metrics,
                get_metric,
                get_metrics_for_control,
                get_coverage_for_metric,
                associate_metric,
                get_tags_for_metric,
                get_metrics_progess,
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
