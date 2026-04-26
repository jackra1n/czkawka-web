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

WORKDIR /app
COPY backend/ ./backend/
COPY --from=frontend-builder /app/frontend/build ./frontend/build

WORKDIR /app/backend
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libchromaprint1 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd -g 1000 czkawka && \
    useradd -u 1000 -g czkawka -d /data -m czkawka

USER czkawka
WORKDIR /app

RUN mkdir -p /app/backend
COPY --from=backend-builder /app/backend/target/release/backend /app/backend/backend
COPY --from=frontend-builder /app/frontend/build /app/frontend/build

ENV HOME=/data
ENV RUST_LOG=warn
VOLUME ["/data"]
EXPOSE 3000

ENTRYPOINT ["/app/backend/backend"]
