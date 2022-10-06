use crate::{
    application::prelude::LogRepository, domain::prelude::LogEntry,
    infrastructure::prelude::PgLogRepo,
};
use actix_web::web;

#[tracing::instrument(name = "Get all logs", skip(log_repo))]
pub async fn get_all_logs(log_repo: web::Data<PgLogRepo>) -> web::Json<Vec<LogEntry>> {
    let logs = log_repo.get_all_logs().await.expect("Cannot get all logs");
    web::Json(logs)
}
