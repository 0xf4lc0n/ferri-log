use async_trait::async_trait;
use domain::prelude::{BlacklistEntry, ReposiotryResult};
use uuid::Uuid;

#[async_trait]
pub trait BlacklistRepository {
    async fn get_entry_by_id(&self, id: Uuid) -> ReposiotryResult<BlacklistEntry>;
    async fn get_entry_by_props(
        &self,
        source: String,
        facility: String,
        message: String,
    ) -> ReposiotryResult<BlacklistEntry>;
    async fn get_all_entries(&self) -> ReposiotryResult<Vec<BlacklistEntry>>;
    async fn create_entry(
        &self,
        id: Uuid,
        source: &str,
        facility: &str,
        message: &str,
    ) -> ReposiotryResult<Uuid>;
    async fn delete_entry(&self, id: Uuid) -> ReposiotryResult<()>;
}
