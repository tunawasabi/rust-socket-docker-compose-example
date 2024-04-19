FROM rust:latest

COPY . /app
WORKDIR /app
RUN cargo build --release --bin sender --lib

ENTRYPOINT [ "/bin/bash", "-c", "/app/target/release/sender" ]
