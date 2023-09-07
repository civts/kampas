use crate::models::control::Control;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

use super::Record;

pub(crate) async fn add_control(control: Control, db: &Surreal<Client>) -> surrealdb::Result<()> {
    // Create a new control with a random id
    let _created: Record = db
        .create(("control", &control.identifier))
        .content(control)
        .await?;
    Ok(())
}

pub(crate) async fn get_controls(db: &Surreal<Client>) -> surrealdb::Result<Vec<Control>> {
    // Select all records
    let controls: Vec<Control> = db.select("control").await?;

    Ok(controls)
}

pub(crate) async fn get_controls_for_metric(
    metric_id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Vec<Control>> {
    let controls: Vec<Control> = db
        .query("SELECT * FROM control WHERE id INSIDE $metric_id->measures.out")
        .bind((
            "metric_id",
            Thing {
                id: Id::String(metric_id.to_string()),
                tb: "metric".to_string(),
            },
        ))
        .await?
        .take(0)?;

    Ok(controls)
}

pub(crate) async fn get_control(
    db: &Surreal<Client>,
    id: String,
) -> surrealdb::Result<Option<Control>> {
    let control: Option<Control> = db.select(("control", id)).await?;

    Ok(control)
}
