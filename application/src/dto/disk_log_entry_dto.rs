use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskLogEntryDto {
    pub timestamp: String,
    pub host: String,
    pub severity: String,
    pub facility: String,
    pub syslog_tag: String,
    pub source: String,
    pub message: String,
}
