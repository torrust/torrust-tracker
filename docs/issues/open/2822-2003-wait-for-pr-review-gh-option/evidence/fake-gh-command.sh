#!/bin/sh
dir="$(dirname "$0")"
echo "$*" >> "$dir/calls"
case "$*" in
  "api user") if [ -f "$dir/user-hangs" ]; then sleep 30; fi; if [ -f "$dir/user-fails" ]; then exit 1; fi; echo '{"login": "some-account"}' ;;
  "api repos/torrust/torrust-tracker/pulls/7") echo '{"head": {"sha": "abc1234"}, "state": "closed", "merged": true}' ;;
  "api --paginate --slurp"*) echo "[[]]" ;;
  *) echo "unexpected: $*" >&2; exit 1 ;;
esac
