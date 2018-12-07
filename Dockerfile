FROM rust:latest
COPY . /usr/src/udpt
WORKDIR /usr/src/udpt

RUN cargo build --release -j4

CMD ["target/release/udpt-rs", "-c", "/usr/src/udpt/udpt.toml"]
