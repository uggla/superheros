#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

declare -i TIMEOUT=0
declare -i MAX_TIMEOUT=${1:-30}

echo "DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}/${POSTGRES_DB}" >.env

wait() {
  while (! nc "${POSTGRES_HOST}" 5432 </dev/null); do
    sleep 1s
    TIMEOUT=$(("${TIMEOUT}" + 1))
    if [[ "${TIMEOUT}" -ge "${MAX_TIMEOUT}" ]]; then
      echo "Timeout connecting to postgres db"
      exit 1
    fi
  done
}

main() {
  wait
  ./diesel setup
  ./diesel migration run
  ./superheros
}

main
