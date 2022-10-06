mod entities;
mod errors;
pub mod prelude {
    pub use super::entities::log_entry::LogEntry;
    pub use super::errors::{ReposiotryResult, RepositoryError};
}
