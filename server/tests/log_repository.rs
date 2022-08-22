use anyhow::Result;
use application::prelude::{DiskLogEntryDto, LogRepository};
use infrastructure::prelude::{get_subscriber, init_subscriber, PgLogRepo};
use once_cell::sync::Lazy;
use server::configuration::{get_configuration, DatabaseSettings};
use sqlx::{migrate::Migrator, types::Uuid, Connection, Executor, PgConnection, PgPool};
use std::path::{Path, PathBuf};
use tracing::info;

#[tokio::test]
async fn successfully_create_log_entry_in_database() {
    let log_repo = spawn_repo().await;

    let log_dto = DiskLogEntryDto {
        facility: "Repo test".into(),
        host: "localhost".into(),
        message: "Sample message".into(),
        severity: "Info".into(),
        source: "Unit test".into(),
        syslog_tag: "Sample tag".into(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let res = log_repo.create_log(log_dto).await;

    assert!(res.is_ok());
    assert!(!res.unwrap().is_nil())
}

#[tokio::test]
async fn successfully_get_log_entry_from_database() -> Result<()> {
    let log_repo = spawn_repo().await;

    let log_dto = DiskLogEntryDto {
        facility: "Repo test".into(),
        host: "localhost".into(),
        message: "Sample message".into(),
        severity: "Info".into(),
        source: "Unit test".into(),
        syslog_tag: "Sample tag".into(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let log_entry_id = log_repo
        .create_log(log_dto)
        .await
        .expect("Cannot add log entry to database");

    let log = log_repo.get_log_by_id(log_entry_id).await;

    assert_eq!(log?.id, log_entry_id);

    Ok(())
}

#[tokio::test]
async fn successfully_get_all_log_entries_from_database() -> Result<()> {
    let log_repo = spawn_repo().await;

    let log_dto_1 = DiskLogEntryDto {
        facility: "Repo test".into(),
        host: "localhost".into(),
        message: "Sample message".into(),
        severity: "Info".into(),
        source: "Unit test".into(),
        syslog_tag: "Sample tag".into(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let mut log_dto_2 = log_dto_1.clone();
    log_dto_2.facility = "Repo test 2".into();

    let mut log_dto_3 = log_dto_1.clone();
    log_dto_3.facility = "Repo test 3".into();

    let ids = vec![
        log_repo
            .create_log(log_dto_1)
            .await
            .expect("Cannot add log entry to database"),
        log_repo
            .create_log(log_dto_2)
            .await
            .expect("Cannot add log entry to database"),
        log_repo
            .create_log(log_dto_3)
            .await
            .expect("Cannot add log entry to database"),
    ];

    let logs = log_repo.get_all_logs().await;

    assert_eq!(logs.as_ref().unwrap().len(), 3);

    for (log, id) in logs?.iter().zip(ids.iter()) {
        assert_eq!(log.id, *id);
    }

    Ok(())
}

#[tokio::test]
async fn successfully_delete_log_entry_from_database() -> Result<()> {
    let log_repo = spawn_repo().await;

    let log_dto_1 = DiskLogEntryDto {
        facility: "Repo test".into(),
        host: "localhost".into(),
        message: "Sample message".into(),
        severity: "Info".into(),
        source: "Unit test".into(),
        syslog_tag: "Sample tag".into(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let id = log_repo.create_log(log_dto_1).await?;
    let log = log_repo.get_log_by_id(id).await?;
    log_repo.delete_log(log.id).await?;

    Ok(())
}

// Ensure that the 'tracing' stack is only initialised once using 'once_cell'
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_repo() -> PgLogRepo {
    // The first time 'initialise' is invoked the code in 'TRACING' is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database).await;

    PgLogRepo::new(db_pool)
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let migrations = PathBuf::from("../infrastructure/migrations");

    let migrator = Migrator::new(migrations.canonicalize().unwrap().as_path())
        .await
        .expect("Cannot read migrations");

    // Connect to the postgres instance
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres instance");

    // Create database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Connect to the created database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    // Migrate database
    migrator
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}
