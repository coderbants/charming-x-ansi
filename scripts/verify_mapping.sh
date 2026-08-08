#!/usr/bin/env bash
# Verifies that UPSTREAM_MAPPING.md accounts for every file in upstream-go/.
set -u

cd "$(dirname "$0")/.."
MAPPING=UPSTREAM_MAPPING.md
fail=0

while IFS= read -r f; do
  case "$f" in
    ansi/*.go)
      base="$(basename "$f")"
      if ! grep -qF "$base" "$MAPPING" && ! grep -qF "$f" "$MAPPING"; then
        echo "MISSING (.go): $f"
        fail=1
      fi
      ;;
    ansi/*)
      base="$(basename "$f")"
      if ! grep -qF "$base" "$MAPPING" && ! grep -qF "ansi/$f" "$MAPPING"; then
        echo "MISSING (ansi support): $f"
        fail=1
      fi
      ;;
    *)
      # Non-ansi monorepo files: covered by the Out-of-Scope section which
      # names each module directory.
      top="${f%%/*}"
      if ! grep -qF "$top" "$MAPPING"; then
        echo "MISSING (monorepo module): $top ($f)"
        fail=1
      fi
      ;;
  esac
done < <(cd upstream-go && git ls-files)

if [ "$fail" -eq 0 ]; then
  echo "OK: every upstream file is accounted for in $MAPPING"
fi
exit "$fail"
