#!/bin/bash

# Single command to setup and run the tracker using the pre-built image.

# Check if 'storage' directory exists
if [ -d "./storage" ]; then
    echo "Warning: 'storage' directory already exists. Please remove or rename it before proceeding."
    exit 1
fi

# Check if 'config.toml' file exists in the current directory
if [ -f "./config.toml" ]; then
    echo "Warning: 'config.toml' file already exists in the root directory. Please remove or rename it before proceeding."
    exit 1
fi

# Check if SQLite3 is installed
if ! command -v sqlite3 &> /dev/null; then
    echo "Warning: SQLite3 is not installed on your system. Please install it and retry."
    exit 1
fi

wget https://raw.githubusercontent.com/torrust/torrust-tracker/v3.0.0-alpha.3/config.toml.local -O config.toml \
  && mkdir -p ./storage/database \
  && mkdir -p ./storage/ssl_certificates \
  && touch ./storage/database/data.db \
  && echo ";" | sqlite3 ./storage/database/data.db
