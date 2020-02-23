FROM rust:latest
COPY Cargo.toml /Cargo.toml
COPY src /src
RUN ["cargo", "build"]
EXPOSE 8080
ENTRYPOINT ["cargo", "run"]
