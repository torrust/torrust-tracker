# Docker

## Requirements

- Docker version 20.10.21

## Dev environment

Build and run locally:

```s
docker context use default
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
    --volume "$(pwd)/storage":"/app/storage" \
    josecelano/torrust-tracker
```

> NOTES:
>
> - You have to create the SQLite DB (`data.db`) and configuration (`config.toml`) before running the tracker. See `bin/install.sh`.
> - You have to replace the user UID (`1000`) with yours.
> - Remember to switch to your default docker context `docker context use default`.

## Prod environment

Deploy to Azure following [docker documentation](https://docs.docker.com/cloud/aci-integration/).

```s
docker context create aci myacicontext
docker context use myacicontext
docker volume create test-volume --storage-account torrustracker
docker run \
    --name torrust-tracker \
    --port 80:80 \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/josecelano/torrust-tracker:0.2.0
```

Detach from container logs when container starts. By default, the command line stays attached and follows container logs.

```s
docker run \
    --detach
    --port 80:80 \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/josecelano/torrust-tracker:0.2.0
```

> NOTES:
>
> - [There is no support for mounting a single file](https://docs.docker.com/cloud/aci-container-features/#persistent-volumes), or mounting a subfolder from an `Azure File Share`.
> - [ACI does not allow port mapping](https://docs.docker.com/cloud/aci-integration/#exposing-ports).
> - [Azure file share volume mount requires the Linux container run as root](https://learn.microsoft.com/en-us/azure/container-instances/container-instances-volume-azure-files#limitations).

## Links

- [Deploying Docker containers on Azure](https://docs.docker.com/cloud/aci-integration/).
- [Docker run options for ACI containers](https://docs.docker.com/cloud/aci-container-features/).
