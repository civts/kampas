use crate::{
    endpoints::tags::tags::GetTagsRole,
    generate_endpoint_roles,
    helpers::surrealdb::metric::{
        add_metric as add_metricl, associate_metric as associate_metricl,
        get_coverage_for_metric as get_coverage_for_metricc, get_metric as get_metricl,
        get_metrics as get_metricss, get_metrics_for_control as get_metrics_for_controll,
        get_tags_for_metric as get_tags_for_metricc,
    },
    models::{metric::Metric, role::Role, user::User},
};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct MetricFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
    pub(crate) effort: Option<u8>,
}

generate_endpoint_roles!(EditMetricRoles, { Role::CreateMetrics });
#[post("/", data = "<form_data>")]
pub(crate) async fn add_metric(
    form_data: Form<MetricFormData>,
    _r: EditMetricRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let metric = Metric::new(&form_data.title, &form_data.description, form_data.effort);
    match add_metricl(metric.clone(), db).await {
        Ok(id) => status::Custom(Status::Ok, format!("{id}").to_string()),
        Err(err) => {
            println!("Could not create new metric: {err:?}");
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct AssociateMetricFormData {
    pub(crate) metric_id: String,
    pub(crate) control_id: String,
    #[field(validate = range(1..101), default = 100)]
    pub(crate) coverage: u8,
}
generate_endpoint_roles!(AssociateMetricRoles, { Role::AssociateMetrics });
#[post("/associate", data = "<form_data>")]
pub(crate) async fn associate_metric(
    form_data: Form<AssociateMetricFormData>,
    _r: AssociateMetricRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let associate_res = associate_metricl(
        &form_data.metric_id,
        &form_data.control_id,
        form_data.coverage,
        db,
    )
    .await;
    match associate_res {
        Ok(_) => status::Custom(Status::Ok, format!("Ok, metric associated").to_string()),
        Err(err) => {
            println!("Could not associate metric: {err:?}");
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

generate_endpoint_roles!(GetMetricsRole, { Role::GetMetrics });
#[get("/")]
pub(crate) async fn get_metrics(
    user: User,
    _required_roles: GetMetricsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let metrics_res = get_metricss(db).await;
    println!("{} is requesting the metrics", user.username);
    match metrics_res {
        Ok(metrics) => status::Custom(
            Status::Ok,
            serde_json::to_string(&metrics).expect("can serialize the metrics to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the metrics: {}", err);
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[get("/?<control_id>")]
pub(crate) async fn get_metrics_for_control(
    user: User,
    control_id: &str,
    _required_roles: GetMetricsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let metrics_res = get_metrics_for_controll(control_id, db).await;
    println!(
        "{} is requesting the metrics for control {control_id}",
        user.username
    );
    match metrics_res {
        Ok(metrics) => status::Custom(
            Status::Ok,
            serde_json::to_string(&metrics).expect("can serialize the metrics to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the metrics: {}", err);
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[get("/<metric_id>")]
pub(crate) async fn get_metric(
    user: User,
    metric_id: String,
    _required_roles: GetMetricsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!("{} is requesting the metric {metric_id}", user.username);
    let metrics_res = get_metricl(db, metric_id).await;
    match metrics_res {
        Ok(metric) => status::Custom(
            Status::Ok,
            serde_json::to_string(&metric).expect("can serialize the metric to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the metrics: {}", err);
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[get("/coverage_for_metric?<metric_id>")]
pub(crate) async fn get_coverage_for_metric(
    user: User,
    metric_id: String,
    _required_roles: GetMetricsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the coverage info for metric {metric_id}",
        user.username
    );
    let metrics_res = get_coverage_for_metricc(&metric_id, db).await;
    match metrics_res {
        Ok(metric) => {
            let a = serde_json::to_string(&metric)
                .expect("can serialize the metric coverage info to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!(
                "Something went wrong getting the metric coverage info: {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[get("/tags_for_metric?<metric_id>")]
pub(crate) async fn get_tags_for_metric(
    user: User,
    metric_id: String,
    _required_roles: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the tags for metric {metric_id}",
        user.username
    );
    let metrics_res = get_tags_for_metricc(&metric_id, db).await;
    match metrics_res {
        Ok(tags) => {
            let a = serde_json::to_string(&tags).expect("can serialize the metric tags to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!("Something went wrong getting the metric tags: {}", err);
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}
