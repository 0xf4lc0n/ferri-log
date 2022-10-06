pub type ReposiotryResult<T> = std::result::Result<T, RepositoryError>;

#[derive(Debug)]
pub enum RepositoryError {
    Database(sqlx::Error),
    Parsing(String),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        RepositoryError::Database(e)
    }
}

impl From<chrono::ParseError> for RepositoryError {
    fn from(e: chrono::ParseError) -> Self {
        RepositoryError::Parsing(e.to_string())
    }
}

impl From<RepositoryError> for anyhow::Error {
    fn from(e: RepositoryError) -> Self {
        anyhow::anyhow!("{:?}", e)
    }
}
