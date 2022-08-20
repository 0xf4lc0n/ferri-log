pub mod dto;
pub mod interfaces;

pub mod prelude {
    pub use crate::dto::disk_log_entry_dto::DiskLogEntryDto;
    pub use crate::interfaces::{
        cache::Cache, fs::FileSystem, repository::log_repository::LogRepository,
    };
}
