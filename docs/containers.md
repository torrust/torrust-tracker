---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/index.md
    - Containerfile
    - share/container/entry_script_sh
---

# Containers (Docker or Podman)

## Demo environment

It is simple to setup the tracker with the default
configuration and run it using the pre-built public docker image:

With Docker:

```sh
docker run -it torrust/tracker:latest
```

or with Podman:

```sh
podman run -it docker.io/torrust/tracker:latest
```

## Requirements

- Tested with recent versions of Docker or Podman.

## Volumes

The [Containerfile](../Containerfile) (i.e. the Dockerfile) Defines Three Volumes:

```Dockerfile
VOLUME ["/var/lib/torrust/tracker","/var/log/torrust/tracker","/etc/torrust/tracker"]
```

When instancing the container image with the `docker run` or `podman run` command, we map these volumes to the local storage:

```s
./storage/tracker/lib -> /var/lib/torrust/tracker
./storage/tracker/log -> /var/log/torrust/tracker
./storage/tracker/etc -> /etc/torrust/tracker
```

> NOTE: You can adjust this mapping for your preference, however this mapping is the default in our guides and scripts.

### Pre-Create Host-Mapped Folders

Please run this command where you wish to run the container:

```sh
mkdir -p ./storage/tracker/lib/ ./storage/tracker/log/ ./storage/tracker/etc/
```

### Matching Ownership ID's of Host Storage and Container Volumes

It is important that the `torrust` user has the same uid `$(id -u)` as the host mapped folders. In our [entry script](../share/container/entry_script_sh), installed to `/usr/local/bin/entry.sh` inside the container, switches to the `torrust` user created based upon the `USER_UID` environmental variable.

When running the container, you may use the `--env USER_ID="$(id -u)"` argument that gets the current user-id and passes to the container.

### Mapped Tree Structure

Using the standard mapping defined above produces this following mapped tree:

```s
storage/tracker/
├── lib
│   ├── database           => created only when SQLite persistence is selected
│   │   └── sqlite3.db     => /var/lib/torrust/tracker/database/sqlite3.db
│   └── tls
│       ├── localhost.crt  => /var/lib/torrust/tracker/tls/localhost.crt [user supplied]
│       └── localhost.key  => /var/lib/torrust/tracker/tls/localhost.key [user supplied]
├── log                    => /var/log/torrust/tracker (future use)
└── etc
    └── tracker.toml        => /etc/torrust/tracker/tracker.toml [auto populated]
```

> NOTE: you only need the `tls` directory and certificates in case you have enabled SSL.

## Building the Container

### Clone and Change into Repository

```sh
# Inside your dev folder
git clone https://github.com/torrust/torrust-tracker.git; cd torrust-tracker
```

### (Docker) Setup Context

Before starting, if you are using docker, it is helpful to reset the context to the default:

```sh
docker context use default
```

### (Docker) Build

```sh
# Release Mode
docker build --target release --tag torrust-tracker:release --file Containerfile .

# Debug Mode
docker build --target debug --tag torrust-tracker:debug --file Containerfile .
```

### (Podman) Build

```sh
# Release Mode
podman build --target release --tag torrust-tracker:release --file Containerfile .

# Debug Mode
podman build --target debug --tag torrust-tracker:debug --file Containerfile .
```

## Running the Container

### Basic Run

No arguments are needed for simply checking the container image works:

#### (Docker) Run Basic

```sh
# Release Mode
docker run -it torrust-tracker:release

# Debug Mode
docker run -it torrust-tracker:debug
```

#### (Podman) Run Basic

```sh
# Release Mode
podman run -it docker.io/torrust-tracker:release

# Debug Mode
podman run -it docker.io/torrust-tracker:debug
```

### Arguments

The arguments need to be placed before the image tag. i.e.

`run [arguments] torrust-tracker:release`

#### Environmental Variables

Environmental variables are loaded through the `--env`, in the format `--env VAR="value"`.

The following environmental variables can be set:

