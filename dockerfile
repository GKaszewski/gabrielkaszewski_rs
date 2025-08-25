# =================================================================
# Stage 1: Build the Rust application
# =================================================================
FROM rust:1.89-slim-bookworm AS builder

RUN apt-get update && apt-get install -y libsqlite3-dev pkg-config build-essential

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo/
COPY migration ./migration

RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/bin/main.rs && \
    echo "fn main() {}" > src/bin/tool.rs
RUN cargo build --release

COPY src ./src
COPY assets ./assets
COPY config ./config
RUN cargo build --release

# =================================================================
# Stage 2: Create the final, lightweight runtime image
# =================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y libsqlite3-0 libssl3 gosu && rm -rf /var/lib/apt/lists/*

RUN addgroup --system nonroot && adduser --system --ingroup nonroot nonroot

WORKDIR /app

COPY --from=builder /app/target/release/gabrielkaszewski_rs-cli ./server

COPY assets ./assets
COPY config ./config

RUN mkdir -p /app/db /app/uploads && chown -R nonroot:nonroot /app/db /app/uploads

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENV LOCO_ENV=production
ENV DATABASE_URL=sqlite:///app/db/production.db?mode=rwc

EXPOSE 5150

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["./server", "start"]