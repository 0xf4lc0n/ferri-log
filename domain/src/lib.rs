mod entities;
mod errors;

pub mod prelude {
    pub use super::entities::{
        log_entry::LogEntry, log_entry_filter::LogEntryFilter,
        log_entry_filter::LogEntryFilterQueryBuilder,
    };
    pub use super::errors::{ReposiotryResult, RepositoryError};
}
