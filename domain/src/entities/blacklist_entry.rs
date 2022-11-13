use serde::{Deserialize, Serialize};

use crate::prelude::LogEntry;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlacklistEntry {
    pub id: uuid::Uuid,
    pub facility: String,
    pub source: String,
    pub message: String,
}

impl PartialEq<LogEntry> for BlacklistEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.facility == other.facility
            && self.source == other.source
            && other.message.contains(&self.message)
    }
}
