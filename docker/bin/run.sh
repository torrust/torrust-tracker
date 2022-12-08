#!/bin/bash

TORRUST_TRACKER_USER_UID=${TORRUST_TRACKER_USER_UID:-1000}

docker run -it \
    --user="$TORRUST_TRACKER_USER_UID" \
    -p 6969:6969 -p 1212:1212 \
    --volume "$(pwd)/storage":"/app/storage" \
    torrust-tracker
