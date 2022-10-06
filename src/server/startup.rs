use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Result;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::infrastructure::prelude::PgLogRepo;

use super::routes::{get_all_logs, get_log_by_id, health_check};

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server> {
    let log_repo = Data::new(PgLogRepo::new(db_pool));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/logs", web::get().to(get_all_logs))
            .route("/logs/{log_id}", web::get().to(get_log_by_id))
            .app_data(log_repo.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
