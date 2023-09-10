use config::Config;
use std::collections::HashMap;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

/// Returns a new surrealdb client to which we can talk
pub(crate) async fn get_client() -> surrealdb::Result<Surreal<Client>> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./config/Default.toml"))
        .add_source(config::Environment::with_prefix("KAMPAS"))
        .build()
        .unwrap();

    let settings = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let url = settings
        .get("surreal_url")
        .unwrap_or_else(|| panic!("The url for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_URL env variable"));
    // Connect to the server
    let db = Surreal::new::<Ws>(url.as_str()).await?;

    // Signin as a namespace, database, or root user
    let username = settings
        .get("surreal_user")
        .unwrap_or_else(|| panic!("The user for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_USER env variable"));
    let password = settings
        .get("surreal_password")
        .unwrap_or_else(|| panic!("The password for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_PASSWORD env variable"));
    db.signin(Root {
        username: username,
        password: password,
    })
    .await?;

    let namespace = settings
        .get("surreal_namespace")
        .unwrap_or_else(|| panic!("The namespace for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_NAMESPACE env variable"));
    let database = settings
        .get("surreal_database")
        .unwrap_or_else(|| panic!("The database for surrealdb must be specified. You may use the config file or the KAMPAS_SURREAL_DATABASE env variable"));
    // Select a specific namespace / database
    db.use_ns(namespace).use_db(database).await?;

    Ok(db)
}
