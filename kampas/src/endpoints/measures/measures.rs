use crate::{
    endpoints::tags::tags::GetTagsRole,
    generate_endpoint_roles,
    helpers::surrealdb::measure::{
        add_measure as add_measurel, associate_measure as associate_measurel,
        get_coverage_for_measure as get_coverage_for_measurec, get_measure as get_measurel,
        get_measures as get_measuress, get_measures_for_control as get_measures_for_controll,
        get_number_controls_batch as get_number_controls_batchh,
        get_tags_for_measure as get_tags_for_measurec, update_measure as update_measurec,
    },
    models::{measure::Measure, role::Role, user::User},
};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct MeasureFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
    pub(crate) effort: Option<u8>,
}

generate_endpoint_roles!(EditMeasureRoles, { Role::CreateMeasures });
#[post("/", data = "<form_data>")]
pub(crate) async fn add_measure(
    form_data: Form<MeasureFormData>,
    _r: EditMeasureRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let measure = Measure::new(&form_data.title, &form_data.description, form_data.effort);
    match add_measurel(measure.clone(), db).await {
        Ok(id) => status::Custom(Status::Ok, format!("{id}").to_string()),
        Err(err) => {
            println!("Could not create new measure: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct AssociateMeasureFormData {
    pub(crate) measure_id: String,
    pub(crate) control_id: String,
    #[field(validate = range(1..101), default = 100)]
    pub(crate) coverage: u8,
}
generate_endpoint_roles!(AssociateMeasureRoles, { Role::AssociateMeasures });
#[post("/associate", data = "<form_data>")]
pub(crate) async fn associate_measure(
    form_data: Form<AssociateMeasureFormData>,
    _r: AssociateMeasureRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let associate_res = associate_measurel(
        &form_data.measure_id,
        &form_data.control_id,
        form_data.coverage,
        db,
    )
    .await;
    match associate_res {
        Ok(_) => status::Custom(Status::Ok, format!("Ok, measure associated").to_string()),
        Err(err) => {
            println!("Could not associate measure: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

generate_endpoint_roles!(GetMeasuresRole, { Role::GetMeasures });
#[get("/")]
pub(crate) async fn get_measures(
    user: User,
    _required_roles: GetMeasuresRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let measures_res = get_measuress(db).await;
    println!("{} is requesting the measures", user.username);
    match measures_res {
        Ok(measures) => status::Custom(
            Status::Ok,
            serde_json::to_string(&measures).expect("can serialize the measures to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the measures: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/?<control_id>")]
pub(crate) async fn get_measures_for_control(
    user: User,
    control_id: &str,
    _required_roles: GetMeasuresRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let measures_res = get_measures_for_controll(control_id, db).await;
    println!(
        "{} is requesting the measures for control {control_id}",
        user.username
    );
    match measures_res {
        Ok(measures) => status::Custom(
            Status::Ok,
            serde_json::to_string(&measures).expect("can serialize the measures to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the measures: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/<measure_id>")]
pub(crate) async fn get_measure(
    user: User,
    measure_id: String,
    _required_roles: GetMeasuresRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!("{} is requesting the measure {measure_id}", user.username);
    let measures_res = get_measurel(db, measure_id).await;
    match measures_res {
        Ok(measure) => status::Custom(
            Status::Ok,
            serde_json::to_string(&measure).expect("can serialize the measure to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the measures: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/coverage_for_measure?<measure_id>")]
pub(crate) async fn get_coverage_for_measure(
    user: User,
    measure_id: String,
    _required_roles: GetMeasuresRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the coverage info for measure {measure_id}",
        user.username
    );
    let measures_res = get_coverage_for_measurec(&measure_id, db).await;
    match measures_res {
        Ok(measure) => {
            let a = serde_json::to_string(&measure)
                .expect("can serialize the measure coverage info to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!(
                "Something went wrong getting the measure coverage info: {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/tags_for_measure?<measure_id>")]
pub(crate) async fn get_tags_for_measure(
    user: User,
    measure_id: String,
    _required_roles: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the tags for measure {measure_id}",
        user.username
    );
    let measures_res = get_tags_for_measurec(&measure_id, db).await;
    match measures_res {
        Ok(tags) => {
            let a = serde_json::to_string(&tags).expect("can serialize the measure tags to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!("Something went wrong getting the measure tags: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UpdateMeasureFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: Option<String>,
    #[field(validate = len(3..300))]
    pub(crate) description: Option<String>,
    pub(crate) effort: Option<u8>,
    pub(crate) progress: Option<u8>,
}

#[patch("/<measure_id>", data = "<form_data>")]
pub(crate) async fn update_measure(
    mut form_data: Form<UpdateMeasureFormData>,
    measure_id: String,
    _r: EditMeasureRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let measure_res = get_measurel(db, measure_id.clone()).await;
    match measure_res {
        Ok(measure_opt) => match measure_opt {
            Some(mut measure) => {
                if let Some(d) = form_data.description.take() {
                    measure.description = d;
                }
                if let Some(t) = form_data.title.take() {
                    measure.title = t;
                }
                if let Some(e) = form_data.effort.take() {
                    if e > 0 {
                        measure.effort = e;
                    }
                }
                if let Some(p) = form_data.progress.take() {
                    if p > 0 && p <= 100 {
                        measure.progress = p;
                    }
                }
                let update_res = update_measurec(measure, db).await;
                match update_res {
                    Ok(_) => status::Custom(Status::Ok, format!("{measure_id}").to_string()),
                    Err(err) => {
                        println!("Could not update the measure {measure_id}: {err:?}");
                        status::Custom(
                            Status::InternalServerError,
                            Status::InternalServerError.reason_lossy().to_string(),
                        )
                    }
                }
            }
            None => status::Custom(
                Status::NotFound,
                Status::NotFound.reason_lossy().to_string(),
            ),
        },
        Err(err) => {
            println!("Could not get the measure to update: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[post("/get_number_controls_batch", data = "<req_data>")]
pub(crate) async fn get_number_controls_batch(
    user: User,
    _required_roles: GetMeasuresRole,
    req_data: String,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let ids_res = serde_json::from_str::<Vec<String>>(&req_data);
    match ids_res {
        Ok(ids) => {
            let progress_res = get_number_controls_batchh(db, ids).await;
            println!(
                "{} is requesting the controls associated to measures (batch)",
                user.username
            );
            match progress_res {
                Ok(measures) => status::Custom(
                    Status::Ok,
                    serde_json::to_string(&measures).expect(
                        "can serialize the controls associated to measures (batch) to JSON",
                    ),
                ),
                Err(err) => {
                    println!(
                "Something went wrong getting the controls associated to measures (batch): {}",
                err
            );
                    status::Custom(
                        Status::InternalServerError,
                        Status::InternalServerError.reason_lossy().to_string(),
                    )
                }
            }
        }
        Err(err) => {
            println!("Something went wrong reading the ids: {}", err);
            status::Custom(
                Status::BadRequest,
                Status::BadRequest.reason_lossy().to_string(),
            )
        }
    }
}