- `TORRUST_TRACKER_CONFIG_TOML_PATH` - The in-container path to the tracker configuration file, (default: `"/etc/torrust/tracker/tracker.toml"`).
- `TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN` - Override of the admin token. If set, this value overrides any value set in the config.
- `TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER` - Optional selection of a packaged persistence configuration for a fresh `/etc/torrust/tracker/tracker.toml` (options: `sqlite3`, `mysql`, `postgresql`). When omitted, the container installs the packaged v3 public tracker configuration, which omits `[core.database]`. This does not override an existing mounted configuration file.
- `TORRUST_TRACKER_CONFIG_TOML` - Load config from this environmental variable instead from a file, (i.e: `TORRUST_TRACKER_CONFIG_TOML=$(cat tracker-tracker.toml)`).
- `USER_ID` - The user id for the runtime crated `torrust` user. Please Note: This user id should match the ownership of the host-mapped volumes, (default `1000`).
- `UDP_PORT` - The port for the UDP tracker. This should match the port used in the configuration, (default `6969`).
- `HTTP_PORT` - The port for the HTTP tracker. This should match the port used in the configuration, (default `7070`).
- `API_PORT` - The port for the tracker API. This should match the port used in the configuration, (default `1212`).
- `HEALTH_CHECK_API_PORT` - The port for the Health Check API. This should match the port used in the configuration, (default `1313`).

#### Persistence-Free Default

With no mounted `tracker.toml` and no database-driver override, the image
installs `tracker.container.no-persistence.toml`. It starts public UDP and HTTP
trackers plus the health API without `[core.database]`. The entrypoint does not
create `/var/lib/torrust/tracker/database` or install a SQLite database in this
mode. Supply a mounted v3 configuration for other listener and capability
combinations.

#### PostgreSQL backend notes

To run the tracker with PostgreSQL in containers:

- Set `TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=postgresql`.
- Use the default PostgreSQL container configuration file:
  `share/default/config/tracker.container.postgresql.toml`.
- Ensure the target database exists before tracker startup.
  The default PostgreSQL DSN in the container config expects `torrust_tracker`.

When using a PostgreSQL container, set `POSTGRES_DB=torrust_tracker` (or create the
same database explicitly) so the tracker can connect at startup.

### Sockets

Socket ports used internally within the container can be mapped to with the `--publish` argument.

The format is: `--publish [optional_host_ip]:[host_port]:[container_port]/[optional_protocol]`, for example: `--publish 127.0.0.1:8080:80/tcp`.

The default ports can be mapped with the following:

```s
--publish 0.0.0.0:7070:7070/tcp \
--publish 0.0.0.0:6969:6969/udp \
--publish 0.0.0.0:1212:1212/tcp \
```

> NOTE: Inside the container it is necessary to expose a socket with the wildcard address `0.0.0.0` so that it may be accessible from the host. Verify that the configuration that the sockets are wildcard.

### HTTP/3 at the edge with a reverse proxy

The tracker does not need native HTTP/3 support to offer HTTP/3 to clients. You can terminate
HTTP/3 at an edge reverse proxy and forward traffic to the tracker over HTTP/1.1 or HTTP/2.

Protocol boundary:

- Client to proxy: HTTP/1.1, HTTP/2, or HTTP/3 (optional).
- Proxy to tracker backend: HTTP/1.1 or HTTP/2.

This keeps deployment flexible while native HTTP/3 support in the Rust HTTP ecosystem continues
to mature.

#### Caddy example

Expose both TCP and UDP on port `443` for QUIC/HTTP/3, and forward tracker endpoints to the
existing tracker HTTP ports.

```text
{
    servers :443 {
        protocols h1 h2 h3
    }
}

tracker.example.com {
    reverse_proxy tracker:7070 {
        # Forward the original client IP when tracker runs behind a proxy.
        header_up X-Forwarded-For {remote_host}
    }
}

api.example.com {
    reverse_proxy tracker:1212
}
```

> **Tracker configuration required:** set `core.net.on_reverse_proxy = true` in the tracker
> configuration so it reads the peer IP from the `X-Forwarded-For` header rather than the proxy's
> TCP connection address. Without this setting, the tracker ignores the forwarded header and
> records the proxy's IP as every peer's address.

If Caddy runs in a container, publish both protocols on `443`:

```sh
--publish 0.0.0.0:443:443/tcp \
--publish 0.0.0.0:443:443/udp
```

