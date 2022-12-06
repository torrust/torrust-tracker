# Docker

## Requirements

- Docker version 20.10.21

## Dev environment

Build and run locally:

```s
export TORRUST_TRACKER_USER_UID=1000
./docker/bin/build.sh $TORRUST_TRACKER_USER_UID
./bin/install.sh
./docker/bin/run.sh $TORRUST_TRACKER_USER_UID
```

Run using the pre-built public docker image:

```s
export TORRUST_TRACKER_USER_UID=1000
docker run -it \
    --user="$TORRUST_TRACKER_USER_UID" \
    -p 6969:6969 -p 1212:1212 \
    --mount type=bind,source="$(pwd)/data.db",target=/app/data.db \
    --mount type=bind,source="$(pwd)/config.toml",target=/app/config.toml \
    josecelano/torrust-tracker
```

> NOTES:
>
> - You have to create the SQLite DB (`data.db`) and configuration (`config.toml`) before running the tracker. See `bin/install.sh`.
> - You have to replace the user UID (`1000`) with yours.
