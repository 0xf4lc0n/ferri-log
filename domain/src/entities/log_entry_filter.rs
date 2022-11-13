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

struct Filter<'a> {
    time: Option<FilterField<'a>>,
    host: Option<FilterField<'a>>,
    severity: Option<FilterField<'a>>,
    facility: Option<FilterField<'a>>,
    syslog_tag: Option<FilterField<'a>>,
    source: Option<FilterField<'a>>,
}

impl<'a> Filter<'a> {
    fn new(filter: LogEntryFilter) -> Self {
        Self {
            time: filter.time.map(|t| FilterField {
                name: "time",
                value: t.to_rfc3339(),
            }),
            host: filter.host.map(|h| FilterField {
                name: "host",
                value: h,
            }),
            severity: filter.severity.map(|s| FilterField {
                name: "severity",
                value: s,
            }),
            facility: filter.facility.map(|f| FilterField {
                name: "facility",
                value: f,
            }),
            syslog_tag: filter.syslog_tag.map(|s| FilterField {
                name: "syslog_tag",
                value: s,
            }),
            source: filter.source.map(|s| FilterField {
                name: "source",
                value: s,
            }),
        }
    }

    fn into_vec(self) -> Vec<Option<FilterField<'a>>> {
        vec![
            self.time,
            self.host,
            self.severity,
            self.facility,
            self.syslog_tag,
            self.source,
        ]
    }
}

#[derive(Debug, Deserialize, Clone)]
struct FilterField<'a> {
    name: &'a str,
    value: String,
}

pub struct LogEntryFilterQueryBuilder<'a> {
    builder: QueryBuilder<'a, Postgres>,
    filter: Vec<Option<FilterField<'a>>>,
}

impl<'a> LogEntryFilterQueryBuilder<'a> {
    pub fn new(filter: LogEntryFilter) -> Self {
        Self {
            builder: QueryBuilder::new("SELECT * FROM logs WHERE "),
            filter: Filter::new(filter).into_vec(),
        }
    }

    pub fn to_sql_query(&'a mut self) -> QueryAs<Postgres, LogEntry, PgArguments> {
        let mut to_bind = PgArguments::default();

        for field in self.filter.iter().flatten() {
            self.builder
                .push(&field.name)
                .push(" = ")
                .push_bind(&field.value)
                .push(" AND ");
            to_bind.add(&field.value);
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
