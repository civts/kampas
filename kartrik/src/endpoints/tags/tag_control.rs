use crate::models::{role::Role, user::User};
use crate::{generate_endpoint_roles, helpers::surrealdb};
use ::surrealdb::{engine::remote::ws::Client, Surreal};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::{form::Form, http::Status, State};
use std::collections::HashSet;

#[derive(Debug, FromForm)]
pub(crate) struct AddTagToControlFormData {
    control_id: String,
    tag_id: String,
}

generate_endpoint_roles!(AddTagsRole, { Role::EditControls });
#[post("/tag_control", data = "<form_data>")]
/// Create a new tag
pub(crate) async fn add_tag_to_control(
    form_data: Form<AddTagToControlFormData>,
    _r: AddTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    match surrealdb::tag::tag_control(&form_data.control_id, &form_data.tag_id, db).await {
        Ok(_) => {
            println!(
                "Control {} tagged with {}",
                form_data.control_id, form_data.tag_id
            );
            status::Custom(Status::Ok, "Tagged successfully!".to_string())
        }
        Err(err) => {
            println!(
                "Error tagging control {} with {}: {:?}",
                form_data.control_id, form_data.tag_id, err
            );
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}

#[delete("/tag_control", data = "<form_data>")]
/// Create a new tag
pub(crate) async fn remove_tag_from_control(
    form_data: Form<AddTagToControlFormData>,
    _r: AddTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    match surrealdb::tag::untag_control(&form_data.control_id, &form_data.tag_id, db).await {
        Ok(_) => {
            println!(
                "Control {} un-tagged with {}",
                form_data.control_id, form_data.tag_id
            );
            status::Custom(Status::Ok, "Untagged successfully!".to_string())
        }
        Err(err) => {
            println!(
                "Error un-tagging control {} with {}: {:?}",
                form_data.control_id, form_data.tag_id, err
            );
            status::Custom(
                Status::InternalServerError,
                "Internal Server Error".to_string(),
            )
        }
    }
}
