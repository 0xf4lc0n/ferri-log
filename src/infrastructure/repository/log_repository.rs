use async_trait::async_trait;
use sqlx::{postgres::PgRow, types::chrono, PgPool, Row};
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::{
    application::prelude::{DiskLogEntryDto, LogRepository},
    domain::prelude::{LogEntry, LogEntryFilter, LogEntryFilterQueryBuilder, ReposiotryResult},
};

pub struct PgLogRepo {
    pool: PgPool,
}

impl PgLogRepo {
    pub fn new(pool: PgPool) -> Self {
        PgLogRepo { pool }
    }
}

#[async_trait]
impl LogRepository for PgLogRepo {
    #[instrument(name = "Retrieving one log entry from the database", skip(self))]
    async fn get_log_by_id(&self, id: uuid::Uuid) -> ReposiotryResult<LogEntry> {
        let log = sqlx::query_as!(LogEntry, "SELECT * FROM logs WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(log)
    }

    #[instrument(name = "Retrieving all logs from the database", skip_all)]
    async fn get_all_logs(&self) -> ReposiotryResult<Vec<LogEntry>> {
        let logs = sqlx::query_as!(LogEntry, "SELECT * FROM logs")
            .fetch_all(&self.pool)
            .await?;

        Ok(logs)
    }

    #[instrument(
        name = "Retrieving logs matching the filter from the database",
        skip(self)
    )]
    async fn get_logs_by_filter(&self, filter: LogEntryFilter) -> ReposiotryResult<Vec<LogEntry>> {
        let mut query_builder = LogEntryFilterQueryBuilder::new(filter);
        let query = query_builder.to_sql_query();
        // debug!("Sqlx query: {}", &query);

        let logs = query.fetch_all(&self.pool).await?;

        // let logs = sqlx::query_as::<_, LogEntry>(&query)
        //     .fetch_all(&self.pool)
        //     .await?;

        Ok(logs)
    }

    #[instrument(name = "Creating log entry in the database", skip(self))]
    async fn create_log(&self, dto: DiskLogEntryDto) -> ReposiotryResult<Uuid> {
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
        .await?;

        Ok(id)
    }

    #[instrument(name = "Deleting log entry from the database", skip(self))]
    async fn delete_log(&self, id: uuid::Uuid) -> ReposiotryResult<()> {
        sqlx::query!("DELETE FROM logs WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
