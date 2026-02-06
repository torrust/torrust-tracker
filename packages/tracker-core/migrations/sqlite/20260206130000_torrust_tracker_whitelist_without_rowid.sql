-- Migrate whitelist table to whitelist_v1 with info_hash as PRIMARY KEY and WITHOUT ROWID optimization
CREATE TABLE IF NOT EXISTS whitelist_v1 (
    info_hash TEXT PRIMARY KEY NOT NULL
) WITHOUT ROWID;

INSERT INTO whitelist_v1 (info_hash) SELECT info_hash FROM whitelist;

DROP TABLE whitelist;
