use super::Record;
use crate::models::ranking::Ranking;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Id;
use surrealdb::Surreal;

pub(crate) async fn add_ranking(
    ranking: Ranking,
    db: &Surreal<Client>,
) -> surrealdb::Result<String> {
    // Create a new ranking with a random id
    let created_opt: Option<Record> = db
        .create(("ranking", &ranking.identifier))
        .content(&ranking)
        .await?;
    match created_opt {
        Some(r) => match r.id.id {
            Id::String(id_str) => Ok(id_str),
            Id::Number(id) => Ok(id.to_string()),
            _ => panic!("We don't do that here. We shall only use String IDs"),
        },
        None => Err(surrealdb::Error::Api(surrealdb::error::Api::InternalError(
            format!("Ranking with id {} was not created", &ranking.identifier).to_string(),
        ))),
    }
}

pub(crate) async fn get_rankings(db: &Surreal<Client>) -> surrealdb::Result<Vec<Ranking>> {
    // Select all records
    let rankings: Vec<Ranking> = db.select("ranking").await?;

    Ok(rankings)
}

pub(crate) async fn get_ranking(
    id: &str,
    db: &Surreal<Client>,
) -> surrealdb::Result<Option<Ranking>> {
    // Select all records
    let rankings: Option<Ranking> = db.select(("ranking", id)).await?;

    Ok(rankings)
}
