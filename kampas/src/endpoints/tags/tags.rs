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
// There should be a cleaner way to do this, but time is running thin
#[get("/?<control_id>&<measure_id>")]
pub(crate) async fn get_tags_for_control(
    user: User,
    _required_roles: GetTagsRole,
    control_id: Option<&str>,
    measure_id: Option<&str>,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    match control_id {
        //get tags for control
        Some(control_id) => match surrealdb::tag::get_tags_for_control(control_id, db).await {
            Ok(tags) => {
                println!("{} tags for {}", &tags.len(), control_id);
                status::Custom(
                    Status::Ok,
                    serde_json::to_string(&tags).expect("can serialize the tags to JSON"),
                )
            }
            Err(err) => {
                println!("Something went wrong getting the tags: {}", err);
                status::Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.reason_lossy().to_string(),
                )
            }
        },
        None => match measure_id {
            //get tags for measure
            Some(measure_id) => {
                println!(
                    "{} is requesting the tags for measure {measure_id}",
                    user.username
                );
                let measures_res = surrealdb::measure::get_tags_for_measure(&measure_id, db).await;
                match measures_res {
                    Ok(tags) => {
                        let a = serde_json::to_string(&tags)
                            .expect("can serialize the measure tags to JSON");
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
            None => {
                //get all tags
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
        },
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

#[get("/get_measure_tag_ids_batch")]
pub(crate) async fn get_measure_tag_ids_batch(
    user: User,
    _required_roles: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let tags_res = surrealdb::tag::get_measure_tag_ids_batch(db).await;
    println!("{} is requesting the measure tags (batch)", user.username);
    match tags_res {
        Ok(measure_to_tags) => status::Custom(
            Status::Ok,
            serde_json::to_string(&measure_to_tags)
                .expect("can serialize the measure tags (batch) completion to JSON"),
        ),
        Err(err) => {
            println!(
                "Something went wrong getting the measure tags (batch): {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}
