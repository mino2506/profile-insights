#!/usr/bin/env bash
set -euo pipefail

cargo clean

docker compose down -v

docker compose up -d postgres

sqlx migrate run --source apps/rust-server/migrations