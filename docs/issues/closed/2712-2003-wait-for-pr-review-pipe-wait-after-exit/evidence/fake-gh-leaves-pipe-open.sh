#!/bin/sh
# Disposable reproduction script for the issue in this folder. It is not a maintained test.
#
# A fake `gh` that exits at once with success, but whose stdout and stderr stay open because a
# background `sleep` inherited them. It answers only `gh auth status`, which is the first call the
# poller makes, so the run never reaches the GitHub API.
#
# Usage: put this file, renamed to `gh`, in a directory that comes first on PATH, for example
#   mkdir -p "$TMPDIR/fake" && cp fake-gh-leaves-pipe-open.sh "$TMPDIR/fake/gh" && chmod +x "$TMPDIR/fake/gh"
#   PATH="$TMPDIR/fake:$PATH" wait-for-pr-review --pr-number 7 --timeout-seconds 3
HOLD_SECONDS="${HOLD_SECONDS:-25}"

case "$*" in
  "auth status")
    # The parentheses and the trailing `&` detach the sleeper, so this script itself exits at once.
    (sleep "$HOLD_SECONDS" &)
    exit 0 ;;
  *)
    echo "unexpected gh call: $*" >&2
    exit 1 ;;
esac
