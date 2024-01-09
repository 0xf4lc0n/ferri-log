mod configuration;
mod middlewares;
mod routes;
mod startup;

use anyhow::Result;
use infrastructure::prelude::{
    get_subscriber, init_subscriber, watch_dir, LinuxFS, PgLogRepo, SkyTableCache,
};
use std::{process::Command, sync::Arc, time::Duration};
use sysinfo::System;
use tokio::time::sleep;
use tracing::info;

use sqlx::postgres::PgPoolOptions;
use startup::run;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "error".into(), std::io::stdout);
    init_subscriber(subscriber);

    run_rsyslogd_if_not_present().await;

    let config = configuration::get_configuration().unwrap();

    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let cache = SkyTableCache::new(&config.cache.host, config.cache.port);
    let log_repo = PgLogRepo::new(connection_pool.clone());
    let file_system = Arc::new(LinuxFS::new(cache, log_repo));

    let _watcher = watch_dir(&config.application.folder_to_watch, file_system)?;

    let address = format!("{}:{}", config.application.host, config.application.port);

    run(address, connection_pool, &config)?
        .await
        .expect("Failed to start HTTP server");

    Ok(())
}

async fn run_rsyslogd_if_not_present() {
    let s = System::new_all();
    if s.processes_by_name("rsyslog").count() == 0 {
        info!("Rsyslogd is not running!");
        info!("Starting rsyslogd...");
        Command::new("rsyslogd")
            .spawn()
            .expect("Failed to run rsyslogd");
        sleep(Duration::from_secs(5)).await;
    }
}
