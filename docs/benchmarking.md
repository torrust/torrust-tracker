# Benchmarking

We have two types of benchmarking:

- E2E benchmarking running the service (HTTP or UDP tracker).
- Internal torrents repository benchmarking.

## E2E benchmarking

We are using the scripts provided by [aquatic](https://github.com/greatest-ape/aquatic).

Installing both commands:

```console
cargo install aquatic_udp_load_test
cargo install aquatic_http_load_test
```

### Run UDP load test

Run the tracker with UDP service enabled on port 3000 and set log level to `error`.

```toml
log_level = "error"

[[udp_trackers]]
bind_address = "0.0.0.0:3000"
enabled = true
```

Run the load test with:

```console
aquatic_udp_load_test
```

Output:

```output
Starting client with config: Config {
    server_address: 127.0.0.1:3000,
    log_level: Error,
    workers: 1,
    duration: 0,
    network: NetworkConfig {
        multiple_client_ipv4s: true,
        first_port: 45000,
        poll_timeout: 276,
        poll_event_capacity: 2877,
        recv_buffer: 6000000,
    },
    requests: RequestConfig {
        number_of_torrents: 10000,
        scrape_max_torrents: 50,
        weight_connect: 0,
        weight_announce: 100,
        weight_scrape: 1,
        torrent_gamma_shape: 0.2,
        torrent_gamma_scale: 100.0,
        peer_seeder_probability: 0.25,
        additional_request_probability: 0.5,
    },
}

Requests out: 32632.43/second
Responses in: 24239.33/second
  - Connect responses:  7896.91
  - Announce responses: 16327.01
  - Scrape responses:   15.40
  - Error responses:    0.00
Peers per announce response: 33.10
```

### Run HTTP load test

Run the tracker with UDP service enabled on port 3000 and set log level to `error`.

```toml
[[udp_trackers]]
bind_address = "0.0.0.0:3000"
enabled = true
```

Run the load test with:

```console
aquatic_http_load_test
```

Output:

```output
Starting client with config: Config {
    server_address: 127.0.0.1:3000,
    log_level: Error,
    num_workers: 1,
    num_connections: 128,
    connection_creation_interval_ms: 10,
    url_suffix: "",
    duration: 0,
    keep_alive: true,
    torrents: TorrentConfig {
        number_of_torrents: 10000,
        peer_seeder_probability: 0.25,
        weight_announce: 5,
        weight_scrape: 0,
        torrent_gamma_shape: 0.2,
        torrent_gamma_scale: 100.0,
    },
    cpu_pinning: CpuPinningConfigDesc {
        active: false,
        direction: Descending,
        hyperthread: System,
        core_offset: 0,
    },
}
```

### Comparing UDP tracker with other Rust implementations

#### Torrust UDP Tracker

Running the tracker:

```console
git@github.com:torrust/torrust-tracker.git
cd torrust-tracker
cargo build --release
TORRUST_TRACKER_PATH_CONFIG="./share/default/config/tracker.udp.benchmarking.toml" ./target/release/torrust-tracker
```

Running the test: `aquatic_udp_load_test`.

```output
Requests out: 13075.56/second
Responses in: 12058.38/second
  - Connect responses:  1017.18
  - Announce responses: 11035.00
  - Scrape responses:   6.20
  - Error responses:    0.00
Peers per announce response: 41.13
```

#### Aquatic UDP Tracker

Running the tracker:

```console
git clone git@github.com:greatest-ape/aquatic.git
cd aquatic
cargo build --release -p aquatic_udp
./target/release/aquatic_udp -c "aquatic-udp-config.toml"
./target/release/aquatic_udp -c "aquatic-udp-config.toml"
```

Running the test: `aquatic_udp_load_test`.

```output
Requests out: 383873.14/second
Responses in: 383440.35/second
  - Connect responses:  429.19
  - Announce responses: 379249.22
  - Scrape responses:   3761.93
  - Error responses:    0.00
Peers per announce response: 15.33
```

#### Torrust-Actix UDP Tracker

Run the tracker with UDP service enabled on port 3000 and set log level to `error`.

```toml
[[udp_trackers]]
bind_address = "0.0.0.0:3000"
enabled = true
```

```console
git clone https://github.com/Power2All/torrust-actix.git
cd torrust-actix
cargo build --release
./target/release/torrust-actix --create-config
./target/release/torrust-actix
```

Running the test: `aquatic_udp_load_test`.

```output
Requests out: 3072.94/second
Responses in: 2395.15/second
  - Connect responses:  556.79
  - Announce responses: 1821.16
  - Scrape responses:   17.20
  - Error responses:    0.00
Peers per announce response: 133.88
```

### Results

Announce request per second:

| Tracker       |  Announce |
|---------------|-----------|
| Aquatic       |   379,249 |
| Torrust       |    11,035 |
| Torrust-Actix |     1,821 |

## Repository benchmarking

You can run it with:

```console
cargo run --release -p torrust-torrent-repository-benchmarks -- --threads 4 --sleep 0 --compare true
```

It tests the different implementation for the internal torrent storage.

```output
tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>
add_one_torrent: Avg/AdjAvg: (60ns, 59ns)
update_one_torrent_in_parallel: Avg/AdjAvg: (10.909457ms, 0ns)
add_multiple_torrents_in_parallel: Avg/AdjAvg: (13.88879ms, 0ns)
update_multiple_torrents_in_parallel: Avg/AdjAvg: (7.772484ms, 7.782535ms)

std::sync::RwLock<std::collections::BTreeMap<InfoHash, Entry>>
add_one_torrent: Avg/AdjAvg: (43ns, 39ns)
update_one_torrent_in_parallel: Avg/AdjAvg: (4.020937ms, 4.020937ms)
add_multiple_torrents_in_parallel: Avg/AdjAvg: (5.896177ms, 5.768448ms)
update_multiple_torrents_in_parallel: Avg/AdjAvg: (3.883823ms, 3.883823ms)

std::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>
add_one_torrent: Avg/AdjAvg: (51ns, 49ns)
update_one_torrent_in_parallel: Avg/AdjAvg: (3.252314ms, 3.149109ms)
add_multiple_torrents_in_parallel: Avg/AdjAvg: (8.411094ms, 8.411094ms)
update_multiple_torrents_in_parallel: Avg/AdjAvg: (4.106086ms, 4.106086ms)

tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<std::sync::Mutex<Entry>>>>
add_one_torrent: Avg/AdjAvg: (91ns, 90ns)
update_one_torrent_in_parallel: Avg/AdjAvg: (3.542378ms, 3.435695ms)
add_multiple_torrents_in_parallel: Avg/AdjAvg: (15.651172ms, 15.651172ms)
update_multiple_torrents_in_parallel: Avg/AdjAvg: (4.368189ms, 4.257572ms)

tokio::sync::RwLock<std::collections::BTreeMap<InfoHash, Arc<tokio::sync::Mutex<Entry>>>>
add_one_torrent: Avg/AdjAvg: (111ns, 109ns)
update_one_torrent_in_parallel: Avg/AdjAvg: (6.590677ms, 6.808535ms)
add_multiple_torrents_in_parallel: Avg/AdjAvg: (16.572217ms, 16.30488ms)
update_multiple_torrents_in_parallel: Avg/AdjAvg: (4.073221ms, 4.000122ms)
```

## Other considerations

We are testing new repository implementations that allow concurrent writes. See <https://github.com/torrust/torrust-tracker/issues/565>.
