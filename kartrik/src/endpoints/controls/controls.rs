use crate::{
    generate_endpoint_roles,
    helpers::surrealdb::{
        add_control as add_controll, get_control as get_controll, get_controls as get_controlss,
    },
    models::{control::Control, role::Role, user::User},
};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use std::collections::HashSet;
use surrealdb::{engine::remote::ws::Client, Surreal};

generate_endpoint_roles!(EditControlRoles, { Role::EditControls, Role::GetControls });
#[post("/", data = "<form_data>")]
pub(crate) async fn add_control(
    form_data: Form<Control>,
    _r: EditControlRoles,
    db: &State<Surreal<Client>>,
) -> String {
    let control = form_data.into_inner();
    match add_controll(control.clone(), db).await {
        Ok(_) => {
            format!("Added {:?}!", &control)
        }
        Err(_) => "nope".to_string(),
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
