use anyhow::Result;
use async_trait::async_trait;
use notify::Event;

#[async_trait]
pub trait FileSystem {
    async fn handle_event(&self, event: Event) -> Result<()>;
}
