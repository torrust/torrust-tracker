#!/bin/bash

TORRUST_TRACKER_USER_UID=${TORRUST_TRACKER_USER_UID:-1000}

docker run -it \
    --user="$TORRUST_TRACKER_USER_UID" \
    -p 6969:6969 -p 1212:1212 \
    --mount type=bind,source="$(pwd)/data.db",target=/app/data.db \
    --mount type=bind,source="$(pwd)/config.toml",target=/app/config.toml \
    torrust-tracker
