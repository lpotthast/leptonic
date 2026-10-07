#!/bin/bash
# Generates baked ICU4X data for an app: only the given locales, only the data markers of the `icu_*_data` crates the
# app depends on (see documentation/build-performance.md, "ICU4X data subset").
#
# Usage: scripts/icu-datagen.sh <app-dir> <out-dir> <locales...>
#
#   <app-dir>  the app whose Cargo.lock decides the ICU4X version, e.g. examples/book-ssr.
#   <out-dir>  replaced with the generated data.
#   <locales>  icu4x-datagen locale families; `^de` is `de` without its regional variants (`de-AT`, `de-CH`, ...).
#
# The app then builds with `--cfg=icu4x_custom_data` and `ICU4X_DATA_DIR=<out-dir>`. Requires `icu4x-datagen` of the
# ICU4X version in the app's lockfile (`cargo install icu4x-datagen@<version> --locked`) and network access (it
# downloads CLDR and ICU data). Rerun it after updating the `icu_*` crates.
set -euo pipefail
APP=$1; OUT=$2; shift 2

# The ICU4X version of the app, e.g. 2.2.0.
VERSION=$(grep -A1 '^name = "icu_datetime_data"$' "$APP/Cargo.lock" | sed -nE 's/^version = "(.*)"$/\1/p')
[ -n "$VERSION" ] || { echo "no icu_datetime_data in $APP/Cargo.lock" >&2; exit 1; }
DATAGEN_VERSION=$(icu4x-datagen --version | awk '{print $2}')
if [ "$DATAGEN_VERSION" != "$VERSION" ]; then
  echo "icu4x-datagen is $DATAGEN_VERSION, the app uses ICU4X $VERSION: cargo install icu4x-datagen@$VERSION --locked" >&2
  exit 1
fi

# Fetch the dependencies, so that the data crates' sources are in the registry.
cargo fetch --manifest-path "$APP/Cargo.toml" >/dev/null
DATA_CRATES=$(cargo metadata --manifest-path "$APP/Cargo.toml" --format-version 1 \
  | python3 -c "
import json, sys
for p in json.load(sys.stdin)['packages']:
    if p['name'].startswith('icu_') and p['name'].endswith('_data') and p['version'] == '$VERSION':
        print(p['manifest_path'].rsplit('/', 1)[0])
" | sort -u)

# A crate's markers are the files of its own baked data (`data/<marker_in_snake_case>.rs.data`).
MARKERS=$(for crate in $DATA_CRATES; do ls "$crate/data"; done | sed -n 's/\.rs\.data$//p' \
  | python3 -c "
import sys
print(' '.join(''.join(part.capitalize() for part in line.strip().split('_')) for line in sys.stdin if line.strip()))
")

# Markers of unstable ICU4X features (e.g. compact decimals) need a datagen built with `unstable`; the stable crates
# don't use them. Leave each one out that datagen refuses.
while true; do
  # shellcheck disable=SC2086
  if ERRORS=$(icu4x-datagen --format baked --markers $MARKERS --locales "$@" --out "$OUT" --overwrite 2>&1); then
    break
  fi
  REFUSED=$(echo "$ERRORS" | sed -nE 's/^Error: Marker "([A-Za-z0-9]+)" requires `unstable`.*/\1/p' | head -1)
  if [ -z "$REFUSED" ]; then
    echo "$ERRORS" >&2
    exit 1
  fi
  echo "leaving out $REFUSED (unstable)" >&2
  MARKERS=$(echo " $MARKERS " | sed "s/ $REFUSED / /")
done
echo "generated ICU4X $VERSION data for $* in $OUT ($(du -sh "$OUT" | cut -f1))"
