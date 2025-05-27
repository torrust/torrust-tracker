CREATE TABLE
    IF NOT EXISTS torrent_aggregate_metrics (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        metric_name TEXT NOT NULL UNIQUE,
        value INTEGER DEFAULT 0 NOT NULL
    );