#!/usr/bin/env bash
# Fetches the latest disposable_email_blocklist.conf from the upstream
# disposable-email-domains/disposable-email-domains repo, vendors it into
# data/, and records the upstream commit it came from.
#
# Exits 0 always; sets the GitHub Actions output `changed=true|false` (or
# prints CHANGED=true|false when run outside Actions) so the calling
# workflow can decide whether to bump the crate version and publish.
#
# Usage: scripts/update_domains.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_FILE="$REPO_ROOT/data/disposable_email_blocklist.conf"
COMMIT_FILE="$REPO_ROOT/data/SOURCE_COMMIT"

UPSTREAM_API="https://api.github.com/repos/disposable-email-domains/disposable-email-domains/commits/main"
UPSTREAM_RAW="https://raw.githubusercontent.com/disposable-email-domains/disposable-email-domains/main/disposable_email_blocklist.conf"

echo "Fetching latest upstream commit metadata..." >&2
latest_commit="$(curl -fsSL "$UPSTREAM_API" | python3 -c 'import json,sys; print(json.load(sys.stdin)["sha"])')"

if [[ -z "$latest_commit" ]]; then
  echo "error: could not determine latest upstream commit" >&2
  exit 1
fi

previous_commit="$(cat "$COMMIT_FILE" 2>/dev/null || echo "")"

echo "Fetching latest domain list..." >&2
tmp_file="$(mktemp)"
curl -fsSL "$UPSTREAM_RAW" -o "$tmp_file"

new_lines="$(wc -l < "$tmp_file" | tr -d ' ')"
if [[ "$new_lines" -lt 1000 ]]; then
  echo "error: fetched list only has $new_lines lines, refusing (possible bad fetch)" >&2
  rm -f "$tmp_file"
  exit 1
fi

if diff -q "$tmp_file" "$DATA_FILE" > /dev/null 2>&1; then
  echo "No changes to domain list (upstream commit still $previous_commit)." >&2
  rm -f "$tmp_file"
  changed=false
else
  echo "Domain list changed. Updating vendored copy." >&2
  mv "$tmp_file" "$DATA_FILE"
  echo "$latest_commit" > "$COMMIT_FILE"
  changed=true
fi

if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "changed=$changed" >> "$GITHUB_OUTPUT"
  echo "upstream_commit=$latest_commit" >> "$GITHUB_OUTPUT"
else
  echo "CHANGED=$changed"
  echo "UPSTREAM_COMMIT=$latest_commit"
fi
