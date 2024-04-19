FROM rust:latest

COPY . /app
WORKDIR /app
RUN cargo build --release --bin receiver --lib
ENTRYPOINT [ "/bin/bash", "-c", "target/release/receiver" ]
