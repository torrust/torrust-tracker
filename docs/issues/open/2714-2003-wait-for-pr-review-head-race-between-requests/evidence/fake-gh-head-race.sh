#!/bin/sh
# Disposable reproduction script for the issue in this folder. It is not a maintained test.
#
# A fake `gh` that plays one specific interleaving of the poller's sequential API calls, as GitHub could
# answer them when someone pushes a commit and a reviewer reviews it while the poller is taking a snapshot:
#
#   1. The pull request is read first and reports head A.
#   2. The review list is read afterwards and, from the second snapshot on, already holds a review of
#      commit B, which was pushed after step 1.
#
# The timing is scripted, not observed on GitHub. The script is stateful: it counts the review-list calls
# in the directory named by FAKE_GH_STATE.
#
# Usage:
#   FAKE_GH_STATE=$(mktemp -d); mkdir "$FAKE_GH_STATE/bin"
#   cp fake-gh-head-race.sh "$FAKE_GH_STATE/bin/gh" && chmod +x "$FAKE_GH_STATE/bin/gh"
#   FAKE_GH_STATE=$FAKE_GH_STATE PATH="$FAKE_GH_STATE/bin:$PATH" wait-for-pr-review --pr-number 7 \
#     --poll-interval-seconds 1 --timeout-seconds 30 > result.json
HEAD_A=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
HEAD_B=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
: "${FAKE_GH_STATE:?set FAKE_GH_STATE to a writable directory}"

case "$*" in
  "auth status")
    exit 0 ;;
  "api repos/torrust/torrust-tracker/pulls/7")
    # Always head A: the pull request is read before the push in every snapshot.
    printf '{"head": {"sha": "%s"}, "state": "open", "merged": false}\n' "$HEAD_A" ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/pulls/7/reviews?per_page=100")
    calls=$(cat "$FAKE_GH_STATE/review-calls" 2>/dev/null || echo 0)
    echo $((calls + 1)) > "$FAKE_GH_STATE/review-calls"
    if [ "$calls" -eq 0 ]; then
      echo '[[]]'
    else
      printf '[[{"id": 1, "user": {"login": "da2ce7"}, "state": "COMMENTED", "commit_id": "%s", "html_url": "https://example.test/review/1", "submitted_at": "2026-11-04T10:00:00Z"}]]\n' "$HEAD_B"
    fi ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/issues/7/comments?per_page=100") echo '[[]]' ;;
  "api --paginate --slurp repos/torrust/torrust-tracker/pulls/7/comments?per_page=100") echo '[[]]' ;;
  *)
    echo "unexpected gh call: $*" >&2
    exit 1 ;;
esac
