mod configuration;
mod routes;
mod startup;

use anyhow::Result;
use infrastructure::prelude::{
    get_subscriber, init_subscriber, watch_dir, LinuxFS, PgLogRepo, SkyTableCache,
};
use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use startup::run;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "error".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = configuration::get_configuration().unwrap();

    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let cache = SkyTableCache::new("127.0.0.1", 2003);
    let log_repo = PgLogRepo::new(connection_pool.clone());
    let file_system = Arc::new(LinuxFS::new(cache, log_repo));

    let _watcher = watch_dir(&config.application.folder_to_watch, file_system)?;

    let address = format!("{}:{}", config.application.host, config.application.port);

    run(address, connection_pool, &config)?
        .await
        .expect("Failed to start HTTP server");

    Ok(())
}
