use crate::helpers::cryptography::generate_random_string;
use crate::helpers::cryptography::hash_salted_password;
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
    #[field(validate = len(8..257))]
    password: String,
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
        let salt = generate_random_string(64);
        let password_hash = hash_salted_password(&req_data.password, &salt);

        let add_res = add_user(
            User {
                username: req_data.username.to_owned(),
                password_hash,
                password_salt: salt,
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
