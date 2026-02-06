# Database Migrations

We don't support automatic migrations yet. The tracker creates all the needed tables when it starts. The SQL sentences are hardcoded in each database driver.

The migrations in this folder were introduced to add some new changes (permanent keys) and to allow users to migrate to the new version. In the future, we will remove the hardcoded SQL and start using a Rust crate for database migrations. For the time being, if you are using the initial schema described in the migration `20240730183000_torrust_tracker_create_all_tables.sql` you will need to run all the subsequent migrations manually.

## Database Tables

The tracker uses 4 tables:

### 1. `whitelist_v1`

Stores whitelisted torrent infohashes for private/whitelisted mode.

| Column | SQLite Type | MySQL Type | Description |
|--------|-------------|------------|-------------|
| `info_hash` | TEXT PRIMARY KEY NOT NULL | BINARY(20) PRIMARY KEY NOT NULL | BitTorrent V1 infohash (SQLite: 40-char hex string, MySQL: 20-byte binary) |

> **Note**: SQLite uses `WITHOUT ROWID` optimization. MySQL stores as binary for efficiency.

### 2. `completed_v1`

Stores per-torrent download completion counts.

| Column | SQLite Type | MySQL Type | Description |
|--------|-------------|------------|-------------|
| `info_hash` | TEXT PRIMARY KEY NOT NULL | BINARY(20) PRIMARY KEY NOT NULL | BitTorrent V1 infohash (SQLite: 40-char hex string, MySQL: 20-byte binary) |
| `count` | INTEGER DEFAULT 1 NOT NULL CHECK (count >= 1) | INTEGER DEFAULT 1 NOT NULL CHECK (count >= 1) | Number of times the torrent has been fully downloaded (minimum 1) |

> **Note**: SQLite uses `WITHOUT ROWID` optimization. MySQL stores info_hash as binary for efficiency.

### 3. `keys`

Stores authentication keys for private trackers.

| Column | SQLite Type | MySQL Type | Description |
|--------|-------------|------------|-------------|
| `id` | INTEGER PRIMARY KEY AUTOINCREMENT | INT NOT NULL AUTO_INCREMENT | Auto-increment ID |
| `key` | TEXT NOT NULL UNIQUE | VARCHAR(32) NOT NULL UNIQUE | Authentication token (32-char alphanumeric string) |
| `valid_until` | INTEGER (nullable) | INT(10) (nullable) | Unix timestamp for key expiration; NULL means permanent key |

### 4. `torrent_aggregate_metrics`

Stores global/aggregate metrics not tied to specific torrents (e.g., total downloads across all torrents).

| Column | SQLite Type | MySQL Type | Description |
|--------|-------------|------------|-------------|
| `id` | INTEGER PRIMARY KEY AUTOINCREMENT | integer PRIMARY KEY AUTO_INCREMENT | Auto-increment ID |
| `metric_name` | TEXT NOT NULL UNIQUE | VARCHAR(50) NOT NULL UNIQUE | Unique metric identifier (e.g., `torrents_downloads_total`) |
| `value` | INTEGER DEFAULT 0 NOT NULL | INTEGER DEFAULT 0 NOT NULL | The metric value |

## Migration Files

### SQLite

| Migration | Description |
|-----------|-------------|
| `20240730183000_torrust_tracker_create_all_tables.sql` | Creates initial tables: `whitelist`, `torrents`, `keys` |
| `20240730183500_torrust_tracker_keys_valid_until_nullable.sql` | Makes `valid_until` column nullable in `keys` table (for permanent keys) |
| `20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql` | Creates `torrent_aggregate_metrics` table for global metrics |
| `20260206120000_torrust_tracker_torrents_completed_non_zero.sql` | Removes rows with completed=0 and adds CHECK constraint (completed >= 1) |
| `20260206130000_torrust_tracker_whitelist_without_rowid.sql` | Optimizes whitelist table: removes `id` column, uses `info_hash` as PRIMARY KEY with WITHOUT ROWID |
| `20260206140000_torrust_tracker_completed_v1.sql` | Migrates `torrents` to `completed_v1`: removes `id` column, renames `completed` to `count`, uses `info_hash` as PRIMARY KEY with WITHOUT ROWID |

### MySQL

| Migration | Description |
|-----------|-------------|
| `20240730183000_torrust_tracker_create_all_tables.sql` | Creates initial tables: `whitelist`, `torrents`, `keys` |
| `20240730183500_torrust_tracker_keys_valid_until_nullable.sql` | Makes `valid_until` column nullable in `keys` table (for permanent keys) |
| `20250527093000_torrust_tracker_new_torrent_aggregate_metrics_table.sql` | Creates `torrent_aggregate_metrics` table for global metrics |
| `20260206120000_torrust_tracker_torrents_completed_non_zero.sql` | Removes rows with completed=0 and adds CHECK constraint (completed >= 1) |
| `20260206130000_torrust_tracker_whitelist_binary.sql` | Optimizes whitelist table: removes `id` column, uses BINARY(20) `info_hash` as PRIMARY KEY |
| `20260206140000_torrust_tracker_completed_v1.sql` | Migrates `torrents` to `completed_v1`: removes `id` column, renames `completed` to `count`, uses BINARY(20) `info_hash` as PRIMARY KEY |
