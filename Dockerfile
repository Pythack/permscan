# 1. Use the Rust official image
FROM rust:1.49

LABEL org.opencontainers.image.source="https://github.com/pythack/permscan"

# Copy the files to the Docker image
WORKDIR /permscan_code/
COPY ./Cargo.toml ./
COPY ./src/ ./src/

# Build binary program for release
RUN cargo build --release
RUN cp ./target/release/permscan /bin

WORKDIR /
