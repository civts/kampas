use std::io;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Connection error: '{0}'")]
    ConnectionError(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] io::Error),
}
