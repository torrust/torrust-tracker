#!/bin/bash

# This script is only intended to be used for local development or testing environments.

# Generate storage directory if it does not exist
mkdir -p ./storage/tracker/lib/database

# Generate the sqlite database if it does not exist
if ! [ -f "./storage/tracker/lib/database/sqlite3.db" ]; then
    sqlite3 ./storage/tracker/lib/database/sqlite3.db "VACUUM;"
fi
