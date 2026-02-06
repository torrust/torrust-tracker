-- Remove any rows with completed = 0 (should not exist in normal operation)
DELETE FROM torrents WHERE completed = 0;

-- SQLite doesn't support adding CHECK constraints to existing tables directly.
-- We need to recreate the table with the new constraint.
CREATE TABLE IF NOT EXISTS torrents_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    info_hash TEXT NOT NULL UNIQUE,
    completed INTEGER DEFAULT 1 NOT NULL CHECK (completed >= 1)
);

INSERT INTO torrents_new (id, info_hash, completed) SELECT id, info_hash, completed FROM torrents;

DROP TABLE torrents;

ALTER TABLE torrents_new RENAME TO torrents;
