#!/bin/bash
function compress {
    local filename=$1

    gzip --force --keep -9 "${filename}"
    brotli --force --keep --quality=11 "${filename}"
    zstd --force --keep --ultra --no-progress --format=zstd -22  "${filename}"
}

root=$1

while IFS= read -r -d '' filename; do
    compress "${filename}"
done < <(find "${root}/pkg" -type f \( -name '*.css' -o -name '*.js' -o -name '*.wasm' \) -print0)
