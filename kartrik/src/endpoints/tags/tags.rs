use crate::models::tag::Tag;
use crate::models::{role::Role, user::User};
use crate::{generate_endpoint_roles, helpers::surrealdb};
use ::surrealdb::{engine::remote::ws::Client, Surreal};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::json::serde_json;
use rocket::{form::Form, http::Status, State};
use std::collections::HashSet;

#[derive(Debug, FromForm)]
pub(crate) struct AddTagData {
    name: String,
    color_hex: String,
}

generate_endpoint_roles!(AddTagsRole, { Role::CreateTags });
#[post("/", data = "<form_data>")]
/// Create a new tag
pub(crate) async fn add_tag(
    form_data: Form<AddTagData>,
    _r: AddTagsRole,
    db: &State<Surreal<Client>>,
) -> String {
    let control_form_data = form_data.into_inner();
    let clean_color = &control_form_data.color_hex.replace("#", "");
    let tag_to_add = Tag::new(&control_form_data.name, clean_color);
    match surrealdb::tag::add_tag(tag_to_add, db).await {
        Ok(id) => {
            format!("Added {:?}!", &id)
        }
        Err(_) => "nope".to_string(),
    }
}

generate_endpoint_roles!(GetTagsRole, { Role::GetTags });
#[get("/")]
/// Get all the tags
pub(crate) async fn get_tags(
    _r: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    match surrealdb::tag::get_tags(db).await {
        Ok(tags) => status::Custom(
            Status::Ok,
            serde_json::to_string(&tags).expect("can serialize the tags to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the tags: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/?<control_id>")]
/// Get all the tags for a control
pub(crate) async fn get_tags_for_control(
    _r: GetTagsRole,
    control_id: &str,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    match surrealdb::tag::get_tags_for_control(control_id, db).await {
        Ok(tags) => status::Custom(
            Status::Ok,
            serde_json::to_string(&tags).expect("can serialize the tags to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the tags: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[post("/get_tags_batch", data = "<reqdata>")]
pub(crate) async fn get_tags_batch(
    user: User,
    reqdata: String,
    _required_roles: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let ids_res = serde_json::from_str::<Vec<String>>(&reqdata);
    match ids_res {
        Ok(control_ids) => {
            let tags_res = surrealdb::tag::get_tags_batch(db, control_ids).await;
            println!("{} is requesting the tags", user.username);
            match tags_res {
                Ok(tag_vecs) => status::Custom(
                    Status::Ok,
                    serde_json::to_string(&tag_vecs)
                        .expect("can serialize the tags completion to JSON"),
                ),
                Err(err) => {
                    println!("Something went wrong getting the tags: {}", err);
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
