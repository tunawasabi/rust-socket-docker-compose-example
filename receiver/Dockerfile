FROM rust：latest

COPY . /app
RUN cargo build --release

ENTRYPOINT [ "/bin/bash", "-c", "cargo run --release" ]
