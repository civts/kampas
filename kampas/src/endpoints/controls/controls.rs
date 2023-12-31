use crate::{
    generate_endpoint_roles,
    helpers::surrealdb::{
        control::{
            add_control as add_controll, get_control as get_controll,
            get_control_completion as get_control_completionn, get_control_completion_batch,
            get_controls as get_controlss, get_controls_for_measure as get_controls_for_measurec,
            get_measures_for_control_count_batch as get_measures_for_control_count_batchh,
            update_control as update_controlc,
        },
        measure::get_measure_control_association,
    },
    models::{control::Control, role::Role, user::User},
};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ControlFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
}

generate_endpoint_roles!(EditControlRoles, { Role::EditControls });
#[post("/", data = "<form_data>")]
pub(crate) async fn add_control(
    form_data: Form<ControlFormData>,
    _r: EditControlRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let control_form_data = form_data.into_inner();
    let control = Control::new(&control_form_data.title, &control_form_data.description);
    match add_controll(control.clone(), db).await {
        Ok(_) => status::Custom(Status::Ok, format!("Added {:?}!", &control).to_string()),
        Err(err) => {
            println!("Could not insert the new control in the database: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

generate_endpoint_roles!(GetControlsRole, { Role::GetControls });
#[get("/")]
pub(crate) async fn get_controls(
    user: User,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let controls_res = get_controlss(db).await;
    println!("{} is requesting the controls", user.username);
    match controls_res {
        Ok(controls) => status::Custom(
            Status::Ok,
            serde_json::to_string(&controls).expect("can serialize the controls to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the controls: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/<control_id>")]
pub(crate) async fn get_control(
    user: User,
    control_id: String,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let controls_res = get_controll(db, control_id).await;
    println!("{} is requesting the controls", user.username);
    match controls_res {
        Ok(control) => status::Custom(
            Status::Ok,
            serde_json::to_string(&control).expect("can serialize the control to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the controls: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/?<measure_id>")]
pub(crate) async fn get_controls_for_measure(
    user: User,
    measure_id: &str,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let controls_res = get_controls_for_measurec(measure_id, db).await;
    println!("{} is requesting the controls", user.username);
    match controls_res {
        Ok(controls) => status::Custom(
            Status::Ok,
            serde_json::to_string(&controls).expect("can serialize the controls to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the controls: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/get_completion?<control_id>")]
pub(crate) async fn get_control_completion(
    user: User,
    control_id: String,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let controls_res = get_control_completionn(db, control_id).await;
    println!("{} is requesting the controls", user.username);
    match controls_res {
        Ok(completion) => status::Custom(Status::Ok, completion.to_string()),
        Err(err) => {
            println!("Something went wrong getting the controls: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[post("/get_completion_batch", data = "<req_data>")]
pub(crate) async fn get_control_completion_b(
    user: User,
    req_data: String,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let ids_res = serde_json::from_str::<Vec<String>>(&req_data);
    match ids_res {
        Ok(control_ids) => {
            let controls_res = get_control_completion_batch(db, control_ids).await;
            println!("{} is requesting the controls", user.username);
            match controls_res {
                Ok(completion_vec) => status::Custom(
                    Status::Ok,
                    serde_json::to_string(&completion_vec)
                        .expect("can serialize the controls completion to JSON"),
                ),
                Err(err) => {
                    println!("Something went wrong getting the controls: {}", err);
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

#[get("/get_measures_count_batch")]
pub(crate) async fn get_measures_for_control_count_batch(
    user: User,
    _required_roles: GetControlsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the number of measures associated to controls (batch)",
        user.username
    );
    let controls_res = get_measures_for_control_count_batchh(db).await;
    match controls_res {
        Ok(data) => status::Custom(
            Status::Ok,
            serde_json::to_string(&data)
                .expect("can serialize the number of associated measures (batch) to JSON"),
        ),
        Err(err) => {
            println!(
                "Something went wrong getting the number of associated measures (batch): {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UpdateControlFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: Option<String>,
    #[field(validate = len(3..300))]
    pub(crate) description: Option<String>,
}

#[patch("/<control_id>", data = "<form_data>")]
pub(crate) async fn update_control(
    mut form_data: Form<UpdateControlFormData>,
    control_id: String,
    _r: EditControlRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let control_res = get_controll(db, control_id.clone()).await;
    match control_res {
        Ok(control_opt) => match control_opt {
            Some(mut control) => {
                if let Some(d) = form_data.description.take() {
                    control.description = d;
                }
                if let Some(t) = form_data.title.take() {
                    control.title = t;
                }
                let update_res = update_controlc(control, db).await;
                match update_res {
                    Ok(_) => status::Custom(Status::Ok, format!("{control_id}").to_string()),
                    Err(err) => {
                        println!("Could not update the control {control_id}: {err:?}");
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
            println!("Could not get the control to update: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

generate_endpoint_roles!(GetControlsMeasuresRole, { Role::GetMeasures, Role::GetControls });
#[get("/get_measure_control_association_batch")]
pub(crate) async fn get_measure_control_association_batch(
    user: User,
    _required_roles: GetControlsMeasuresRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the measure-control association (batch)",
        user.username
    );
    let controls_res = get_measure_control_association(db).await;
    match controls_res {
        Ok(data) => status::Custom(
            Status::Ok,
            serde_json::to_string(&data)
                .expect("can serialize the measure-control association (batch) to JSON"),
        ),
        Err(err) => {
            println!(
                "Something went wrong getting the measure-control association (batch): {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}
