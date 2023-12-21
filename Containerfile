# syntax=docker/dockerfile:latest

# Torrust Tracker

## Builder Image
FROM rustlang/rust:nightly-bookworm as chef
WORKDIR /tmp
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-chef cargo-nextest

## Tester Image
FROM rustlang/rust:nightly-bookworm-slim as tester
WORKDIR /tmp

RUN apt-get update; apt-get install -y curl sqlite3; apt-get autoclean
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-nextest

COPY ./share/ /app/share/torrust
RUN mkdir -p /app/share/torrust/default/database/; \
    sqlite3 /app/share/torrust/default/database/tracker.sqlite3.db  "VACUUM;"

## Su Exe Compile
FROM docker.io/library/gcc:bookworm as gcc
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN cc -Wall -Werror -g /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec; chmod +x /usr/local/bin/su-exec


## Chef Prepare (look at project and see wat we need)
FROM chef AS recipe
WORKDIR /build/src
COPY . /build/src
RUN cargo chef prepare --recipe-path /build/recipe.json


## Cook (debug)
FROM chef AS dependencies_debug
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --tests --benches --examples --workspace --all-targets --all-features --recipe-path /build/recipe.json
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/temp.tar.zst ; rm -f /build/temp.tar.zst

## Cook (release)
FROM chef AS dependencies
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --tests --benches --examples --workspace --all-targets --all-features --recipe-path /build/recipe.json --release
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/temp.tar.zst --release  ; rm -f /build/temp.tar.zst


## Build Archive (debug)
FROM dependencies_debug AS build_debug
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/torrust-tracker-debug.tar.zst

## Build Archive (release)
FROM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/torrust-tracker.tar.zst --release


# Extract and Test (debug)
FROM tester as test_debug
WORKDIR /test
COPY . /test/src/
COPY --from=build_debug \
  /build/torrust-tracker-debug.tar.zst \
  /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN mkdir -p /app/bin/; cp -l /test/src/target/debug/torrust-tracker /app/bin/torrust-tracker
RUN mkdir /app/lib/; cp -l $(realpath $(ldd /app/bin/torrust-tracker | grep "libz\.so\.1" | awk '{print $3}')) /app/lib/libz.so.1
RUN chown -R root:root /app; chmod -R u=rw,go=r,a+X /app; chmod -R a+x /app/bin

# Extract and Test (release)
FROM tester as test
WORKDIR /test
COPY . /test/src
COPY --from=build \
  /build/torrust-tracker.tar.zst \
  /test/torrust-tracker.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-tracker.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN mkdir -p /app/bin/; cp -l /test/src/target/release/torrust-tracker /app/bin/torrust-tracker; cp -l /test/src/target/release/http_health_check /app/bin/http_health_check
RUN mkdir -p /app/lib/; cp -l $(realpath $(ldd /app/bin/torrust-tracker | grep "libz\.so\.1" | awk '{print $3}')) /app/lib/libz.so.1
RUN chown -R root:root /app; chmod -R u=rw,go=r,a+X /app; chmod -R a+x /app/bin


## Runtime
FROM gcr.io/distroless/cc-debian12:debug as runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec

ARG TORRUST_TRACKER_PATH_CONFIG="/etc/torrust/tracker/tracker.toml"
ARG TORRUST_TRACKER_DATABASE_DRIVER="sqlite3"
ARG USER_ID=1000
ARG UDP_PORT=6969
ARG HTTP_PORT=7070
ARG API_PORT=1212
ARG HEALTH_CHECK_API_PORT=1313

ENV TORRUST_TRACKER_PATH_CONFIG=${TORRUST_TRACKER_PATH_CONFIG}
ENV TORRUST_TRACKER_DATABASE_DRIVER=${TORRUST_TRACKER_DATABASE_DRIVER}
ENV USER_ID=${USER_ID}
ENV UDP_PORT=${UDP_PORT}
ENV HTTP_PORT=${HTTP_PORT}
ENV API_PORT=${API_PORT}
ENV HEALTH_CHECK_API_PORT=${HEALTH_CHECK_API_PORT}
ENV TZ=Etc/UTC

EXPOSE ${UDP_PORT}/udp
EXPOSE ${HTTP_PORT}/tcp
EXPOSE ${API_PORT}/tcp
EXPOSE ${HEALTH_CHECK_API_PORT}/tcp

RUN mkdir -p /var/lib/torrust/tracker /var/log/torrust/tracker /etc/torrust/tracker

ENV ENV=/etc/profile
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh

VOLUME ["/var/lib/torrust/tracker","/var/log/torrust/tracker","/etc/torrust/tracker"]

ENV RUNTIME="runtime"
ENTRYPOINT ["/usr/local/bin/entry.sh"]


## Torrust-Tracker (debug)
FROM runtime as debug
ENV RUNTIME="debug"
COPY --from=test_debug /app/ /usr/
RUN env
CMD ["sh"]

## Torrust-Tracker (release) (default)
FROM runtime as release
ENV RUNTIME="release"
COPY --from=test /app/ /usr/
HEALTHCHECK --interval=5s --timeout=5s --start-period=3s --retries=3 \  
  CMD /usr/bin/http_health_check http://localhost:${HEALTH_CHECK_API_PORT}/health_check \
    || exit 1
CMD ["/usr/bin/torrust-tracker"]
