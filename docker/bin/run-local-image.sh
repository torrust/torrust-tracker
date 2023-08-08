#!/bin/bash

TORRUST_TRACKER_USER_UID=${TORRUST_TRACKER_USER_UID:-1000}
TORRUST_TRACKER_CONFIG=$(cat config.toml)

docker run -it \
    --user="$TORRUST_TRACKER_USER_UID" \
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \
    --env TORRUST_TRACKER_CONFIG="$TORRUST_TRACKER_CONFIG" \
    --volume "$(pwd)/storage":"/app/storage" \
    torrust-tracker
