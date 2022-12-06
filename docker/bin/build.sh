#!/bin/bash

TORRUST_TRACKER_USER_UID=${TORRUST_TRACKER_USER_UID:-1000}

echo "Building docker image ..."
echo "TORRUST_TRACKER_USER_UID: $TORRUST_TRACKER_USER_UID"

docker build \
    --build-arg UID="$TORRUST_TRACKER_USER_UID" \
    -t torrust-tracker .
