use crate::{
    endpoints::tags::tags::GetTagsRole,
    generate_endpoint_roles,
    helpers::surrealdb::enabler::{
        add_enabler as add_enablerl, associate_enabler as associate_enablerl,
        get_coverage_for_enabler as get_coverage_for_enablerc, get_enabler as get_enablerl,
        get_enablers as get_enablerss, get_enablers_for_control as get_enablers_for_controll,
        get_number_controls_batch as get_number_controls_batchh,
        get_tags_for_enabler as get_tags_for_enablerc, update_enabler as update_enablerc,
    },
    models::{enabler::Enabler, role::Role, user::User},
};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{form::Form, http::Status, response::status, serde::json::serde_json, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct EnablerFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: String,
    #[field(validate = len(3..300))]
    pub(crate) description: String,
    pub(crate) effort: Option<u8>,
}

generate_endpoint_roles!(EditEnablerRoles, { Role::CreateEnablers });
#[post("/", data = "<form_data>")]
pub(crate) async fn add_enabler(
    form_data: Form<EnablerFormData>,
    _r: EditEnablerRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let enabler = Enabler::new(&form_data.title, &form_data.description, form_data.effort);
    match add_enablerl(enabler.clone(), db).await {
        Ok(id) => status::Custom(Status::Ok, format!("{id}").to_string()),
        Err(err) => {
            println!("Could not create new enabler: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct AssociateEnablerFormData {
    pub(crate) enabler_id: String,
    pub(crate) control_id: String,
    #[field(validate = range(1..101), default = 100)]
    pub(crate) coverage: u8,
}
generate_endpoint_roles!(AssociateEnablerRoles, { Role::AssociateEnablers });
#[post("/associate", data = "<form_data>")]
pub(crate) async fn associate_enabler(
    form_data: Form<AssociateEnablerFormData>,
    _r: AssociateEnablerRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let associate_res = associate_enablerl(
        &form_data.enabler_id,
        &form_data.control_id,
        form_data.coverage,
        db,
    )
    .await;
    match associate_res {
        Ok(_) => status::Custom(Status::Ok, format!("Ok, enabler associated").to_string()),
        Err(err) => {
            println!("Could not associate enabler: {err:?}");
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

generate_endpoint_roles!(GetEnablersRole, { Role::GetEnablers });
#[get("/")]
pub(crate) async fn get_enablers(
    user: User,
    _required_roles: GetEnablersRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let enablers_res = get_enablerss(db).await;
    println!("{} is requesting the enablers", user.username);
    match enablers_res {
        Ok(enablers) => status::Custom(
            Status::Ok,
            serde_json::to_string(&enablers).expect("can serialize the enablers to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the enablers: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/?<control_id>")]
pub(crate) async fn get_enablers_for_control(
    user: User,
    control_id: &str,
    _required_roles: GetEnablersRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let enablers_res = get_enablers_for_controll(control_id, db).await;
    println!(
        "{} is requesting the enablers for control {control_id}",
        user.username
    );
    match enablers_res {
        Ok(enablers) => status::Custom(
            Status::Ok,
            serde_json::to_string(&enablers).expect("can serialize the enablers to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the enablers: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/<enabler_id>")]
pub(crate) async fn get_enabler(
    user: User,
    enabler_id: String,
    _required_roles: GetEnablersRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!("{} is requesting the enabler {enabler_id}", user.username);
    let enablers_res = get_enablerl(db, enabler_id).await;
    match enablers_res {
        Ok(enabler) => status::Custom(
            Status::Ok,
            serde_json::to_string(&enabler).expect("can serialize the enabler to JSON"),
        ),
        Err(err) => {
            println!("Something went wrong getting the enablers: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/coverage_for_enabler?<enabler_id>")]
pub(crate) async fn get_coverage_for_enabler(
    user: User,
    enabler_id: String,
    _required_roles: GetEnablersRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the coverage info for enabler {enabler_id}",
        user.username
    );
    let enablers_res = get_coverage_for_enablerc(&enabler_id, db).await;
    match enablers_res {
        Ok(enabler) => {
            let a = serde_json::to_string(&enabler)
                .expect("can serialize the enabler coverage info to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!(
                "Something went wrong getting the enabler coverage info: {}",
                err
            );
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[get("/tags_for_enabler?<enabler_id>")]
pub(crate) async fn get_tags_for_enabler(
    user: User,
    enabler_id: String,
    _required_roles: GetTagsRole,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    println!(
        "{} is requesting the tags for enabler {enabler_id}",
        user.username
    );
    let enablers_res = get_tags_for_enablerc(&enabler_id, db).await;
    match enablers_res {
        Ok(tags) => {
            let a = serde_json::to_string(&tags).expect("can serialize the enabler tags to JSON");
            status::Custom(Status::Ok, a)
        }
        Err(err) => {
            println!("Something went wrong getting the enabler tags: {}", err);
            status::Custom(
                Status::InternalServerError,
                Status::InternalServerError.reason_lossy().to_string(),
            )
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct UpdateEnablerFormData {
    #[field(validate = len(3..30))]
    pub(crate) title: Option<String>,
    #[field(validate = len(3..300))]
    pub(crate) description: Option<String>,
    pub(crate) effort: Option<u8>,
    pub(crate) progress: Option<u8>,
}

#[patch("/<enabler_id>", data = "<form_data>")]
pub(crate) async fn update_enabler(
    mut form_data: Form<UpdateEnablerFormData>,
    enabler_id: String,
    _r: EditEnablerRoles,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let enabler_res = get_enablerl(db, enabler_id.clone()).await;
    match enabler_res {
        Ok(enabler_opt) => match enabler_opt {
            Some(mut enabler) => {
                if let Some(d) = form_data.description.take() {
                    enabler.description = d;
                }
                if let Some(t) = form_data.title.take() {
                    enabler.title = t;
                }
                if let Some(e) = form_data.effort.take() {
                    if e > 0 {
                        enabler.effort = e;
                    }
                }
                if let Some(p) = form_data.progress.take() {
                    if p > 0 && p <= 100 {
                        enabler.progress = p;
                    }
                }
                let update_res = update_enablerc(enabler, db).await;
                match update_res {
                    Ok(_) => status::Custom(Status::Ok, format!("{enabler_id}").to_string()),
                    Err(err) => {
                        println!("Could not update the enabler {enabler_id}: {err:?}");
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
            println!("Could not get the enabler to update: {err:?}");
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
    _required_roles: GetEnablersRole,
    req_data: String,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let ids_res = serde_json::from_str::<Vec<String>>(&req_data);
    match ids_res {
        Ok(ids) => {
            let progress_res = get_number_controls_batchh(db, ids).await;
            println!(
                "{} is requesting the controls associated to enablers (batch)",
                user.username
            );
            match progress_res {
                Ok(enablers) => status::Custom(
                    Status::Ok,
                    serde_json::to_string(&enablers).expect(
                        "can serialize the controls associated to enablers (batch) to JSON",
                    ),
                ),
                Err(err) => {
                    println!(
                "Something went wrong getting the controls associated to enablers (batch): {}",
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
