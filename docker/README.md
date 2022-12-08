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
    -p 6969:6969/udp -p 1212:1212 \
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

You have to create the ACI context and the storage:

```s
docker context create aci myacicontext
docker context use myacicontext
docker volume create test-volume --storage-account torrustracker
```

You need to create all the files needed by the application in the storage dir:

- `storage/config/config.toml`
- `storage/database`

And finally, you can run the container:

```s
docker run \
    --publish 6969:6969/udp \
    --publish 1212:1212 \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/josecelano/torrust-tracker:0.5.0
```

Detach from container logs when the container starts. By default, the command line stays attached and follows container logs.

```s
docker run \
    --detach
    --publish 6969:6969/udp \
    --publish 1212:1212 \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/josecelano/torrust-tracker:0.5.0
```

You should see something like this:

```s
$ docker run \ \
    --publish 6969:6969/udp \
    --publish 1212:1212 \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/josecelano/torrust-tracker:0.5.0
[+] Running 2/2
 ⠿ Group intelligent-hawking  Created                                                                                                                                                                    5.0s
 ⠿ intelligent-hawking        Created                                                                                                                                                                   41.7s
2022-12-08T18:39:19.697869300+00:00 [torrust_tracker::logging][INFO] logging initialized.
2022-12-08T18:39:19.712651100+00:00 [torrust_tracker::jobs::udp_tracker][INFO] Starting UDP server on: 0.0.0.0:6969
2022-12-08T18:39:19.712792700+00:00 [torrust_tracker::jobs::tracker_api][INFO] Starting Torrust API server on: 0.0.0.0:1212
2022-12-08T18:39:19.725124+00:00 [torrust_tracker::jobs::tracker_api][INFO] Torrust API server started
```

You can see the container with:

```s
$ docker ps
CONTAINER ID          IMAGE                                                      COMMAND             STATUS              PORTS
intelligent-hawking   registry.hub.docker.com/josecelano/torrust-tracker:0.5.0                       Running             4.236.213.57:6969->6969/udp, 4.236.213.57:1212->1212/tcp
```

After a while, you can use the tracker API `http://4.236.213.57:1212/api/stats?token=MyAccessToken` and the UDP tracker with your BitTorrent client using this tracker announce URL `udp://4.236.213.57:6969/announce`.

> NOTES:
>
> - [There is no support for mounting a single file](https://docs.docker.com/cloud/aci-container-features/#persistent-volumes), or mounting a subfolder from an `Azure File Share`.
> - [ACI does not allow port mapping](https://docs.docker.com/cloud/aci-integration/#exposing-ports).
> - [Azure file share volume mount requires the Linux container run as root](https://learn.microsoft.com/en-us/azure/container-instances/container-instances-volume-azure-files#limitations).
> - It can take some minutes until the public IP for the ACI container is available.
> - You can use the Azure web UI to download files from the storage. For example, the SQLite database.
> - [It seems you can only expose web interfaces on port 80 on Azure Container Instances](https://stackoverflow.com/a/56768087/3012842). Not official documentation!

## Links

- [Deploying Docker containers on Azure](https://docs.docker.com/cloud/aci-integration/).
- [Docker run options for ACI containers](https://docs.docker.com/cloud/aci-container-features/).
- [Quickstart: Deploy a container instance in Azure using the Docker CLI](https://learn.microsoft.com/en-us/azure/container-instances/quickstart-docker-cli).
