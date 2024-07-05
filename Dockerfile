# Source for Dockerfile:
# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# BUILD IMAGE
FROM rust as build
RUN USER=root cargo new --bin tako
WORKDIR /tako

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release && rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/tako* && cargo build --release

# RELEASE IMAGE
FROM debian:bookworm-slim
COPY --from=build /tako/target/release/tako .
CMD ["./tako"]
