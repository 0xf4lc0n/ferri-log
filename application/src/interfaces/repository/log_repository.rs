use crate::dto::disk_log_entry_dto::DiskLogEntryDto;
use async_trait::async_trait;
use ferri_log_core::prelude::LogEntry;
use uuid::Uuid;

#[async_trait]
pub trait LogRepository {
    async fn get_log_by_id(&self, id: uuid::Uuid) -> anyhow::Result<LogEntry>;
    async fn get_all_logs(&self) -> anyhow::Result<Vec<LogEntry>>;
    async fn create_log(&self, disk_log_dto: DiskLogEntryDto) -> anyhow::Result<Uuid>;
    async fn delete_log(&self, id: uuid::Uuid) -> anyhow::Result<()>;
}
