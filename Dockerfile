FROM clux/muslrust:stable AS chef
WORKDIR /app
RUN cargo install cargo-chef


FROM chef AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
WORKDIR /app
ARG UID=1000
# Create the app user
ENV USER=appuser
ENV UID=$UID
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"
# Build dependencies
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Build the application
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin torrust-tracker
# Strip the binary
# More info: https://github.com/LukeMathWalker/cargo-chef/issues/149
RUN strip /app/target/x86_64-unknown-linux-musl/release/torrust-tracker


FROM alpine:latest
WORKDIR /app
ARG RUN_AS_USER=appuser
ARG TRACKER_UDP_PORT=6969
ARG TRACKER_HTTP_PORT=7070
ARG TRACKER_API_PORT=1212
RUN apk --no-cache add ca-certificates
ENV TZ=Etc/UTC
ENV RUN_AS_USER=$RUN_AS_USER
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder --chown=$RUN_AS_USER \
  /app/target/x86_64-unknown-linux-musl/release/torrust-tracker \
  /app/torrust-tracker
RUN chown -R $RUN_AS_USER:$RUN_AS_USER /app
USER $RUN_AS_USER:$RUN_AS_USER
EXPOSE $TRACKER_UDP_PORT/udp
EXPOSE $TRACKER_HTTP_PORT/tcp
EXPOSE $TRACKER_API_PORT/tcp
ENTRYPOINT ["/app/torrust-tracker"]