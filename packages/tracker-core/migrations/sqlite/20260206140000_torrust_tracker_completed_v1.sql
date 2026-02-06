-- Migrate torrents table to completed_v1 with info_hash as PRIMARY KEY and WITHOUT ROWID optimization
-- Rename 'completed' column to 'count'
CREATE TABLE IF NOT EXISTS completed_v1 (
    info_hash TEXT PRIMARY KEY NOT NULL,
    count INTEGER DEFAULT 1 NOT NULL CHECK (count >= 1)
) WITHOUT ROWID;

INSERT INTO completed_v1 (info_hash, count) SELECT info_hash, completed FROM torrents;

DROP TABLE torrents;
