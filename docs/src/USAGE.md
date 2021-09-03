# Usage
__Notice:__ Skip the first step if you've downloaded the binaries directly. 

1. After building __Torrust Tracker__, navigate to the folder.
```bash
cd torrust-tracker/target
```

2. Create a file called `configuration.toml`:
```toml
mode = "public"

[udp]
bind_address = "0.0.0.0:6969"
announce_interval = 120 # Two minutes

[http]
bind_address = "127.0.0.1:1212"

[http.access_tokens]
someone = "MyAccessToken"
```

3. And run __Torrust Tracker__:
```bash
./torrust-tracker -c configuration.toml
```
