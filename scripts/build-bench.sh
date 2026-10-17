#!/bin/bash
# Measures `cargo leptos build` of an app (see documentation/build-performance.md, "How to measure").
#
# Usage: scripts/build-bench.sh <app-dir> <target-name> <scenario> [cargo-leptos build args...]
#
#   <app-dir>      e.g. examples/book-ssr, or an experiment copy of it.
#   <target-name>  directory under <app-dir>/target/ to build in (a new name for `fresh`); delete it afterwards.
#   <scenario>     fresh       build (in an empty target directory: a fresh build), print the wasm size
#                  noop        build with nothing changed
#                  book-edit   change a string literal in a book page, build; change it back, build
#                  lib-edit    insert a line into a leptonic hook, build; remove it, build
#
# Edits touch the shared working tree for the duration of one build each and are always reverted. Prints one line
# per build: wall time, CPU time (user, sys) and the load average (other builds on the machine skew wall times).
# Runs on Linux and macOS (`sed -i.<suffix>`, the in-place form both GNU and BSD sed accept).
set -u
APP=$(cd "$1" && pwd); T=$2; SC=$3; shift 3
REPO=$(cd "$(dirname "$0")/.." && pwd)
BOOKFILE=$REPO/examples/book-ssr/src/pages/documentation/atoms/button.rs
LIBFILE=$REPO/leptonic/src/hooks/button/use_button.rs
cd "$APP" || exit 1
export CARGO_TARGET_DIR=$APP/target/$T LEPTOS_SITE_ROOT=$APP/target/$T/site
LOGDIR=$CARGO_TARGET_DIR/bench-logs
mkdir -p "$LOGDIR"

now() { python3 -c 'import time; print(time.time())'; }

# In-place sed: GNU sed takes `-i` without a suffix, BSD sed needs one; both accept an attached suffix.
sed_in_place() { local file=${*: -1}; sed -i.bench-sed "$@" && rm -f "$file.bench-sed"; }

run() {
  local label=$1; shift
  local start; start=$(now)
  /usr/bin/time -p cargo leptos build "$@" > "$LOGDIR/$label.log" 2>&1
  local rc=$?
  local end; end=$(now)
  echo "$T $label rc=$rc wall=$(python3 -c "print(round($end - $start, 1))")" \
    "$(grep -E '^(user|sys)' "$LOGDIR/$label.log" | tr '\n' ' ')" \
    "load=$(uptime | awk -F'load averages?: ' '{print $2}')"
  grep -E "^error" "$LOGDIR/$label.log" | head -3
}

# An edited file is restored however the script ends (an error, Ctrl-C, a closed output pipe), not only after the
# edit's build: a killed run must not leave the edit in the shared tree.
EDITED=""
restore_edit() { if [ -n "$EDITED" ]; then cp "$LOGDIR/edit.bak" "$EDITED"; EDITED=""; fi; }
trap restore_edit EXIT
trap 'exit 1' INT TERM HUP PIPE
edit() { # <file> <sed expression>
  cp "$1" "$LOGDIR/edit.bak"; EDITED=$1
  sed_in_place "$2" "$1"
  if cmp -s "$1" "$LOGDIR/edit.bak"; then echo "the edit doesn't apply to $1 any more: update this script" >&2; exit 2; fi
}

case $SC in
  fresh)
    run fresh "$@"
    ls -l "$CARGO_TARGET_DIR"/site/pkg/*.wasm | awk '{print "wasm bytes", $5, $9}' ;;
  noop) run noop "$@" ;;
  book-edit)
    edit "$BOOKFILE" 's/<DocPage title="Button Atom">/<DocPage title="Button Atom (bench)">/'
    run book-edit "$@"
    restore_edit
    run book-revert "$@" ;;
  lib-edit)
    edit "$LIBFILE" 's/^pub fn use_button(input: UseButtonInput) -> UseButtonReturn {$/&\
    let _bench = 0u8;/'
    run lib-edit "$@"
    restore_edit
    run lib-revert "$@" ;;
  *) echo "unknown scenario: $SC" >&2; exit 2 ;;
esac
