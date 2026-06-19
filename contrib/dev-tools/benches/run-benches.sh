#!/bin/bash

# This script is only intended to be used for local development or testing environments.

cargo bench --package torrust-tracker-torrent-repository

cargo bench --package torrust-tracker-http-core

cargo bench --package torrust-tracker-udp-core
