#!/bin/bash

# This script is only intended to be used for local development or testing environments.

cargo bench --package torrust-tracker-torrent-repository

cargo bench --package bittorrent-http-tracker-core

cargo bench --package bittorrent-udp-tracker-core
