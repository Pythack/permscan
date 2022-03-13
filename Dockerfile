FROM rust:1.49
LABEL org.opencontainers.image.source="https://github.com/pythack/permscan"

WORKDIR /permscan_code/
COPY ./Cargo.toml ./
COPY ./src/ ./src/

RUN cargo build --release
RUN cp ./target/release/permscan /bin

WORKDIR /
