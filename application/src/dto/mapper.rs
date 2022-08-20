use ferri_log_core::prelude::LogEntry;

use crate::prelude::DiskLogEntryDto;

impl From<LogEntry> for DiskLogEntryDto {
    fn from(log: LogEntry) -> Self {
        DiskLogEntryDto {
            timestamp: log.timestamp.to_string(),
            host: log.host,
            severity: log.severity,
            facility: log.facility,
            syslog_tag: log.syslog_tag,
            source: log.source,
            message: log.message,
        }
    }
}
