#!/bin/bash

# This script is only intended to be used for local development or testing environments.
# docs/benchmarking.md lists every benchmark and what it measures.

cargo bench --package torrust-tracker-torrent-repository-benchmarking

cargo bench --package torrust-tracker-http-core

cargo bench --package torrust-tracker-udp-core

cargo bench --package torrust-tracker-udp-server
