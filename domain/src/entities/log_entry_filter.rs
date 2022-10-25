use serde::Deserialize;
use sqlx::{postgres::PgArguments, query::QueryAs, Arguments, Postgres, QueryBuilder};

use super::log_entry::LogEntry;

#[derive(Debug, Deserialize)]
pub struct LogEntryFilter {
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    pub host: Option<String>,
    pub severity: Option<String>,
    pub facility: Option<String>,
    pub syslog_tag: Option<String>,
    pub source: Option<String>,
}

pub struct LogEntryFilterQueryBuilder<'a> {
    filter: LogEntryFilter,
    builder: QueryBuilder<'a, Postgres>,
}

impl<'a> LogEntryFilterQueryBuilder<'a> {
    pub fn new(filter: LogEntryFilter) -> Self {
        Self {
            filter,
            builder: QueryBuilder::new("SELECT * FROM logs WHERE "),
        }
    }

    pub fn to_sql_query(&'a mut self) -> QueryAs<Postgres, LogEntry, PgArguments> {
        let mut to_bind = PgArguments::default();

        if let Some(t) = &self.filter.time {
            self.builder.push("timestamp = ").push_bind(t).push(" AND ");
            to_bind.add(t);
        }

        if let Some(h) = &self.filter.host {
            self.builder.push("host = ").push_bind(h).push(" AND ");
            to_bind.add(h);
        }

        if let Some(s) = &self.filter.severity {
            self.builder.push("severity = ").push_bind(s).push(" AND ");
            to_bind.add(s);
        }

        if let Some(f) = &self.filter.facility {
            self.builder.push("facility = ").push_bind(f).push(" AND ");
            to_bind.add(f);
        }

        if let Some(s) = &self.filter.syslog_tag {
            self.builder
                .push("syslog_tag = ")
                .push_bind(s)
                .push(" AND ");
            to_bind.add(s);
        }

        if let Some(s) = &self.filter.source {
            self.builder.push("source = ").push_bind(s);
            to_bind.add(s);
        }

        let sql = self.builder.sql();

        let sql = if sql.ends_with(" AND ") {
            &sql[0..sql.len() - 5]
        } else {
            sql
        };

        sqlx::query_as_with::<'a, Postgres, LogEntry, PgArguments>(sql, to_bind)
    }
}
