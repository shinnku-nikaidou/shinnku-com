# Backend Dockerfile for Rust application
FROM rust:1.80 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin shinnku-com-backend

# Runtime stage
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN addgroup --system --gid 1001 appuser
RUN adduser --system --uid 1001 appuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/shinnku-com-backend /usr/local/bin
# Copy configuration files
COPY --chown=appuser:appuser config.toml ./
COPY --chown=appuser:appuser data/ ./data/

# Change to non-root privilege
USER appuser

EXPOSE 8080

CMD ["shinnku-com-backend"]
