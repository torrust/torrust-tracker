#!/bin/bash

# This script is only intended to be used for local development or testing environments.

# Generate storage directory if it does not exist
mkdir -p ./benches

cargo bench --package torrust-tracker-torrent-repository 
mv ./target/criterion/report/index.html ./benches/torrust-tracker-torrent-repository.html
