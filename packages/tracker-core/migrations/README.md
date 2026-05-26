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

1. Pick a UTC timestamp prefix higher than every existing file and **strictly
   greater than `20250527093000`** (the last legacy migration; see
   [Upgrading from older versions](#upgrading-from-older-versions)). Use the
   pattern `YYYYMMDDhhmmss_short_description.sql`. You can either create the
   file by hand or, if you have [`sqlx-cli`][sqlx-cli] installed
   (`cargo install sqlx-cli`), run `sqlx migrate add <name>` inside the target
   backend folder — it only generates the empty file with the right timestamp
   and has no runtime role.
2. Create the file under **every** backend folder where the change applies, so
   the `_sqlx_migrations` history stays aligned across backends.
3. This project uses the simple, forward-only migration style. Do **not** add
   `.up.sql` / `.down.sql` pairs — `sqlx` does not allow mixing the two styles
   in the same folder.
4. Use SQL syntax supported by `sqlx`'s statement splitter — separate
   statements with `;` and use `--` for line comments (this applies to both
   the SQLite and MySQL backends; `#`-style comments are not accepted).
5. Run the test suite: `cargo test -p torrust-tracker-core`. A rebuild is
   required for the new migration to be embedded into the binary.

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
[sqlx-cli]: https://github.com/launchbadge/sqlx/tree/main/sqlx-cli
