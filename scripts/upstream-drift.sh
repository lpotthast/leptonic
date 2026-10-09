#!/usr/bin/env bash
# Report react-spectrum commits that leptonic has not absorbed yet.
#
# Every leptonic file that ports react-aria / react-stately code, or mirrors react-spectrum tests,
# declares its upstream files in top-level lines at its start:
#
#   // Upstream: react-aria/src/interactions/usePress.ts @ 6f664fe911
#   // Upstream: react-aria/test/interactions/usePress.test.js @ 6f664fe911
#
# Upstream test files count like sources: a native test module (`mod tests`) names the test files
# its cases mirror in its source file's header, a browser test file in its own. The path is relative
# to react-spectrum's `packages/` directory (the atom theme's stylesheets name `../starters/docs/src/...`),
# the hash is the react-spectrum commit the file was last synced against. This script lists, per
# leptonic file, the upstream commits touching those files since then.
#
# Usage:
#   scripts/upstream-drift.sh [-v] [PATH_FILTER]   Report drift (most drifted first). -v lists the commits.
#   scripts/upstream-drift.sh --mark-synced FILE... Set the sync point of FILE(s) to react-spectrum's HEAD.
#
# Environment:
#   REACT_SPECTRUM   Path to a react-spectrum checkout (default: ../react-spectrum relative to the repo root).
#   IGNORE_PATTERN   Extended regex matched against commit subjects; matching commits are not counted
#                    (default: repo-wide reformatting, lint migrations and releases, which never change behavior).
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RS="${REACT_SPECTRUM:-$ROOT/../react-spectrum}"
IGNORE_PATTERN="${IGNORE_PATTERN:-oxfmt|oxlint|[Ff]ormat with|[Pp]ublish}"

if [[ ! -d "$RS/.git" ]]; then
  echo "react-spectrum checkout not found at '$RS' (set REACT_SPECTRUM)." >&2
  exit 1
fi

if [[ "${1:-}" == "--mark-synced" ]]; then
  shift
  head_sha="$(git -C "$RS" rev-parse --short=10 HEAD)"
  for file in "$@"; do
    sed -i -E "s|^(// Upstream: [^ ]+ @ )[0-9a-f]+$|\1$head_sha|" "$file"
    echo "$file -> $head_sha"
  done
  exit 0
fi

verbose=0
if [[ "${1:-}" == "-v" ]]; then
  verbose=1
  shift
fi
filter="${1:-}"

report=""
while IFS= read -r file; do
  [[ -n "$filter" && "$file" != *"$filter"* ]] && continue
  total=0
  details=""
  while IFS= read -r line; do
    upstream="$(sed -E 's|^// Upstream: ([^ ]+) @ ([0-9a-f]+)$|\1|' <<<"$line")"
    sha="$(sed -E 's|^// Upstream: ([^ ]+) @ ([0-9a-f]+)$|\2|' <<<"$line")"
    commits="$(git -C "$RS" log --format='%h %as %s' "$sha..HEAD" -- "packages/$upstream" \
      | grep -Ev "^[0-9a-f]+ [0-9-]+ .*($IGNORE_PATTERN)" || true)"
    count=0
    [[ -n "$commits" ]] && count="$(wc -l <<<"$commits")"
    total=$((total + count))
    if [[ $verbose -eq 1 && $count -gt 0 ]]; then
      details+="    $upstream ($count since $sha)"$'\n'
      details+="$(sed 's/^/      /' <<<"$commits")"$'\n'
    fi
  done < <(grep -E '^// Upstream: [^ ]+ @ [0-9a-f]+$' "$file")
  if [[ $total -gt 0 ]]; then
    report+="$total"$'\t'"${file#"$ROOT"/}"$'\n'
    [[ $verbose -eq 1 ]] && report+="$details"
  fi
done < <(grep -rlE '^// Upstream: ' "$ROOT/leptonic/src" "$ROOT/leptonic/tests" "$ROOT/leptonic-theme/scss" | sort)

if [[ -z "$report" ]]; then
  echo "No upstream drift."
  exit 0
fi

if [[ $verbose -eq 1 ]]; then
  printf '%s' "$report"
else
  printf '%s' "$report" | sort -rn | awk -F'\t' '{ printf "%4d  %s\n", $1, $2 }'
fi
