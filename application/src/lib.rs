mod dto;
mod interfaces;

pub mod prelude {
    pub use super::dto::disk_log_entry_dto::DiskLogEntryDto;
    pub use super::interfaces::{
        cache::Cache, fs::FileSystem, repository::blacklist_repository::BlacklistRepository,
        repository::log_repository::LogRepository,
    };
}
