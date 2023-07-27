#[macro_use]
extern crate rocket;
mod helpers;
use helpers::{
    cors::CORS,
    surrealdb::{add_control as add_controll, get_client, get_controls as get_controlss, Control},
};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[post("/api/v1/controls", data = "<form_data>")]
async fn add_control(form_data: Form<Control>, db: &State<Surreal<Client>>) -> String {
    let control = form_data.into_inner();
    match add_controll(control.clone(), db).await {
        Ok(_) => {
            format!("Added {:?}!", &control)
        }
        Err(_) => "nope".to_string(),
    }
}

#[get("/api/v1/controls")]
async fn get_controls(db: &State<Surreal<Client>>) -> status::Custom<String> {
    let controls_res = get_controlss(db).await;
    match controls_res {
        Ok(controls) => status::Custom(Status::Ok, serde_json::to_string(&controls).unwrap()),
        Err(_) => status::Custom(
            Status::InternalServerError,
            "Internal Server Error".to_string(),
        ),
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hi"
}

#[launch]
async fn rocket() -> _ {
    let client = get_client().await.unwrap();
    rocket::build()
        .mount("/", routes![index, hello, add_control, get_controls])
        .attach(CORS)
        .manage(client)
}
