# Multi-stage build for minimal image size
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/web-api /usr/local/bin/

EXPOSE 3000

CMD ["web-api"]
