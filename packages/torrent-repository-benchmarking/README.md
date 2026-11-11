# Torrust Tracker Torrent Repository Benchmarking

A library to run benchmarks for different implementations of a repository of torrents and their peers. It is not published to crates.io. Torrent repositories are used by the [Torrust Tracker](https://github.com/torrust/torrust-tracker).

## Benchmarking

```console
cargo bench -p torrust-tracker-torrent-repository-benchmarking
```

Example partial output:

```output
     Running benches/repository_benchmark.rs (target/release/deps/repository_benchmark-a9b0013c8d09c3c3)
add_one_torrent/RwLockStd
                        time:   [63.057 ns 63.242 ns 63.506 ns]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
add_one_torrent/RwLockStdMutexStd
                        time:   [62.505 ns 63.077 ns 63.817 ns]
```

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](https://github.com/torrust/torrust-tracker/blob/develop/LICENSE).
