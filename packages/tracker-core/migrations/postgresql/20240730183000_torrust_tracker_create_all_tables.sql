CREATE TABLE
    IF NOT EXISTS whitelist (
        id SERIAL PRIMARY KEY,
        info_hash VARCHAR(40) NOT NULL UNIQUE
    );

-- todo: rename to `torrent_metrics`
CREATE TABLE
    IF NOT EXISTS torrents (
        id SERIAL PRIMARY KEY,
        info_hash VARCHAR(40) NOT NULL UNIQUE,
        completed INTEGER DEFAULT 0 NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS keys (
        id SERIAL PRIMARY KEY,
        key VARCHAR(32) NOT NULL UNIQUE,
        valid_until INTEGER NOT NULL
    );