FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN groupadd -r redis-exporter && useradd -r -g redis-exporter redis-exporter

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/redis-exporter /usr/local/bin/

USER redis-exporter

ENTRYPOINT ["redis-exporter"]
