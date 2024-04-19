FROM rust:latest

COPY . /app
WORKDIR /app
RUN cargo build --release

ENTRYPOINT [ "/bin/bash", "-c", "cargo run --release" ]
