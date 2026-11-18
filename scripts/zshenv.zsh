# tpa: route Rust build artefacts to /Volumes/Space2/tpa/target-shared when the SSD is
# mounted, else tpa/target-shared. Install by appending this file to ~/.zshenv.
# PWD-guarded to this repo so other projects are unaffected.
if [[ "$PWD" == /Users/stefan/work/crust/tpa || "$PWD" == /Users/stefan/work/crust/tpa/* ]]; then
  if [[ -x /Users/stefan/work/crust/tpa/scripts/cargo-target.sh ]]; then
    export CARGO_TARGET_DIR="$(/Users/stefan/work/crust/tpa/scripts/cargo-target.sh)"
  fi
fi
