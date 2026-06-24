#!/usr/bin/env bash
set -euo pipefail

# Fail if a shell script advertises direct execution with a shebang but is not
# executable. Shell fragments that are only sourced should not have shebangs.
fail=0
while IFS= read -r -d '' script; do
  if head -n 1 "$script" | grep -q '^#!'; then
    if [ ! -x "$script" ]; then
      echo "Missing executable bit for shebang script: ${script#./}"
      fail=1
    fi
  fi
done < <(find . -type f -name '*.sh' -not -path './.git/*' -print0 | sort -z)

if [ "$fail" -ne 0 ]; then
  echo "Fix with: chmod +x <path> (or remove the shebang from sourced-only fragments)." >&2
  exit 1
fi
