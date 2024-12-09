FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/assets /usr/app/assets
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/gabrielkaszewski_rs-cli /usr/app/gabrielkaszewski_rs-cli

ENTRYPOINT ["/usr/app/gabrielkaszewski_rs-cli", "start"]