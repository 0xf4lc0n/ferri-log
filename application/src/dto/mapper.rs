use super::disk_log_entry_dto::DiskLogEntryDto;
use domain::prelude::LogEntry;

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
