CREATE TABLE Blacklist(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    facility TEXT NOT NULL,
    source TEXT NOT NULL,
    message TEXT NOT NULL
);