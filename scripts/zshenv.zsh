# tpa: route Rust build artefacts to /Volumes/Space2/tpa/<checkout> when the SSD is
# mounted, else <checkout>/target-shared. Worktrees live under tpa/worktrees/.
# Install by appending this file to ~/.zshenv.
# PWD-guarded to this repo so other projects are unaffected.
if [[ "$PWD" == /Users/stefan/work/crust/tpa || "$PWD" == /Users/stefan/work/crust/tpa/* ]]; then
  root=/Users/stefan/work/crust/tpa
  [[ "$PWD" =~ '^(/Users/stefan/work/crust/tpa/worktrees/[^/]+)' ]] && root=$match[1]
  if [[ -x $root/scripts/cargo-target.sh ]]; then
    export CARGO_TARGET_DIR="$($root/scripts/cargo-target.sh)"
  fi
  unset root
fi
