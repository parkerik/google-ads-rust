FROM rust:1.79 as builder

RUN USER=root cargo new --bin google-ads-rust
WORKDIR /google-ads-rust

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src
COPY ./.env ./.env

RUN rm ./target/release/deps/google_ads_rust*
RUN cargo build --release

FROM debian:stable-slim

RUN apt-get update && apt-get install ca-certificates -y && update-ca-certificates

WORKDIR /usr/api
COPY --from=builder /google-ads-rust/target/release/google-ads-rust .

CMD ["./google-ads-rust"]
