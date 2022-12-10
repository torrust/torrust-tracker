#!/bin/bash

TORRUST_TRACKER_USER_UID=${TORRUST_TRACKER_USER_UID:-1000}

docker run -it \
    --user="$TORRUST_TRACKER_USER_UID" \
    --publish 6969:6969/udp \
    --publish 1212:1212 \
    --volume "$(pwd)/storage":"/app/storage" \
    torrust-tracker
