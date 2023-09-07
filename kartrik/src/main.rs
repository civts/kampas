#[macro_use]
extern crate rocket;
mod endpoints;
mod helpers;
mod models;
use endpoints::{
    auth,
    controls::{
        controls::{add_control, get_control, get_controls, get_controls_for_metric},
        upload::upload,
    },
    metrics::metrics::{
        add_metric, associate_metric, get_coverage_for_metric, get_metric, get_metrics,
        get_metrics_for_control, get_tags_for_metric,
    },
    rankings::ranking::{get_ranking, get_rankings, new_ranking},
    tags::{
        tag_control::add_tag_to_control,
        tags::{add_tag, get_tags, get_tags_for_control},
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
                get_controls_for_metric
            ],
        )
        .mount(
            "/api/v1/tags",
            routes![add_tag, get_tags, get_tags_for_control, add_tag_to_control],
        )
        .mount(
            "/api/v1/metrics",
            routes![
                add_metric,
                get_metrics,
                get_metric,
                get_metrics_for_control,
                get_coverage_for_metric,
                associate_metric,
                get_tags_for_metric
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
