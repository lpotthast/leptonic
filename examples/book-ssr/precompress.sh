#!/bin/bash
function compress {
    local filename=$1

    gzip --force --keep -9 "${filename}"
    brotli --force --keep --quality=11 "${filename}"
    zstd --force --keep --ultra --no-progress --format=zstd -22  "${filename}"
}

root=$1

for filename in ${root}/js/*.js; do
    compress "${filename}"
done

for filename in ${root}/pkg/*.css; do
    compress "${filename}"
done
for filename in ${root}/pkg/*.js; do
    compress "${filename}"
done
for filename in ${root}/pkg/*.wasm; do
    compress "${filename}"
done
