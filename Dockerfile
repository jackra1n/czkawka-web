# Stage 1: Frontend Build
FROM oven/bun:1 AS frontend-builder

WORKDIR /app/frontend
COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend/ ./
RUN bun run build

# Stage 2: Backend Build
FROM rust:1-bookworm AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    libchromaprint-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/backend

# Copy dependency configs first to cache dependency compilation
COPY backend/Cargo.toml backend/Cargo.lock ./

# Create dummy source files to compile dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Clean up dummy build files of our package (but keep dependencies in target/release/deps)
RUN rm -rf src/ target/release/deps/backend* target/release/backend*

# Now copy the real source code
COPY backend/src ./src

# Build the actual backend application
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gosu \
    libssl3 \
    libchromaprint1 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 czkawka && \
    useradd -u 1000 -g czkawka -d /data -m czkawka

WORKDIR /app

COPY scripts/entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

RUN mkdir -p /app/backend
COPY --from=backend-builder /app/backend/target/release/backend /app/backend/backend
COPY --from=frontend-builder /app/frontend/build /app/frontend/build

ENV HOME=/data
ENV DEFAULT_SCAN_PATH=/mnt/files
ENV RUST_LOG=warn,backend=info
ENV PORT=6198
VOLUME ["/data"]
EXPOSE 6198

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/api/health || exit 1

ENTRYPOINT ["/entrypoint.sh"]
CMD ["/app/backend/backend"]
