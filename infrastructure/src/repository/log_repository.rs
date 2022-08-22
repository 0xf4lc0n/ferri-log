use anyhow::Result;
use application::prelude::{DiskLogEntryDto, LogRepository};
use async_trait::async_trait;
use ferri_log_core::prelude::LogEntry;
use sqlx::{types::chrono, PgPool};
use tracing::instrument;
use uuid::Uuid;

pub struct PgLogRepo {
    pool: PgPool,
}

impl PgLogRepo {
    pub fn try_new(url: &str) -> Result<Self> {
        Ok(PgLogRepo {
            pool: PgPool::connect_lazy(url)?,
        })
    }

    pub fn new(pool: PgPool) -> Self {
        PgLogRepo { pool }
    }
}

#[async_trait]
impl LogRepository for PgLogRepo {
    #[instrument(name = "Retrieving one log entry from the database", skip(self))]
    async fn get_log_by_id(&self, id: uuid::Uuid) -> anyhow::Result<LogEntry> {
        let log = sqlx::query_as!(LogEntry, "SELECT * FROM logs WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {e:?}");
                e
            })?;

        Ok(log)
    }

    #[instrument(name = "Retrieving all logs from the database", skip_all)]
    async fn get_all_logs(&self) -> anyhow::Result<Vec<LogEntry>> {
        let logs = sqlx::query_as!(LogEntry, "SELECT * FROM logs")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {e:?}");
                e
            })?;

        Ok(logs)
    }

    #[instrument(name = "Creating log entry in the database", skip(self))]
    async fn create_log(&self, dto: DiskLogEntryDto) -> anyhow::Result<Uuid> {
        let date = chrono::DateTime::parse_from_rfc3339(&dto.timestamp)?;
        let id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO logs (id, timestamp, host, severity, facility, syslog_tag, source, message)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            id,
            date,
            dto.host,
            dto.severity,
            dto.facility,
            dto.syslog_tag,
            dto.source,
            dto.message,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {e:?}");
            e
        })?;

        Ok(id)
    }

    #[instrument(name = "Deleting log entry from the database", skip(self))]
    async fn delete_log(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        sqlx::query!("DELETE FROM logs WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {e:?}");
                e
            })?;

        Ok(())
    }
}
