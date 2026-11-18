#!/bin/sh
# Print the Cargo target directory for this checkout: /Volumes/Space2/tpa/<checkout name>
# when the external SSD is really mounted, else <checkout>/target-shared. Each worktree
# gets its own directory so parallel builds never contend for one cargo lock. Used by ~/.zshenv
# (PWD-guarded to this repo) so every shell here, including agents', builds there.
# Override the mount point with TPA_CARGO_VOLUME.

: "${TPA_CARGO_VOLUME:=/Volumes/Space2}"

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)

# A stale /Volumes/<name> directory left by an unclean disconnect lives on the system
# disk; a real mount has a different device id from /.
if [ -d "$TPA_CARGO_VOLUME" ] && [ "$(stat -f %d /)" != "$(stat -f %d "$TPA_CARGO_VOLUME")" ]; then
    printf '%s/tpa/%s\n' "$TPA_CARGO_VOLUME" "$(basename "$repo")"
else
    printf '%s/target-shared\n' "$repo"
fi