Reference: [Caddy HTTP/3 documentation](https://caddyserver.com/docs/protocol/http3)

Latest reference from Torrust Tracker Demo:
[torrust-tracker-demo Caddy config](https://raw.githubusercontent.com/torrust/torrust-tracker-demo/refs/heads/main/server/opt/torrust/storage/caddy/etc/Caddyfile)

#### Operational guidance

- HTTP/3 at the edge is optional. Keep the tracker backend unchanged and enable/disable HTTP/3 in
  the proxy configuration when needed.
- Roll out gradually. Start with a single environment and compare behaviour before broad rollout.
- Monitor CPU and memory on the proxy, plus request error rates, as QUIC load can shift resource
  usage from backend services to the edge.
- Keep an easy rollback path: remove `h3` support in the proxy and keep serving HTTP/1.1 and
  HTTP/2 without tracker code changes.

#### Manual verification

Use these commands to verify HTTP/3 against the Torrust demo tracker. Replace
`http1.torrust-tracker-demo.com` with your own hostname to verify your own deployment:

```bash
# 1) Confirm alt-svc advertisement for h3
curl -sI https://http1.torrust-tracker-demo.com/announce | grep -i alt-svc

# 2) Force HTTP/3 only (requires curl built with HTTP/3 support)
/snap/bin/curl --http3-only -sI https://http1.torrust-tracker-demo.com/announce

# 3) Optional: inspect QUIC and protocol negotiation
/snap/bin/curl --http3-only -v https://http1.torrust-tracker-demo.com/announce 2>&1 \
    | grep -E 'QUIC|HTTP/3|h3|Connected|protocol'
```

Expected for step 2: the response status line starts with `HTTP/3 200`.

### Host-mapped Volumes

By default the container will use install volumes for `/var/lib/torrust/tracker`, `/var/log/torrust/tracker`, and `/etc/torrust/tracker`, however for better administration it good to make these volumes host-mapped.

The argument to host-map volumes is `--volume`, with the format: `--volume=[host-src:]container-dest[:<options>]`.

The default mapping can be supplied with the following arguments:

```s
--volume ./storage/tracker/lib:/var/lib/torrust/tracker:Z \
--volume ./storage/tracker/log:/var/log/torrust/tracker:Z \
--volume ./storage/tracker/etc:/etc/torrust/tracker:Z \
```

Please not the `:Z` at the end of the podman `--volume` mapping arguments, this is to give read-write permission on SELinux enabled systemd, if this doesn't work on your system, you can use `:rw` instead.

## Complete Example

### With Docker

```sh
## Setup Docker Default Context
docker context use default

## Build Container Image
docker build --target release --tag torrust-tracker:release --file Containerfile .

## Setup Mapped Volumes
mkdir -p ./storage/tracker/lib/ ./storage/tracker/log/ ./storage/tracker/etc/

## Run Torrust Tracker Container Image
docker run -it \
    --env TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN="MySecretToken" \
    --env USER_ID="$(id -u)" \
    --publish 0.0.0.0:7070:7070/tcp \
    --publish 0.0.0.0:6969:6969/udp \
    --publish 0.0.0.0:1212:1212/tcp \
    --volume ./storage/tracker/lib:/var/lib/torrust/tracker:Z \
    --volume ./storage/tracker/log:/var/log/torrust/tracker:Z \
    --volume ./storage/tracker/etc:/etc/torrust/tracker:Z \
    torrust-tracker:release
```

### With Podman

```sh
## Build Container Image
podman build --target release --tag torrust-tracker:release --file Containerfile .

## Setup Mapped Volumes
mkdir -p ./storage/tracker/lib/ ./storage/tracker/log/ ./storage/tracker/etc/

## Run Torrust Tracker Container Image
podman run -it \
    --env TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN="MySecretToken" \
    --env USER_ID="$(id -u)" \
    --publish 0.0.0.0:7070:7070/tcp \
    --publish 0.0.0.0:6969:6969/udp \
    --publish 0.0.0.0:1212:1212/tcp \
    --volume ./storage/tracker/lib:/var/lib/torrust/tracker:Z \
    --volume ./storage/tracker/log:/var/log/torrust/tracker:Z \
    --volume ./storage/tracker/etc:/etc/torrust/tracker:Z \
    docker.io/torrust-tracker:release
```

## Docker Compose

The docker-compose configuration includes the MySQL service configuration. If you want to use MySQL instead of SQLite you should verify the `/etc/torrust/tracker/tracker.toml` (i.e `./storage/tracker/etc/tracker.toml`) configuration:

```toml
[core.database]
driver = "mysql"
path = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker"
```

Important: if the MySQL password contains reserved URL characters (for example `+`, `/`, `@`, or `:`), it must be percent-encoded in the DSN password component. For example, if the raw password is `a+b/c`, use `a%2Bb%2Fc` in the DSN.

When generating secrets automatically, prefer URL-safe passwords (`A-Z`, `a-z`, `0-9`, `-`, `_`) to avoid DSN parsing issues.

### Build and Run:

```sh
docker build --target release --tag torrust-tracker:release --file Containerfile .

mkdir -p ./storage/tracker/lib/ ./storage/tracker/log/ ./storage/tracker/etc/

USER_ID=$(id -u) \
    TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_API__ACCESS_TOKENS__ADMIN="MySecretToken" \
    docker compose up --build
```

After running the `compose up` command you will have two running containers:

```s
$ docker ps
CONTAINER ID   IMAGE             COMMAND                  CREATED          STATUS                   PORTS                                                                                                                             NAMES
06feacb91a9e   torrust-tracker   "cargo run"              18 minutes ago   Up 4 seconds             0.0.0.0:1212->1212/tcp, :::1212->1212/tcp, 0.0.0.0:7070->7070/tcp, :::7070->7070/tcp, 0.0.0.0:6969->6969/udp, :::6969->6969/udp   torrust-tracker-1
34d29e792ee2   mysql:8.0         "docker-entrypoint.s…"   18 minutes ago   Up 5 seconds (healthy)   0.0.0.0:3306->3306/tcp, :::3306->3306/tcp, 33060/tcp                                                                              torrust-mysql-1
```

And you should be able to use the application, for example making a request to the API:

<https://127.0.0.1:1212/api/v1/stats?token=MySecretToken>

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

You can use a certificate for localhost. You can create your [localhost certificate](https://letsencrypt.org/docs/certificates-for-localhost/#making-and-trusting-your-own-certificates) and use it in the `storage` folder and the configuration file (`tracker.toml`). For example:

The storage folder must contain your certificates:

```s
storage/tracker/lib/tls
                    ├── localhost.crt
                    └── localhost.key
storage/http_api/lib/tls
                     ├── localhost.crt
                     └── localhost.key
```

You have not enabled it in your `tracker.toml` file:

```toml
[http_trackers.tsl_config]
ssl_cert_path = "./storage/tracker/lib/tls/localhost.crt"
ssl_key_path = "./storage/tracker/lib/tls/localhost.key"

[http_api.tsl_config]
ssl_cert_path = "./storage/http_api/lib/tls/localhost.crt"
ssl_key_path = "./storage/http_api/lib/tls/localhost.key"
```

> NOTE: you can enable it independently for each HTTP tracker or the API.

If you enable the SSL certificate for the API, for example, you can load the API with this URL:

<https://localhost:1212/api/v1/stats?token=MyAccessToken>

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

You need to create all the files needed by the application in the storage dir `storage/lib/database`.

And finally, you can run the container:

```s
docker run \
    --env USER_ID="$(id -u)" \
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \
    --volume torrustracker/lib:/var/lib/torrust/tracker:rw \
    --volume torrustracker/log:/var/log/torrust/tracker:rw \
    --volume torrustracker/etc:/etc/torrust/tracker:rw \
    registry.hub.docker.com/torrust/tracker:latest
```

Detach from container logs when the container starts. By default, the command line stays attached and follows container logs.

```s
docker run \
    --detach
    --env USER_ID="$(id -u)" \
    --publish 6969:6969/udp \
    --publish 7070:7070/tcp \
    --publish 1212:1212/tcp \latest
    --volume torrustracker/lib:/var/lib/torrust/tracker:rw \
    --volume torrustracker/log:/var/log/torrust/tracker:rw \
    --volume torrustracker/etc:/etc/torrust/tracker:rw \
    registry.hub.docker.com/torrust/tracker:latest
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
intelligent-hawking   registry.hub.docker.com/torrust/tracker:latest                       Running             4.236.213.57:6969->6969/udp, 4.236.213.57:1212->1212/tcp
```

After a while, you can use the tracker API `http://4.236.213.57:1212/api/v1/stats?token=MyAccessToken` and the UDP tracker with your BitTorrent client using this tracker announce URL `udp://4.236.213.57:6969`.

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
