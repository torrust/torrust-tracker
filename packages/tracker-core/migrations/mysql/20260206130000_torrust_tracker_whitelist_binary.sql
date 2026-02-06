-- Migrate whitelist table to whitelist_v1 with BINARY(20) PRIMARY KEY for efficient storage
-- BINARY(20) stores the raw 20-byte infohash instead of 40-char hex string

CREATE TABLE IF NOT EXISTS whitelist_v1 (
    info_hash BINARY(20) PRIMARY KEY NOT NULL
);

-- Convert existing hex strings to binary
INSERT INTO whitelist_v1 (info_hash) SELECT UNHEX(info_hash) FROM whitelist;

DROP TABLE whitelist;
