mod configuration;

use anyhow::Result;
use application::prelude::FileSystem;
use infrastructure::prelude::{LinuxFS, SkyTableCache};
use infrastructure::telemetry;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config = configuration::get_configuration().unwrap();

    let cache = Arc::new(SkyTableCache::new("127.0.0.1", 2003));
    let file_system = LinuxFS::new(cache);

    file_system
        .watch_file(&config.application.folder_to_watch)
        .await?;

    Ok(())
}
