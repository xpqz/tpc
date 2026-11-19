#!/bin/sh
# Usage: tools/swag.sh run [--sample N] [--jobs J] [--only PREFIX] | report
# SWAG validation sweep; see tools/swag.py.
exec python3 -I "$(dirname "$0")/swag.py" "$@"
