#!/bin/sh
set -e

chown -R nonroot:nonroot /app/db /app/uploads

exec gosu nonroot "$@"