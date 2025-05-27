CREATE TABLE
    IF NOT EXISTS whitelist (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        info_hash TEXT NOT NULL UNIQUE
    );

# todo: rename to `torrent_metrics`
CREATE TABLE
    IF NOT EXISTS torrents (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        info_hash TEXT NOT NULL UNIQUE,
        completed INTEGER DEFAULT 0 NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS keys (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        key TEXT NOT NULL UNIQUE,
        valid_until INTEGER NOT NULL
    );