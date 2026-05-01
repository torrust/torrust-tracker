ALTER TABLE torrents
ALTER COLUMN completed TYPE BIGINT;

ALTER TABLE torrent_aggregate_metrics
ALTER COLUMN value TYPE BIGINT;