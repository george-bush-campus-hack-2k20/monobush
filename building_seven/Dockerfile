FROM rust:latest
ADD Cargo.toml /Cargo.toml
ADD src /src
RUN cargo build
ENTRYPOINT ["cargo", "run"]
