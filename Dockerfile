FROM rust:1.49
LABEL org.opencontainers.image.source="https://github.com/pythack/permscan"

WORKDIR /permscan_code/
COPY ./Cargo.toml ./
COPY ./src/ ./src/

RUN cargo build --release
RUN cp ./target/release/permscan /bin

RUN mkdir permscan-x86_64-unknown-linux-gnu
RUN mv ./target/release/permscan permscan-x86_64-unknown-linux-gnu
RUN tar -czvf permscan-x86_64-unknown-linux-gnu.tar.gz permscan-x86_64-unknown-linux-gnu
RUN rm -rf permscan-x86_64-unknown-linux-gnu

WORKDIR /
