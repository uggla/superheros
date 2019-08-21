#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

echo "DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}/${POSTGRES_DB}" >.env

./diesel setup
./diesel migration run
./superheros
