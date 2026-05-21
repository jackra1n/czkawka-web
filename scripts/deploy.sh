#!/usr/bin/env bash

set -euo pipefail

# Find script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if .env.deploy exists
ENV_FILE="$PROJECT_ROOT/.env.deploy"
if [ ! -f "$ENV_FILE" ]; then
    echo "Error: $ENV_FILE not found."
    echo "Please copy .env.deploy.example to .env.deploy and fill in your details:"
    echo "  cp .env.deploy.example .env.deploy"
    exit 1
fi

# Load variables
while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" =~ ^[[:space:]]*# ]] || [[ "$line" =~ ^[[:space:]]*$ ]]; then
        continue
    fi
    trimmed=$(echo "$line" | xargs)
    export "$trimmed"
done < "$ENV_FILE"

# Check required variables
if [ -z "${DEPLOY_HOST:-}" ] || [ -z "${DEPLOY_USER:-}" ] || [ -z "${DEPLOY_COMPOSE_DIR:-}" ]; then
    echo "Error: DEPLOY_HOST, DEPLOY_USER, and DEPLOY_COMPOSE_DIR must be defined in .env.deploy"
    exit 1
fi

IMAGE_TAG="czkawka-web:local"
TEMP_TAR="/tmp/czkawka-web-deploy.tar"
REMOTE_TEMP_TAR="/tmp/czkawka-web-deploy.tar"

echo "=== 1. Building Docker image locally ==="
docker build -t "$IMAGE_TAG" -f "$PROJECT_ROOT/Dockerfile" "$PROJECT_ROOT"

echo "=== 2. Saving Docker image to tarball ==="
docker save "$IMAGE_TAG" -o "$TEMP_TAR"

echo "=== 3. Transferring tarball to remote server ==="
scp "$TEMP_TAR" "$DEPLOY_USER@$DEPLOY_HOST:$REMOTE_TEMP_TAR"

echo "=== 4. Loading image on remote server ==="
ssh "$DEPLOY_USER@$DEPLOY_HOST" "docker load -i $REMOTE_TEMP_TAR"

echo "=== 5. Cleaning up remote temporary files ==="
ssh "$DEPLOY_USER@$DEPLOY_HOST" "rm -f $REMOTE_TEMP_TAR"

echo "=== 6. Restarting containers via Docker Compose ==="
ssh "$DEPLOY_USER@$DEPLOY_HOST" "cd $DEPLOY_COMPOSE_DIR && docker compose down && docker compose up -d"

echo "=== 7. Cleaning up local temporary files ==="
rm -f "$TEMP_TAR"

echo "=== Deploy completed successfully! ==="
