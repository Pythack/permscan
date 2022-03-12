# 1. This tells docker to use the Rust official image
FROM rust:1.49

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release
RUN cp ./target/release/permscan /bin
RUN rm -r target
RUN rm -r src
RUN rm Cargo.*
RUN rm Dockerfile

# Run the binary
#CMD ["cp ./target/release/permscan /bin"]
#EXPOSE 3030
