use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub host: String,
    pub severity: String,
    pub facility: String,
    pub syslog_tag: String,
    pub source: String,
    pub message: String,
}
