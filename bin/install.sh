#!/bin/bash

# Generate the default settings file if it does not exist
if ! [ -f "./config.toml" ]; then
    cp ./config.toml.local ./config.toml
fi

# Generate the sqlite database if it does not exist
if ! [ -f "./data.db" ]; then
    # todo: it should get the path from config.toml and only do it when we use sqlite
    touch ./data.db
    echo ";" | sqlite3 ./data.db
fi

