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
# todo: it seems the previous cache layer is not working. The dependencies are compiled always.
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin torrust-tracker


FROM alpine:latest
WORKDIR /app
ARG RUN_AS_USER=appuser
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
EXPOSE 6969/udp
EXPOSE 6969/tcp
EXPOSE 1212/tcp
ENTRYPOINT ["/app/torrust-tracker"]