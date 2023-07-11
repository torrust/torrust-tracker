#!/bin/bash

# Generate the default settings file if it does not exist
if ! [ -f "./config.toml" ]; then
    cp ./config.toml.local ./config.toml
fi

# Generate storage directory if it does not exist
mkdir -p "./storage/database"

# Generate the sqlite database if it does not exist
if ! [ -f "./storage/database/data.db" ]; then
    # todo: it should get the path from config.toml and only do it when we use sqlite
    touch ./storage/database/data.db
    echo ";" | sqlite3 ./storage/database/data.db
fi
