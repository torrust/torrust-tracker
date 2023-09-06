#!/bin/bash

mkdir -p ./storage/tracker/lib/ ./storage/tracker/log/ ./storage/tracker/etc/

docker run -it \
    --env USER_ID"$(id -u)" \
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \
    --volume ./storage/tracker/lib:/var/lib/torrust/tracker:rw \
    --volume ./storage/tracker/log:/var/log/torrust/tracker:rw \
    --volume ./storage/tracker/etc:/etc/torrust/tracker:rw \
    torrust-tracker:release
