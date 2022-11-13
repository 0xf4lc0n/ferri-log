use application::prelude::BlacklistRepository;
use async_trait::async_trait;
use domain::prelude::{BlacklistEntry, ReposiotryResult};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

pub struct PgBlkLstRepo {
    pool: PgPool,
}

impl PgBlkLstRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BlacklistRepository for PgBlkLstRepo {
    #[instrument(name = "Retrieving one blacklist entry from the database", skip(self))]
    async fn get_entry_by_id(&self, id: Uuid) -> ReposiotryResult<BlacklistEntry> {
        let entry = sqlx::query_as!(BlacklistEntry, "SELECT * FROM blacklist WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(entry)
    }

    #[instrument(name = "Retrieving one blacklist entry from the database", skip(self))]
    async fn get_entry_by_props(
        &self,
        source: String,
        facility: String,
        message: String,
    ) -> ReposiotryResult<BlacklistEntry> {
        let entry = sqlx::query_as!(
            BlacklistEntry,
            "SELECT * FROM blacklist WHERE source = $1 AND facility = $2 AND message = $3",
            source,
            facility,
            message
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }

    #[instrument(
        name = "Retrieving all blacklist entries from the database",
        skip(self)
    )]
    async fn get_all_entries(&self) -> ReposiotryResult<Vec<BlacklistEntry>> {
        let entries = sqlx::query_as!(BlacklistEntry, "SELECT * FROM blacklist")
            .fetch_all(&self.pool)
            .await?;

        Ok(entries)
    }

    #[instrument(name = "Creating new blacklist entry in the database", skip(self))]
    async fn create_entry(
        &self,
        id: Uuid,
        source: &str,
        facility: &str,
        message: &str,
    ) -> ReposiotryResult<Uuid> {
        sqlx::query!(
            r#"
            INSERT INTO blacklist (id, facility, source, message)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            facility,
            source,
            message,
        )
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    #[instrument(name = "Deleting blacklist entry from the database", skip(self))]
    async fn delete_entry(&self, id: Uuid) -> ReposiotryResult<()> {
        sqlx::query!("DELETE FROM blacklist WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
