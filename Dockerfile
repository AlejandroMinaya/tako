# Source for Dockerfile:
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# BUILD IMAGE
FROM rust as build
RUN USER=root cargo new --bin tako
WORKDIR /tako

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo install sqlx-cli --no-default-features --features openssl-vendored,native-tls,postgres\
    && cargo build --release\
    && rm src/*.rs
COPY ./src ./src

# BUILD FOR RELEASE IMAGE
RUN rm ./target/release/deps/tako* &&\
    cargo build --release

# RELEASE IMAGE
FROM debian:bookworm-slim
COPY ./migrations ./migrations
COPY --from=build /usr/local/cargo/bin/sqlx .
COPY --from=build /tako/target/release/tako .
CMD ["sh", "-c", "./sqlx database setup --database-url $(cat /run/secrets/db_conn_url) && ./tako"]
