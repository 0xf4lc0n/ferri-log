use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait FileSystem {
    async fn watch_file(&self, path: &str) -> Result<()>;
}
