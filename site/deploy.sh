#!/bin/sh
# Publishes tmp/site/ (from `make site`) as the gh-pages branch of origin. Works in a fresh
# clone under tmp/, so the checkout and its branches are never touched.
set -eu
site=tmp/site
[ -f "$site/index.html" ] || { echo "run 'make site' first" >&2; exit 1; }
origin=$(git remote get-url origin)
source=$(git rev-parse --short HEAD)
clone=$(mktemp -d tmp/deploy.XXXXXX)
trap 'rm -rf "$clone"' EXIT

if git ls-remote --exit-code --heads origin gh-pages >/dev/null; then
  git clone --quiet --branch gh-pages "$origin" "$clone"
else
  git clone --quiet "$origin" "$clone"
  git -C "$clone" checkout --quiet --orphan gh-pages
fi
# Replace the whole tree: files the site no longer has must go.
git -C "$clone" rm -rfq --ignore-unmatch . >/dev/null
cp -R "$site"/. "$clone"/
touch "$clone/.nojekyll"
git -C "$clone" add -A
if git -C "$clone" diff --cached --quiet; then
  echo "gh-pages is already up to date"
  exit 0
fi
git -C "$clone" commit --quiet -m "Site from $source"
git -C "$clone" push origin gh-pages
