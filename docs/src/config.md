# Configuring UDPT
UDPT's configuration is a simple TOML file.

## Configuration
At the root level, the following options are configurable:
`mode` - Specifies which mode the tracker will operate in. Values can be `static`, `dynamic` or `private`.

### Root Level
- `mode` - Required. Possbile Values: `private`, `static` or `dynamic`.
- `log_level` - Default: `info`. Possible Values: `off`, `error`, `warning`, `info`, `debug`, `trace`.
- `db_path` - Database path. If not set, database will be volatile.
- `cleanup_interval` - Default: 600. Interval to run cleanup in seconds. Cleanup also saves the Database.

### `[udp]` section
This section must exist.

- `bind_address` - Required. This is where the UDP port will bind to. Example: `0.0.0.0:6969`.
- `announce_interval` - Required. Sets the `announce_interval` that will be sent to peers (in seconds).

### `[http]` section
This section is optional.

- `bind_address` - Required (if section exists). The HTTP REST API will be bound to this address. It's best not to expose this address publically. Example: `127.0.0.1:1234`.

### `[http.access_tokens]` section
Section is required if `[http]` section exists.

In this section you can make up keys that would be user ids, and values that would be their access token.
If this section is empty, the REST API will not be very useful.

## Sample Configuration
```toml
mode = "dynamic"
db_path = "database.json.bz2"

[udp]
announce_interval = 120         # Two minutes
bind_address = "0.0.0.0:1212"

[http]
bind_address = "127.0.0.1:1212"

[http.access_tokens]
someone = "MyAccessToken"
```
