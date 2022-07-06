FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 AS chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin torrust-tracker


FROM debian:bullseye-slim AS runtime
WORKDIR /app

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p /app

RUN chown -R $APP_USER:$APP_USER /app

RUN apt-get -y update \
  && apt-get -y upgrade \
  && apt-get install -y sqlite3 libssl1.1

EXPOSE 6969
EXPOSE 1212

COPY --from=builder /app/target/release/torrust-tracker /app

USER $APP_USER

ENTRYPOINT ["/app/torrust-tracker"]
