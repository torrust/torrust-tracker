# Usage
__Notice:__ Skip the first step if you've downloaded the binaries directly. 

1. After building __Torrust Tracker__, navigate to the folder.
```bash
cd torrust-tracker/target
```

2. Create a file called `configuration.toml` with the following contents and change the [configuration](https://torrust.github.io/torrust-tracker/CONFIG.html) according to your liking:
```toml
mode = "public"
external_ip = "0.0.0.0" # set this to your external IP

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

4. Add your tracker announce url as follows:
`udp://localhost:6969` or `udp://{external_url}:6969`.
Make sure to port forward the udp port.
