# Database Migrations

The tracker uses SQL migrations as the schema source of truth for all SQL
backends:

- `mysql/`
- `postgresql/`
- `sqlite/`

The tracker applies these migrations automatically when a database-backed store
is first used.

The files intentionally remain split per backend because SQL syntax and column
types differ across engines. Migration ordering is shared by timestamp prefix so
the schema evolution remains aligned across backends.
