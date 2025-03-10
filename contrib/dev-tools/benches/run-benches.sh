#!/bin/bash

# This script is only intended to be used for local development or testing environments.

# Generate benches directory if it does not exist
mkdir -p ./benches

cargo bench --package torrust-tracker-torrent-repository 
mv -b ./target/criterion/add_one_torrent/report/ ./benches/torrust-tracker-torrent-repository/add_one_torrent
mv -b ./target/criterion/add_multiple_torrents_in_parallel/report/ ./benches/torrust-tracker-torrent-repository/add_multiple_torrents_in_parallel
mv -b ./target/criterion/update_multiple_torrents_in_parallel/report/ ./benches/torrust-tracker-torrent-repository/update_multiple_torrents_in_parallel
mv -b ./target/criterion/update_one_torrent_in_parallel/report/ ./benches/torrust-tracker-torrent-repository/update_one_torrent_in_parallel
cargo bench --package bittorrent-http-tracker-core 
mv -b target/criterion/http_tracker_handle_announce_once/ ./benches/http_tracker_handle_announce_once
cargo bench --package bittorrent-udp-tracker-core 
mv -b target/criterion/udp_tracker_connect_once/ ./benches/udp_tracker_connect_once
