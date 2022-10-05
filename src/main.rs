mod application;
mod domain;
mod infrastructure;
mod server;

use std::{net::TcpListener, sync::Arc};

use anyhow::Result;
use ferri_log::server::{configuration, startup::run};
use infrastructure::prelude::{
    get_subscriber, init_subscriber, watch_dir, LinuxFS, PgLogRepo, SkyTableCache,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = configuration::get_configuration().unwrap();

    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let cache = SkyTableCache::new("127.0.0.1", 2003);
    let log_repo = PgLogRepo::new(connection_pool.clone());
    let file_system = Arc::new(LinuxFS::new(cache, log_repo));

    let _watcher = watch_dir(&config.application.folder_to_watch, file_system)?;

    let address = format!("{}:{}", config.application.host, config.application.port);

    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?
        .await
        .expect("Failed to start HTTP server");

    Ok(())

    // loop {
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    // }
}
