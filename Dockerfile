# Dockerfile for Verve Server
FROM rust:1.95.0-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin verve-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/verve-server /usr/local/bin/
EXPOSE 3097
CMD ["verve-server", "--port", "3097"]
