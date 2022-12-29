use anyhow::{anyhow, Result};
use config::{File, FileFormat};
use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub certificates: CertificateSettings,
    pub cache: CacheSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub folder_to_watch: String,
    pub host: String,
    pub port: u16,
    pub request_pool: u32,
    pub one_request_replenishment_time: u64,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);
        options.log_statements(tracing::log::LevelFilter::Trace);
        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

pub fn get_configuration() -> Result<Settings> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path
        .join("configuration/config")
        .into_os_string()
        .into_string()
        .map_err(|path| anyhow!("Cannot convert {path:?} to String"))?;

    let config = config::Config::builder()
        .add_source(File::new(&configuration_directory, FileFormat::Toml))
        .build()?;

    config
        .try_deserialize()
        .map_err(|err| anyhow!("Cannot deserialize Config to Settings struct {err:?}"))
}

#[derive(serde::Deserialize)]
pub struct CertificateSettings {
    pub ca_cert_path: String,
    pub server_cert_path: String,
    pub server_key_path: String,
}

#[derive(serde::Deserialize)]
pub struct CacheSettings {
    pub host: String,
    pub port: u16,
}
