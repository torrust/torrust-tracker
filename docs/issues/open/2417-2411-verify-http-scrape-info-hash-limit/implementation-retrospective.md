# Implementation Retrospective - Issue #2417

## Outcome

HTTP scrapes now keep the first 100 `info_hash` params, an abuse-mitigation
policy value; UDP keeps 74, computed from `MAX_PACKET_SIZE`. Each limit lives in
its protocol package with its reason, `tracker-core` has no cap, and an ADR
records the decision. Every decision and edge case is pinned by a test with
literal counts. The unified `tracker_client` no longer caps UDP scrape
arguments and warns on stderr when a tracker truncates. Binary-level tests run
the client against fake trackers, which also unblocked the monitor success-path
test deferred in refactor plan #1178.

## What Went Well

1. Reconsidering the HTTP cap from its own reason, instead of copying UDP's 74,
   produced a value and documentation that state why each limit exists.
2. Proving tests red before the fix, and mutation-checking tests that could not
   be red first, caught a test that would have passed for the wrong reason.
3. The fake trackers turned two manual checks (M2, V4) into automated,
   deterministic tests and gave the client a reusable harness.

## What Changed During Implementation

1. The UDP server reads datagrams into a `MAX_PACKET_SIZE` buffer that holds
   exactly 74 hashes, so the kernel already truncates a 75-hash datagram. A
   socket-level test could not prove the parser cap; a `handle_packet` unit test
   was added, and mutating the cap to 75 showed only that test fails.
2. The maintained client hardcoded the UDP limit (`num_args = 1..=74`), which
   blocked M2. The maintainer chose to remove it and warn on truncation instead.
3. The maintainer asked for an automated end-to-end client test rather than
   relying on manual evidence; fake trackers made that cheap, and the same
   harness covered the monitor's deferred success path.

## Root Cause

The original Warp HTTP tracker rejected scrapes above the shared limit. The Axum
rewrite dropped the check without a recorded decision, and no test referenced
the limit, so the loss went unnoticed. The shared constant also carried only
the UDP reason, which made the HTTP documentation wrong rather than merely
incomplete.

## Improvements for Future Work

1. Pin protocol limits with tests that use literal values, so a change fails a
   test and leads back to the decision record.
2. When a transport layer can produce the same observable result as the code
   under test, add a test below the transport and mutation-check it.
3. Gate commits on the pre-commit script's real exit code. Two commits on this
   branch were created after a failing hook because the gate used `;` and a
   `grep` that exited 0 on the word `FAIL`; both failures were in files outside
   the commit, and later commits fixed them.

## Avoiding Overcorrection

Not every limit needs a fake-tracker test: use binary-level tests where the
behavior is the CLI contract (stdout, stderr, exit code), and unit tests for
decisions that live in a single function.
