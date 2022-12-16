# Docker

## Requirements

- Docker version 20.10.21
- You need to create the `storage` directory with this structure and files:

```s
$ tree storage/
storage/
├── database
│   └── data.db
└── ssl_certificates
    ├── localhost.crt
    └── localhost.key
```

> NOTE: you only need the `ssl_certificates` directory and certificates in case you have enabled SSL for the one HTTP tracker or the API.

## Dev environment

### With docker

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
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \
    --volume "$(pwd)/storage":"/app/storage" \
    torrust/torrust-tracker
```

> NOTES:
>
> - You have to create the SQLite DB (`data.db`) and configuration (`config.toml`) before running the tracker. See `bin/install.sh`.
> - You have to replace the user UID (`1000`) with yours.
> - Remember to switch to your default docker context `docker context use default`.

### With docker-compose

The docker-compose configuration includes the MySQL service configuration. If you want to use MySQL instead of SQLite you have to change your `config.toml` configuration:

```toml
db_driver = "MySQL"
db_path = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker"
```

If you want to inject an environment variable into docker-compose you can use the file `.env`. There is a template `.env.local`.

Build and run it locally:

```s
docker compose up --build
```

After running the "up" command you will have two running containers:

```s
$ docker ps
CONTAINER ID   IMAGE             COMMAND                  CREATED          STATUS                   PORTS                                                                                                                             NAMES
06feacb91a9e   torrust-tracker   "cargo run"              18 minutes ago   Up 4 seconds             0.0.0.0:1212->1212/tcp, :::1212->1212/tcp, 0.0.0.0:7070->7070/tcp, :::7070->7070/tcp, 0.0.0.0:6969->6969/udp, :::6969->6969/udp   torrust-tracker-1
34d29e792ee2   mysql:8.0         "docker-entrypoint.s…"   18 minutes ago   Up 5 seconds (healthy)   0.0.0.0:3306->3306/tcp, :::3306->3306/tcp, 33060/tcp                                                                              torrust-mysql-1
```

And you should be able to use the application, for example making a request to the API:

<https://127.0.0.1:1212/api/stats?token=MyAccessToken>

You can stop the containers with:

```s
docker compose down
```

Additionally, you can delete all resources (containers, volumes, networks) with:

```s
docker compose down -v
```

### Access Mysql with docker

These are some useful commands for MySQL.

Open a shell in the MySQL container using docker or docker-compose.

```s
docker exec -it torrust-mysql-1 /bin/bash 
docker compose exec mysql /bin/bash
```

Connect to MySQL from inside the MySQL container or from the host:

```s
mysql -h127.0.0.1 -uroot -proot_secret_password
```

The when MySQL container is started the first time, it creates the database, user, and permissions needed.
If you see the error "Host is not allowed to connect to this MySQL server" you can check that users have the right permissions in the database. Make sure the user `root` and `db_user` can connect from any host (`%`).

```s
mysql> SELECT host, user FROM mysql.user;
+-----------+------------------+
| host      | user             |
+-----------+------------------+
| %         | db_user          |
| %         | root             |
| localhost | mysql.infoschema |
| localhost | mysql.session    |
| localhost | mysql.sys        |
| localhost | root             |
+-----------+------------------+
6 rows in set (0.00 sec)
```

If the database, user or permissions are not created the reason could be the MySQL container volume can be corrupted. Delete it and start again the containers.

### SSL Certificates

You can use a certificate for localhost. You can create your [localhost certificate](https://letsencrypt.org/docs/certificates-for-localhost/#making-and-trusting-your-own-certificates) and use it in the `storage` folder and the configuration file (`config.toml`). For example:

The storage folder must contain your certificates:

```s
$ tree storage/
storage/
├── database
│   └── data.db
└── ssl_certificates
    ├── localhost.crt
    └── localhost.key
```

You have not enabled it in your `config.toml` file:

```toml
...
[[http_trackers]]
enabled = true
bind_address = "0.0.0.0:7070"
ssl_enabled = true
ssl_cert_path = "./storage/ssl_certificates/localhost.crt"
ssl_key_path = "./storage/ssl_certificates/localhost.key"

[http_api]
enabled = true
bind_address = "0.0.0.0:1212"
ssl_enabled = true
ssl_cert_path = "./storage/ssl_certificates/localhost.crt"
ssl_key_path = "./storage/ssl_certificates/localhost.key"
...
```

> NOTE: you can enable it independently for each HTTP tracker or the API.

If you enable the SSL certificate for the API, for example, you can load the API with this URL:

<https://localhost:1212/api/stats?token=MyAccessToken>

## Prod environment

In this section, you will learn how to deploy the tracker to a single docker container in Azure Container Instances.

> NOTE: Azure Container Instances is a solution when you want to run an isolated container. If you need full container orchestration, including service discovery across multiple containers, automatic scaling, and coordinated application upgrades, we recommend [Kubernetes](https://kubernetes.io/).

Deploy to Azure Container Instance following [docker documentation](https://docs.docker.com/cloud/aci-integration/).

You have to create the ACI context and the storage:

```s
docker context create aci myacicontext
docker context use myacicontext
docker volume create test-volume --storage-account torrustracker
```

You need to create all the files needed by the application in the storage dir `storage/database`.

And finally, you can run the container:

```s
docker run \
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/torrust/torrust-tracker:latest
```

Detach from container logs when the container starts. By default, the command line stays attached and follows container logs.

```s
docker run \
    --detach
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \latest
    --volume torrustracker/test-volume:/app/storage \
    registry.hub.docker.com/torrust/torrust-tracker:latest
```

You should see something like this:

```s
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
intelligent-hawking   registry.hub.docker.com/torrust/torrust-tracker:latest                       Running             4.236.213.57:6969->6969/udp, 4.236.213.57:1212->1212/tcp
```

After a while, you can use the tracker API `http://4.236.213.57:1212/api/stats?token=MyAccessToken` and the UDP tracker with your BitTorrent client using this tracker announce URL `udp://4.236.213.57:6969`.

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
