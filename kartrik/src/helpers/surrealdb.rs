use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub(crate) struct Control {
    #[field(validate = len(3..30))]
    title: String,
    #[field(validate = len(3..300))]
    description: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

/// Returns a new surrealdb client to which we can talk
pub(crate) async fn get_client() -> surrealdb::Result<Surreal<Client>> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8002").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    Ok(db)
}

pub(crate) async fn add_control(control: Control, db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Create a new control with a random id
    let _created: Record = db.create("control").content(control).await?;
    Ok(())
}

pub(crate) async fn get_controls(db: &Surreal<Client>) -> surrealdb::Result<Vec<Control>> {
    // Select all records
    let controls: Vec<Control> = db.select("control").await?;

    Ok(controls)
}

// pub(crate) async fn update_control(db: Surreal<Client>) -> surrealdb::Result<Vec<Record>> {
//     // Update a control record with a specific id
//     let updated: Record = db
//         .update(("control", "jaime"))
//         .merge(Responsibility { marketing: true })
//         .await?;
//     dbg!(updated);
// }

//     // Perform a custom advanced query
//     let groups = db
//         .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
//         .bind(("table", "control"))
//         .await?;
//     dbg!(groups);

//     Ok(())
