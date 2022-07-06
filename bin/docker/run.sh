#!/bin/bash

docker run --rm -it \
    -p 6969:6969 -p 1212:1212 \
    --volume "$(pwd)/storage":"/app/storage" \
    --mount type=bind,source="$(pwd)/config.toml",target=/app/config.toml \
    torrust-tracker
