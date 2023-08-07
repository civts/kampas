use crate::{
    helpers::surrealdb::{add_control as add_controll, get_controls as get_controlss},
    models::{control::Control, user::User},
};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/", data = "<form_data>")]
pub(crate) async fn add_control(form_data: Form<Control>, db: &State<Surreal<Client>>) -> String {
    let control = form_data.into_inner();
    match add_controll(control.clone(), db).await {
        Ok(_) => {
            format!("Added {:?}!", &control)
        }
        Err(_) => "nope".to_string(),
    }
}

#[get("/")]
pub(crate) async fn get_controls(
    user: User,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let controls_res = get_controlss(db).await;
    println!("{} is requesting the controls", user.username);
    match controls_res {
        Ok(controls) => status::Custom(Status::Ok, serde_json::to_string(&controls).unwrap()),
        Err(_) => status::Custom(
            Status::InternalServerError,
            "Internal Server Error".to_string(),
        ),
    }
}
