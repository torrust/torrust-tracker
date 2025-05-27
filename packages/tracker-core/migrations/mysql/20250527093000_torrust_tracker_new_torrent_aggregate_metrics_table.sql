CREATE TABLE
    IF NOT EXISTS torrent_aggregate_metrics (
        id integer PRIMARY KEY AUTO_INCREMENT,
        metric_name VARCHAR(50) NOT NULL UNIQUE,
        value INTEGER DEFAULT 0 NOT NULL
    );