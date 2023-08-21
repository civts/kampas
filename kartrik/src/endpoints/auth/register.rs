use crate::helpers::cryptography::generate_random_string;
use crate::helpers::cryptography::hash_salted_password;
use crate::helpers::surrealdb::add_user;
use crate::helpers::surrealdb::does_user_exist;
use crate::models::role::Role;
use crate::models::user::User;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
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
) -> status::Custom<&str> {
    //Ask the db if the user is already present
    if let Ok(true) = does_user_exist(&req_data.username, db).await {
        println!("{} already registered", &req_data.username);
        status::Custom(Status::Conflict, "Username already registered")
    } else {
        let salt = generate_random_string(64);
        let password_hash = hash_salted_password(&req_data.password, &salt);

        let add_res = add_user(
            User {
                username: req_data.username.to_owned(),
                password_hash,
                password_salt: salt,
                roles: vec![Role::GetControls],
            },
            db,
        )
        .await;
        match add_res {
            Ok(_) => {
                println!("Added new user: {}", &req_data.username);
                status::Custom(Status::Ok, "User created")
            }
            Err(err) => {
                let username = &req_data.username;
                println!("Could not add user {username}: {err:?}");
                status::Custom(
                    Status::InternalServerError,
                    Status::InternalServerError.reason_lossy(),
                )
            }
        }
    }
}
