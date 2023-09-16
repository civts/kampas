use config::Config;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::helpers::error::Error;

/// Returns a new surrealdb client to which we can talk
pub(crate) async fn get_client() -> Result<Surreal<Client>, Error> {
    let settings_res = Config::builder()
        .add_source(config::File::with_name("./config/Default.toml"))
        .add_source(config::Environment::with_prefix("KAMPAS"))
        .build()
        .and_then(|settings| settings.try_deserialize::<HashMap<String, String>>());

    let retry_delay = Duration::from_secs(10);
    let mut remaining_attempts = 12;

    match settings_res {
        Ok(settings) => {
            let mut connection_res: Result<Surreal<Client>, Error> = Err(Error::ConnectionError(
                "No connection attempt was made".to_string(),
            ));
            while remaining_attempts > 0 {
                connection_res = attempt_connection(&settings).await;
                match &connection_res {
                    Ok(_) => {
                        println!("Connection to surrealdb established successfully");
                        break;
                    }
                    Err(err) => {
                        remaining_attempts = remaining_attempts - 1;
                        println!("Could not connect to db. Rertying in {} seconds. {remaining_attempts} retries left. Err: {err:?})", retry_delay.as_secs());
                        thread::sleep(retry_delay);
                    }
                }
            }
            connection_res
        }
        Err(err) => Err(Error::ConnectionError(
            format!("Parse the settings: {err}").to_string(),
        )),
    }
}

async fn attempt_connection(settings: &HashMap<String, String>) -> Result<Surreal<Client>, Error> {
    let url = settings
        .get("surreal_url")
        .ok_or(Error::ConnectionError("The url for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_URL env variable".to_string()))?;
    // Connect to the server
    let db = Surreal::new::<Ws>(url.as_str()).await?;

    // Signin as a namespace, database, or root user
    let username = settings
        .get("surreal_user")
        .ok_or(Error::ConnectionError("The user for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_USER env variable".to_string()))?;
    let password = settings
        .get("surreal_password")
        .ok_or(Error::ConnectionError("The password for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_PASSWORD env variable".to_string()))?;
    db.signin(Root { username, password }).await?;

    let namespace = settings
        .get("surreal_namespace")
        .ok_or(Error::ConnectionError("The namespace for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_NAMESPACE env variable".to_string()))?;
    let database = settings
        .get("surreal_database")
        .ok_or(Error::ConnectionError("The database for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_DATABASE env variable".to_string()))?;
    // Select a specific namespace / database
    db.use_ns(namespace).use_db(database).await?;

    Ok(db)
}
