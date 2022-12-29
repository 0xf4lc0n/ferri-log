use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use infrastructure::prelude::{PgBlkLstRepo, PgLogRepo};
use openssl::{
    ssl::{
        SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod, SslSessionCacheMode,
        SslVerifyMode, SslVersion,
    },
    x509::{store::X509StoreBuilder, X509},
};
use sqlx::PgPool;
use std::fs;
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::Settings,
    middlewares::{get_client_cert, Auth},
    routes::{
        add_to_blacklist, delete_entry_from_blacklist, get_all_logs, get_blacklist,
        get_blacklist_entry_by_id, get_log_by_id, get_logs_by_filter, health_check,
    },
};

pub fn run(address: String, db_pool: PgPool, settings: &Settings) -> Result<Server> {
    let log_repo = Data::new(PgLogRepo::new(db_pool.clone()));
    let blacklist = Data::new(PgBlkLstRepo::new(db_pool));

    let ssl_builder = setup_certificate_auth(settings)?;

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(settings.application.one_request_replenishment_time)
        .burst_size(settings.application.request_pool)
        .finish()
        .unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(Auth)
            .route("/health_check", web::get().to(health_check))
            .route("/logs/blacklist", web::post().to(add_to_blacklist))
            .route("/logs/blacklist", web::get().to(get_blacklist))
            .route(
                "/logs/blacklist/{entry_id}",
                web::delete().to(delete_entry_from_blacklist),
            )
            .route(
                "/logs/blacklist{entry_id}",
                web::get().to(get_blacklist_entry_by_id),
            )
            .route("/logs", web::get().to(get_all_logs))
            .route("/logs/filtered", web::get().to(get_logs_by_filter))
            .route("/logs/{log_id}", web::get().to(get_log_by_id))
            .app_data(log_repo.clone())
            .app_data(blacklist.clone())
    })
    .on_connect(get_client_cert)
    .bind_openssl(address, ssl_builder)?
    .run();

    Ok(server)
}

fn setup_certificate_auth(settings: &Settings) -> Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())?;
    builder.set_private_key_file(&settings.certificates.server_key_path, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(&settings.certificates.server_cert_path)?;

    let ca_cert = fs::read_to_string(&settings.certificates.ca_cert_path)?.into_bytes();

    let client_ca_cert = X509::from_pem(&ca_cert)?;
    let mut x509_client_store_builder = X509StoreBuilder::new()?;
    x509_client_store_builder.add_cert(client_ca_cert)?;
    let client_cert_store = x509_client_store_builder.build();
    builder.set_verify_cert_store(client_cert_store)?;

    let mut verify_mode = SslVerifyMode::empty();
    verify_mode.set(SslVerifyMode::PEER, true);
    builder.set_verify(verify_mode);

    builder.set_session_cache_mode(SslSessionCacheMode::OFF);
    let min_ssl_verion = Some(SslVersion::TLS1_2);
    builder.set_min_proto_version(min_ssl_verion)?;

    Ok(builder)
}
