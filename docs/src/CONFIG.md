# Configuring Torrust Tracker
Torrust Tracker's configuration is a simple TOML file. If no TOML file is found, it will run a default configuration.

## Configuration

### Root Level
- `REQUIRED` `mode`: Possible Values: `public`, `listed`, `private` or `private_listed`.
- `REQUIRED` `external_ip`: Set this to your external IP, like `"99.123.43.128"`,
- `OPTIONAL` `log_level`: Possible Values: `off`, `error`, `warning`, `info`, `debug`, `trace`.
- `OPTIONAL` `cleanup_interval`: Interval to clean inactive peers `in seconds`.

### `REQUIRED` `[udp]` Section
- `REQUIRED` `bind_address`: This is where the UDP port will bind to. Example: `0.0.0.0:6969`.
- `REQUIRED` `announce_interval`: Sets the announce interval that will be sent to peers `in seconds`.

### `OPTIONAL` `[http]` Section
- `REQUIRED` `bind_address`: The HTTP REST API will be bound to this address. It's best not to expose this address publicly. Example: `127.0.0.1:80`.

### `REQUIRED IF [http] EXISTS` `[http.access_tokens]` Section
In this section you can make up keys that would be user ids, and values that would be their access token.
If this section is empty, the REST API will not be very useful.

## Sample Configuration
```toml
mode = "public"
external_ip = "0.0.0.0" # set this to your external IP

[udp]
bind_address = "0.0.0.0:6969"
announce_interval = 120 # Two minutes

[http]
bind_address = "127.0.0.1:80"

[http.access_tokens]
someone = "MyAccessToken"
```
