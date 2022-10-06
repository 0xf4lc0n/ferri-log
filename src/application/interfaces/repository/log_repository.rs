use crate::{
    application::prelude::DiskLogEntryDto,
    domain::prelude::{LogEntry, ReposiotryResult},
};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait LogRepository {
    async fn get_log_by_id(&self, id: uuid::Uuid) -> ReposiotryResult<LogEntry>;
    async fn get_all_logs(&self) -> ReposiotryResult<Vec<LogEntry>>;
    async fn create_log(&self, disk_log_dto: DiskLogEntryDto) -> ReposiotryResult<Uuid>;
    async fn delete_log(&self, id: uuid::Uuid) -> ReposiotryResult<()>;
}
