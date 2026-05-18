# czkawka-web

A lightning-fast, minimalist web frontend for the [Czkawka](https://github.com/qarmin/czkawka) duplicate file finder.

I built this because I wanted to run Czkawka on my server, but I needed a good UI and easy Docker deployment for it.

![Czkawka Web Preview](docs/images/czkawka-web_preview.webp)

## Features

* **All Czkawka scans:** Supports every scan type that Czkawka offers.
* **Image comparisons:** Side-by-side, swipe, and onion skin comparison modes.
* **Docker deployment:** Easy deployment on headless servers via Docker.

## Docker

Pull the pre-built image from GitHub Container Registry:

```bash
docker pull ghcr.io/jackra1n/czkawka-web
```

### Run

```bash
docker run -d \
  --name czkawka-web \
  -p 3000:3000 \
  -v czkawka-web-data:/data \
  -v /path/to/your/files:/mnt/files \
  ghcr.io/jackra1n/czkawka-web
```

### Docker Compose

```yaml
services:
  czkawka-web:
    image: ghcr.io/jackra1n/czkawka-web
    container_name: czkawka-web
    ports:
      - "3000:3000"
    volumes:
      - ./data:/data
      # Mount directories you want to scan:
      # - /path/on/host/to/files:/mnt/files
    restart: unless-stopped
```

Then start it:

```bash
docker compose up -d
```

## Tech Stack
This project is built with an emphasis on zero bloat and high performance.

* **Backend:** [Rust](https://www.rust-lang.org/) + [Axum](https://github.com/tokio-rs/axum)
* **Frontend:** [SvelteKit](https://kit.svelte.dev/)
* [Bun](https://bun.sh/)
* [Mise](https://mise.jdx.dev/)

## Development / Local Build

If you want to build the image locally from source instead of using the pre-built one, use the development compose file:

```bash
cp .env.example .env
# Edit .env and set SCAN_PATH to the directory you want to scan
mkdir -p data
docker compose -f compose.dev.yaml up
```

If you use [Mise](https://mise.jdx.dev/), you can also run:

```bash
mise run docker
```

## Acknowledgments
A huge thanks to [qarmin](https://github.com/qarmin) for creating and maintaining [czkawka](https://github.com/qarmin/czkawka). This project wouldn't be possible without the incredible work put into that amazing project.
