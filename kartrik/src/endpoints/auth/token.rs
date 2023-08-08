use crate::helpers::cryptography::hash_salted_password;
use crate::helpers::surrealdb::add_token;
use crate::helpers::surrealdb::get_token;
use crate::helpers::surrealdb::get_user;
use crate::models::token::Token;
use blake3::Hash;
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(FromForm)]
pub(crate) struct LoginFormData {
    user_id: String,
    password: String,
}

/// Generates a new token for the user
#[post("/generate_token", data = "<form_data>")]
pub(crate) async fn generate_token(
    form_data: Form<LoginFormData>,
    db: &State<Surreal<Client>>,
) -> status::Custom<String> {
    let user_id = &form_data.user_id;
    let user_res = get_user(user_id, db).await;
    match user_res {
        Ok(user_opt) => match user_opt {
            Some(user) => {
                assert!(
                    &user.username == user_id,
                    "The user we fetched must be the same as the logged in one"
                );

                // Note: the Hash type we are using implements contant-time comparison.
                // This is essential to prevent time-based attacks
                let db_password_hash = Hash::from_hex(user.password_hash)
                    .expect("The password hash in the db is valid");
                let password_hash = Hash::from_hex(hash_salted_password(
                    &form_data.password,
                    &user.password_salt,
                ))
                .expect("This must be a valid hash, else we can't program");
                if password_hash == db_password_hash {
                    let previous_token_opt = get_token(user_id, db).await.unwrap_or(None);
                    if let Some(token) = previous_token_opt {
                        if token.is_expired() {
                            println!("Token expired, generating a new one");
                            let token = Token::new(user_id);
                            add_token(&token, db)
                                .await
                                .expect("Can add the token to the DB");
                            status::Custom(Status::Ok, token.token)
                        } else {
                            status::Custom(Status::Ok, token.token)
                        }
                    } else {
                        let token = Token::new(user_id);
                        add_token(&token, db)
                            .await
                            .expect("Can add the token to the DB");
                        status::Custom(Status::Ok, token.token)
                    }
                } else {
                    status::Custom(Status::Forbidden, "Wrong credentials".to_string())
                }
            }
            None => {
                println!("No such user: {user_id:?}");
                status::Custom(Status::NotFound, "No such user".to_string())
            }
        },
        Err(err) => {
            println!("Could not get user from db: {err:?}");
            status::Custom(Status::InternalServerError, "Oops".to_string())
        }
    }
}
