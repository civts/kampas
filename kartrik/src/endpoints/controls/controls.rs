use crate::{
    generate_endpoint_roles,
    helpers::surrealdb::control::{
        add_control as add_controll, get_control as get_controll, get_controls as get_controlss,
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

generate_endpoint_roles!(EditControlRoles, { Role::EditControls, Role::GetControls });
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
                "Internal Server Error".to_string(),
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
                "Internal Server Error".to_string(),
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
                "Internal Server Error".to_string(),
            )
        }
    }
}
