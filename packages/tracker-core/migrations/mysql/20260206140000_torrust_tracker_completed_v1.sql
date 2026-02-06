-- Migrate torrents table to completed_v1 with BINARY(20) info_hash as PRIMARY KEY
-- Rename 'completed' column to 'count'
-- BINARY(20) stores the raw 20-byte infohash instead of 40-char hex string

CREATE TABLE IF NOT EXISTS completed_v1 (
    info_hash BINARY(20) PRIMARY KEY NOT NULL,
    count INTEGER DEFAULT 1 NOT NULL CHECK (count >= 1)
);

-- Convert existing hex strings to binary
INSERT INTO completed_v1 (info_hash, count) SELECT UNHEX(info_hash), completed FROM torrents;

DROP TABLE torrents;
