use actix_web::{web, HttpResponse};
use application::prelude::{BlacklistRepository, LogRepository};
use domain::prelude::BlacklistEntry;
use infrastructure::prelude::{PgBlkLstRepo, PgLogRepo};
use uuid::Uuid;

#[tracing::instrument(name = "Adding log to blacklist", skip(log_repo, blklst_repo))]
pub async fn add_to_blacklist(
    log_id: web::Json<Uuid>,
    log_repo: web::Data<PgLogRepo>,
    blklst_repo: web::Data<PgBlkLstRepo>,
) -> HttpResponse {
    let log = log_repo
        .get_log_by_id(log_id.0)
        .await
        .expect("Cannot get log by ID");

    let message = {
        let trimed_message = log.message.trim();

        if trimed_message.starts_with('[') {
            trimed_message
                .find(']')
                .map(|idx| trimed_message[idx..].trim_start())
        } else {
            None
        }
    };

    // ToDo: Handle "already blacklisted case"
    let entry_id = blklst_repo
        .create_entry(
            log.id,
            &log.source,
            &log.facility,
            message.unwrap_or(&log.message),
        )
        .await
        .expect("Cannot add blacklist entry");

    HttpResponse::Created()
        .append_header(("Location", format!("/logs/blacklist/{}", entry_id)))
        .finish()
}

#[tracing::instrument(name = "Retrieving all entries from the blacklist", skip(blklst_repo))]
pub async fn get_blacklist(blklst_repo: web::Data<PgBlkLstRepo>) -> web::Json<Vec<BlacklistEntry>> {
    let entries = blklst_repo
        .get_all_entries()
        .await
        .expect("Cannot get blacklist entries");
    web::Json(entries)
}

#[tracing::instrument(name = "Retrieving one entry from the blacklist", skip(blklst_repo))]
pub async fn get_blacklist_entry_by_id(
    entry_id: web::Json<Uuid>,
    blklst_repo: web::Data<PgBlkLstRepo>,
) -> web::Json<BlacklistEntry> {
    let entry = blklst_repo
        .get_entry_by_id(entry_id.0)
        .await
        .expect("Cannot get blacklist entries");

    web::Json(entry)
}

#[tracing::instrument(name = "Deleting log from blacklist", skip(blklst_repo))]
pub async fn delete_entry_from_blacklist(
    entry_id: web::Path<Uuid>,
    blklst_repo: web::Data<PgBlkLstRepo>,
) -> HttpResponse {
    blklst_repo
        .delete_entry(*entry_id)
        .await
        .expect("Cannot add blacklist entry");

    HttpResponse::Ok().finish()
}
