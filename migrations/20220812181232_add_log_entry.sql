-- Add migration script here
CREATE TABLE logs(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    timestamp timestamptz NOT NULL,
    host TEXT NOT NULL,
    severity TEXT NOT NULL,
    facility TEXT NOT NULL,
    syslog_tag TEXT NOT NULL,
    source TEXT NOT NULL,
    message TEXT NOT NULL
);