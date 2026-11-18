#!/bin/sh
# Print the Cargo target directory for tpa: /Volumes/Space2/tpa/target-shared when the
# external SSD is really mounted, else <repo>/target-shared. Used by ~/.zshenv
# (PWD-guarded to this repo) so every shell here, including agents', builds there.
# Override the mount point with TPA_CARGO_VOLUME.

: "${TPA_CARGO_VOLUME:=/Volumes/Space2}"

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)

# A stale /Volumes/<name> directory left by an unclean disconnect lives on the system
# disk; a real mount has a different device id from /.
if [ -d "$TPA_CARGO_VOLUME" ] && [ "$(stat -f %d /)" != "$(stat -f %d "$TPA_CARGO_VOLUME")" ]; then
    printf '%s/tpa/target-shared\n' "$TPA_CARGO_VOLUME"
else
    printf '%s/target-shared\n' "$repo"
fi
