use crate::{
    application::prelude::LogRepository,
    domain::prelude::{LogEntry, RepositoryError},
    infrastructure::prelude::PgLogRepo,
};
use actix_web::{web, HttpResponse, Responder};
use tracing::{error, info};
use uuid::Uuid;

#[tracing::instrument(name = "Get all logs", skip(log_repo))]
pub async fn get_all_logs(log_repo: web::Data<PgLogRepo>) -> web::Json<Vec<LogEntry>> {
    let logs = log_repo.get_all_logs().await.expect("Cannot get all logs");
    web::Json(logs)
}

pub async fn get_log_by_id(
    log_id: web::Path<Uuid>,
    log_repo: web::Data<PgLogRepo>,
) -> impl Responder {
    match log_repo.get_log_by_id(*log_id).await {
        Ok(log) => HttpResponse::Ok().json(log),
        Err(RepositoryError::Database(sqlx::Error::RowNotFound)) => {
            info!("Log entry with id '{}' doesn't exist", log_id);
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            error!("Cannot get log entry with id '{}'. Reason: {:?}", log_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
