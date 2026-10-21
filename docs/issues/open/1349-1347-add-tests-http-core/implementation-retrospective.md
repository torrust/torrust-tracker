---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/open/1349-1347-add-tests-http-core/lessons.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/implementation-retrospective.md
---

# Implementation Retrospective — Issue #1349 Add Tests to the HTTP Core Package

## Purpose

Record evidence-based process improvements discovered while implementing #1349. This is a blameless review of the
implementation approach; it does not replace the acceptance verification in [ISSUE.md](ISSUE.md).

## Outcome

All 13 source files of `packages/http-core/src/` were processed through the per-file workflow in seven pull
requests (the specification and the inventory, then four test slices and this final one). Unit-only lines went from
1,751 / 1,970 (88.88%) to 3,030 / 3,052 (99.28%), regions from 90.41% to 99.25% and functions from 84.29% to 99.74%;
tests from 33 to 191 (the totals include the new test modules). Seven production lines stay uncovered, each with a
recorded owner ([coverage-evidence.md](coverage-evidence.md)). A whole-package mutation run gave 50 caught, 1 missed,
1 timeout and 44 unviable mutants ([mutation-evidence.md](mutation-evidence.md)). The manual run of M1-M5 against a
built tracker passed ([manual-verification-evidence.md](manual-verification-evidence.md)). Production code changed
only by one corrected comment (`services/mod.rs`) and the removal of a dead test helper (`event.rs`, S4). One finding
needs a decision outside this issue (the `tcp4`/`tcp6` counters, see below).

## What Went Well

1. **Slicing made the code reviews small and fast.** The #1348 pull request (#2538) added 5,407 lines over 50 files
   and 53 commits and drew 18 review submissions. The largest #1349 slice added 2,034 lines over 16 files (S5, 38% of
   it), and the four code slices (#2575, #2586, #2593, #2608) merged within about 30 minutes to 2 hours of opening,
   with 1, 1, 3 and 5 review submissions. The cost moved rather than vanished: the two documentation-only pull
   requests (#2546, #2558) took about 15 and 75 hours and drew 26 and 21 review submissions, because they carried the
   specification and the process rules for all later slices.
2. **An independent review per slice found real survivors every time.** Label tables in S4; the recorded time and
   unit, the spawned listener's wiring and an error-swallowing path in S5. The #1348 plan of one late review would
   have found them all at once, in a much larger diff.
3. **The prose-first AAA table per plan kept tests readable** and gave the reviewer one place to challenge a test's
   intent.
4. **Named mutations in each plan, with the exact revert checked by a diff,** killed weak tests early; the final
   cargo-mutants run then found only one survivor, the debug log, which the handler plan had already decided.
5. **Pinning a behavior that looks wrong, and handing the question off, kept the tests honest.** The `tcp4`/`tcp6`
   test documents what the code does, so a future decision must update it knowingly.

## What Changed During Implementation

- **`develop` moved under the work.** The S3 merge brought #2414 into `statistics/event/listener.rs`; the S4-start
  record had said `http-core` was unchanged and had to be corrected with a re-measured baseline. Every slice now
  starts by diffing the package against the previous slice's head.
- **Three plans called a property unobservable and were wrong.** The recorded sample time and metric unit are
  visible through `Metrics`' `PartialEq`; seven mutants survived until the reviewer showed it.
- **A helper needed its own tests.** `RecordingSender` documented "records only when polled", which no service test
  checked; a mutant recording at call time passed all 183 tests.
- **The same documentation defect exists for UDP.** #1348 recorded that the `udp4_*`/`udp6_*` docs say "from IPv4/IPv6
  peers" while the code selects on the server binding; nobody took it up, and #1349 found the HTTP twin. The finding
  procedure produced a draft (`.tmp/1349-tcp-family-counter-docs/`, covering both) instead of two parallel tickets.
- **Records slipped under parallel edits.** A staging race lost log entries in S5, a guessed time needed a
  correction in S6, and over-long `old_str` edits deleted the adjacent text several times. All were repaired and
  disclosed in the records.

## Root Cause

- The per-file plan asked "what does this module decide?" but not "what do the types already let me compare?", so
  claims that a property cannot be observed were accepted without checking derives.
- A shared test helper is used by many tests, so its own contract looks "covered"; coverage cannot show that its
  documented property is unpinned.
- The slice rule said to start from the merged records, but did not say to re-diff the package, which the first
  re-slicing cost showed was necessary.
- Edit hygiene was left to individual care; the failure mode (replacement shorter than the replaced text) is
  mechanical and repeats.

## Improvements for Future Work

1. **Plan template:** add to the "Coverage and decision analysis" guidance: before recording a property as
   unobservable, list the type's derives (`PartialEq`, `Clone`, `Debug`) and public comparison paths.
2. **Plan template or guidelines:** require a test of any shared test double's documented contract, with a mutation
   that breaks exactly that contract.
3. **Slice workflow:** at each slice start, diff the package against the previous head and record the result in the
   start entry (done informally here from S4).
4. **Metrics wording:** decide the meaning of the `tcp4`/`tcp6` and `udp4`/`udp6` counters once, for both transports
   (the draft in [lessons.md](lessons.md)). Until then, any package that adds family counters will copy the
   "peers" comment.
5. **Mutation runs:** for a package of about a hundred mutants, run the whole package rather than a sample; it took
   about 14 minutes, and the 44 unviable mutants were explained by 4 reasons.
6. **`tracker_client`:** `http scrape` should reject an announce URL, or say that it needs the base URL; the wrong URL
   produced a misleading "missing param peer_id".

## Avoiding Overcorrection

- Do not require a pull request per file; a slice of two to five related files kept the review load reasonable.
- Do not require mutation runs per slice. One whole-package run at the end, after the per-file named mutations, found
  almost nothing new and was cheap enough to keep as a closing check only.
- Do not add property tests by default: the announce assessment fixed its two survivors with examples.
- Do not promote the disposable-run steps of the manual verification to a script; M1-M5 take under two minutes by
  hand with `tracker_client` and `curl`.

## Evidence

- [ISSUE.md](ISSUE.md): ledger, slice table, progress log, acceptance review.
- [coverage-evidence.md](coverage-evidence.md), [mutation-evidence.md](mutation-evidence.md),
  [manual-verification-evidence.md](manual-verification-evidence.md).
- [test-refactor-plans/](test-refactor-plans/README.md): one plan per file with mutation proofs.
- [lessons.md](lessons.md): follow-up candidates and testing lessons.
- [agent-review-reports.md](agent-review-reports.md): one independent review per slice.
- Pull requests #2538 (the #1348 baseline for comparison), #2546, #2558, #2575, #2586, #2593 and #2608.
