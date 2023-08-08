use crate::helpers::surrealdb::add_user;
use crate::helpers::surrealdb::does_user_exist;
use crate::models::user::User;
use rocket::form::Form;
use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
#[derive(FromForm)]
pub(crate) struct RegisterRqData {
    #[field(validate = len(3..30))]
    username: String,
}

/// Creates a new user
#[post("/register", data = "<req_data>")]
pub(crate) async fn register(
    req_data: Form<RegisterRqData>,
    db: &State<Surreal<Client>>,
) -> String {
    //Ask the db if the user is already present
    if let Ok(true) = does_user_exist(&req_data.username, db).await {
        "Error".to_string()
    } else {
        let add_res = add_user(
            User {
                username: req_data.username.to_owned(),
                password_hash: "TODO".to_string(),
                password_salt: "TODOOOOOOO".to_string(),
            },
            db,
        )
        .await;
        match add_res {
            Ok(_) => "User Created".to_string(),
            Err(_) => "Error".to_string(),
        }
    }
}
