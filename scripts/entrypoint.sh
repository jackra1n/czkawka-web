#!/bin/sh
set -e

# Fast path: already running as non-root
if [ "$(id -u)" != "0" ]; then
    mkdir -p /data
    exec "$@"
fi

# Root path: configure permissions
PUID=${PUID:-1000}
PGID=${PGID:-1000}

# Update the built-in czkawka user to match requested IDs
groupmod -o -g "${PGID}" czkawka 2>/dev/null || true
usermod -o -u "${PUID}" czkawka 2>/dev/null || true

# Create data dir and ensure ownership
mkdir -p /data
chown czkawka:czkawka /data

# Drop privileges and run
exec gosu czkawka "$@"
