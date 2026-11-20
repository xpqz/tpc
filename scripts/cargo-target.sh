#!/bin/sh
# Prints the Cargo target directory for this checkout. The Makefile, the pre-commit hook and the
# tests that build the compiler all ask this script, so every build of a checkout lands in one
# place, and each worktree gets its own directory (parallel builds never share a cargo lock).
#
# The default is <checkout>/target. To build elsewhere (another disk, say), name a directory in
# TPA_TARGET_ROOT or in a one-line file `.target-root` at the top of the main checkout (git-ignored,
# found from worktrees too); builds then go to <that directory>/<checkout name>. A root that does
# not exist, such as a disk that is not mounted, falls back to the default.

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
main=$(cd "$repo" && cd "$(git rev-parse --path-format=absolute --git-common-dir 2>/dev/null)/.." 2>/dev/null && pwd)

root=${TPA_TARGET_ROOT:-}
if [ -z "$root" ] && [ -n "$main" ] && [ -f "$main/.target-root" ]; then
    root=$(head -n 1 "$main/.target-root")
fi

if [ -n "$root" ] && [ -d "$root" ]; then
    printf '%s/%s\n' "$root" "$(basename "$repo")"
else
    printf '%s/target\n' "$repo"
fi
