#!/bin/bash
# Bakes the ICU4X data of the given locales for an app (see documentation/build-performance.md, "ICU4X data subset").
# The app builds with it through `--cfg=icu4x_custom_data` and `ICU4X_DATA_DIR=<out-dir>`. The data must come from
# the ICU4X version the app uses: rerun this after updating the `icu_*` crates.
#
# Usage: scripts/icu-datagen.sh <app-dir> <out-dir> <locales...>
#
#   <app-dir>  the app whose Cargo.lock decides the ICU4X version, e.g. examples/book-ssr.
#   <out-dir>  replaced with the generated data.
#   <locales>  icu4x-datagen locale families; `^de` is `de` without its regional variants (`de-AT`, `de-CH`, ...).
#
# Needs network access (it downloads CLDR and ICU data).
set -euo pipefail
APP=$1; OUT=$2; shift 2

# The app's ICU4X minor version (e.g. 2.3): data formats change between minor versions.
VERSION=$(sed -n '/^name = "icu_provider"$/{n;s/^version = "\([0-9]*\.[0-9]*\).*"$/\1/p;}' "$APP/Cargo.lock")
if [[ "$(icu4x-datagen --version | head -1)" != "icu4x-datagen $VERSION."* ]]; then
  echo "needs icu4x-datagen $VERSION: cargo install icu4x-datagen@$VERSION --locked" >&2
  exit 1
fi

# All markers: the linker drops data the app doesn't use. No segmenter models (large): leptonic doesn't segment
# text.
icu4x-datagen --format baked --markers all --segmenter-models none --locales "$@" --out "$OUT" --overwrite
