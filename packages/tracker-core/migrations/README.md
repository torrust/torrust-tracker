# Database Migrations

The tracker applies schema migrations automatically on startup using
[`sqlx::migrate!`][sqlx-migrate]. Each backend has its own migration folder:

- `migrations/sqlite/` — applied to SQLite databases
- `migrations/mysql/` — applied to MySQL databases

Migration files are embedded into the binary at compile time and applied in
timestamp order. The `_sqlx_migrations` table (created automatically on the
target database) records which migrations have already run, so each migration
is applied exactly once per database.

## Adding a new migration

1. Pick a UTC timestamp prefix higher than every existing file
   (`YYYYMMDDhhmmss_short_description.sql`).
2. Create the file under **every** backend folder where the change applies, so
   the `_sqlx_migrations` history stays aligned across backends.
3. Use SQL syntax supported by `sqlx`'s simple statement splitter — separate
   statements with `;` and use `--` for line comments. The SQLite parser does
   not accept `#`-style comments.
4. Run the test suite: `cargo test -p bittorrent-tracker-core`.

## Migration file immutability

Once a migration file has been deployed it must never be modified. `sqlx`
records each migration's checksum in `_sqlx_migrations`; editing a committed
migration file causes a checksum-mismatch error on the next startup for any
database that has already applied that migration. To fix or extend an existing
schema, add a new migration with a later timestamp.

## Upgrading from older versions

Users of pre-v4 trackers must have applied all three legacy migrations
(`20240730183000_*`, `20240730183500_*`, and `20250527093000_*`) before
upgrading. The legacy bootstrap path of `create_database_tables()` detects
existing schemas without a `_sqlx_migrations` table and seeds the migration
history so the embedded migrator skips them on subsequent runs.

[sqlx-migrate]: https://docs.rs/sqlx/latest/sqlx/macro.migrate.html
