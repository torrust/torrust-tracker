---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2320-review/review-retrospective.md
    - docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2320 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2320>.

This record was first committed at 18:32 UTC on 2026-09-23, after four review rounds had been
processed as GitHub threads without the `process-pr-review` workflow (see `review-retrospective.md`,
cost item 8). Its first version said all fourteen threads were resolved; that was false when
committed, because round 5 (18:26 UTC) had already opened two more. The record is extended each
round; the Findings table is current as of the round named in the last Processing Log entry, and
the Processing Log is append-only, so entries known to be wrong are followed by a dated
correction rather than rewritten. Every `Current-tree verification` was re-run against the tree
when its entry was written, not copied from a reply.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Audit IDs `F1`-`F4` are the Copilot round (no reviewer IDs). The human reviewer numbered their
findings `F1`-`F17` across rounds 1-7; those collide with the Copilot IDs and are recorded here as
`F5`-`F21` (reviewer `F<k>` is audit `F<k+4>`), with the reviewer's original ID in each detail entry.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2320-f1` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2320-f2` | Copilot | Minor (inferred) | testing | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2320-f3` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2320-f4` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2320-f5` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2320-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2320-f7` | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2320-f8` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2320-f9` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2320-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2320-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2320-f12` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2320-f13` | Human | Minor | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F14 | `review-finding:pr-2320-f14` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2320-f15` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2320-f16` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2320-f17` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | `review-finding:pr-2320-f18` | Human | Minor | documentation | RE_RAISE_OF:F15 | FIXED | RESOLVED |
| F19 | `review-finding:pr-2320-f19` | Human | Minor | documentation | RE_RAISE_OF:F16 | FIXED | RESOLVED |
| F20 | `review-finding:pr-2320-f20` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | `review-finding:pr-2320-f21` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Benchmark connection cookie can expire during a longer Criterion run

- PR number: 2320
- Source review ID: 5291390450
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4082767037>
- Concern: the benchmark built one `connection_id` at `issue_time` and validated it strictly
  against a ±60 s range, so a longer measurement time would start benchmarking the error path.
- Solution: widened the benchmark-only range to ±`BENCHMARK_COOKIE_VALIDITY_SECS` (24 h) around
  `issue_time`; `Strict` validation kept, since `check` is two float comparisons and inert for
  timing.
- Current-tree verification: `rg -n BENCHMARK_COOKIE_VALIDITY_SECS packages/udp-server/benches/udp_tracker_server_benchmark.rs`
  matches line 27 (constant) and line 82 (`valid_range`).
- Resolution reference: `fix(udp-server): address scrape review feedback`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085874031>

### F2 - Duplicate of F1 on the `valid_range` line

- PR number: 2320
- Source review ID: 5291390450
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4082767079>
- Concern: identical text to F1, anchored on the `valid_range` line rather than the
  `connection_id` line.
- Solution: no separate change; the F1 change covers both anchors.
- Current-tree verification: same as F1; the two anchored lines are the constant and its single use.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085874270>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085874270>

### F3 - Lookup key built from a temporary conversion inside the loop

- PR number: 2320
- Source review ID: 5291390450
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4082767128>
- Concern: `scrape_data.files.get(&info_hash.0.into())` is harder to read and brittle if the
  conversion changes.
- Solution: bind the converted key once per iteration and look it up by name.
- Current-tree verification: `packages/udp-server/src/handlers/scrape.rs:113` reads
  `let info_hash = info_hash.0.into();` followed by `.get(&info_hash)`.
- Resolution reference: `fix(udp-server): address scrape review feedback`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085874482>

### F4 - Seeder-count literals rely on inference to become `u8`

- PR number: 2320
- Source review ID: 5291390450
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4082767157>
- Concern: `[8, 3, 6, 1, 7, 2, 5, 4]` matches the helper's `u8` parameter only by inference.
- Solution: type the first literal, `[8u8, 3, 6, 1, 7, 2, 5, 4]`.
- Current-tree verification: `packages/udp-server/src/handlers/scrape.rs:438` contains
  `.zip([8u8, 3, 6, 1, 7, 2, 5, 4])`.
- Resolution reference: `fix(udp-server): address scrape review feedback`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085874758>

### F5 - P1 "Code Under Test" commit contains no benchmark

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073038>
- Concern: the P1 row named `e03b22f7` as containing the P0 microbenchmark; that commit is the
  #2315 spec merge and has no `packages/udp-server/benches/`, so the baseline could not have been
  measured there as described.
- Solution: the row now states P1 ran on `e03b22f7` plus the then-uncommitted P0 benchmark and
  names the published patch with the same scrape-path code state by its commit subject (the
  round-2 fix named it by branch id; F13 replaced that with the subject).
- Current-tree verification: `scrape-benchmark-evidence.md:51` reads "plus the then-uncommitted
  P0 microbenchmark … `perf(udp-server): benchmark scrape response handling`".
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084382101>

### F6 - V2 artifact commit not reachable from the PR

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073058>
- Concern: `aef07ca4` was a pre-rebase local commit orphaned by the first rebase; a reader
  cannot resolve it.
- Solution: the artifact line now says it was a pre-rebase local commit and names the published
  fix by subject (the round-2 fix named it by branch id; F13 replaced that).
- Current-tree verification: `manual-verification-evidence.md:161-163` reads "a pre-rebase local
  commit (since orphaned by rebases onto `develop`); the published patch carrying the same fix is
  `fix(udp-server): preserve scrape response order`".
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084382392>

### F7 - Non-latest `criterion` added without a recorded rationale

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073067>
- Concern: `criterion 0.5.1` was added while `0.8.2` is already in `Cargo.lock`; Engineering
  Policy 1 requires the reason for a non-latest version to be written down.
- Solution: manifest comment above the entry stating the pin matches the `http-core` and
  `udp-core` benches and should be upgraded with them.
- Current-tree verification: `packages/udp-server/Cargo.toml:50` reads
  `# Pinned to match the http-core and udp-core benches; upgrade all three together.`
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084382629>

### F8 - End-to-end load test did not use the spec's scrape-heavy mix

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073085>
- Concern: the run used the generic `docs/benchmarking.md` config (`weight_scrape = 1`, 10 hashes)
  instead of the spec's `weight_scrape = 80`, `scrape_max_torrents = 74`; the deviation, the P2
  reset procedure, and the missing P2 connect/announce breakdown were unrecorded.
- Solution: recorded all three in Anomalies, with why the microbenchmark is the load-bearing
  instrument and that a rerun with the spec's mix would sharpen the end-to-end leg. No rerun.
- Current-tree verification: `scrape-benchmark-evidence.md:209` begins "The load-test config used
  for both phases is the generic UDP config from `docs/benchmarking.md`".
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084382909>

### F9 - Post-fix measurements committed before the fix

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073096>
- Concern: P1 and P2 results both land in the benchmark commit, against the spec's Commit Points;
  the P2 row said "working tree" so no commit could be checked out for it.
- Solution: recorded the ordering in Anomalies (history kept per `AGENTS.md`) and named the
  published post-fix patch by subject in the P2 row.
- Current-tree verification: `scrape-benchmark-evidence.md:217` begins "The P1 and P2 results
  were committed together in the `perf(udp-server): benchmark scrape response handling` patch".
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084383149>

### F10 - "at most 1/8!" overstates `HashMap` guarantees

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073102>
- Concern: `HashMap` iteration order is unspecified, not a uniform permutation, so `1/N!` is a
  heuristic, not an upper bound.
- Solution: "about `1/8!`", with the caveat stated and a note that the two sibling regressions are
  deterministic.
- Current-tree verification: `manual-verification-evidence.md:267` reads "probability about
  $1/8! = 1/40320$".
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084383434>

### F11 - Templated checkpoint text narrowed

- PR number: 2320
- Source review ID: 5292925383
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084073117>
- Concern: "pre-push checks" was dropped from the Automatic verification checkpoint when it was
  checked.
- Solution: restored the templated wording.
- Current-tree verification: `ISSUE.md:403` reads
  ``- [x] Automatic verification completed (`linter all`, relevant tests, pre-push checks)``.
- Resolution reference: `docs(issues): address #2320 review findings on scrape evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084383673>

### F12 - "the same code state" is exact for the benchmark, not the tree

- PR number: 2320
- Source review ID: 5293432174
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4084509719>
- Concern: the P1/P2 rows claimed the named patches had "the same code state" as the measured
  tree; only the scrape path is byte-identical, the trees differ by #2316.
- Solution: both rows now say "the same scrape-path code state".
- Current-tree verification: `rg -n 'scrape-path code state' scrape-benchmark-evidence.md`
  matches lines 51 and 52.
- Resolution reference: `docs(issues): cite scrape evidence patches by subject, not branch id`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085561417>

### F13 - Evidence names commit ids that the rebase rewrote

- PR number: 2320
- Source review ID: 5294414950
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085306190>
- Concern: the F5/F6 fix named branch commits by id; the rebase between rounds 2 and 3 rewrote
  them, so five lines pointed at ids unreachable from `develop`. Same defect class as F5 and F6.
- Solution: all five sites name the patch by Conventional Commit subject with a note under the
  Code Under Test table; the author-side rule was added to the evidence template, `fix-bug`, and
  `open-pull-request` in a separate ride-along commit.
- Current-tree verification: `rg -n 'b6619256|d38c1d2c|aef07ca4|a2e99d36|dc3c85f1' docs/issues/open/2314-preserve-udp-scrape-response-order/`
  returns nothing; the only repository ids in the folder are `60a4a160` and `e03b22f7`, both
  `develop` commits.
- Resolution reference: `docs(issues): cite scrape evidence patches by subject, not branch id`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085561683>

### F14 - New pre-push check cannot be cleared on its own reference folder

- PR number: 2320
- Source review ID: 5294842958
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085653178>
- Concern: run as written, the grep returned 20 hits on the #2314 folder and the disposition list
  covered 10; `[0-9a-f]{7,40}` also matches seven-digit decimals, and info-hashes and recorded ids
  had no category. An info-hash and a git id are indistinguishable by pattern.
- Solution: filter purely decimal tokens with `grep '[a-f]'`, add "non-commit data" as a fourth
  disposition, and state that the PR body and review replies are outside the grep.
- Current-tree verification: the command at `open-pull-request/SKILL.md:66` run on the #2314
  folder returns seven tokens (`48a229cea`, `60a4a160`, `822a801c…`, `aaaa…`, `bbbb…`,
  `e03b22f7`, `e03b22f7…`), each with a listed disposition.
- Resolution reference: `docs(skills): make the branch-id pre-push check clear its reference case`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085766408>

### F15 - Three retrospective timeline rows contradicted by git and API timestamps

- PR number: 2320
- Source review ID: 5295088674
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085863328>
- Concern: the retrospective dated its own first commit 18:16 when `git log` says 18:08, placed
  events "after 18:16" that all occurred by 18:15, placed the human-thread resolution after
  17:52 when round 4 at 18:02 shows the threads still open, and attributed that resolution to a
  maintainer request the GitHub record does not contain.
- Solution: rows rewritten with git author times and GitHub `created_at` minutes; the resolution
  row is placed after 18:02 and states the request came from the editor chat session. Recorded as
  cost item 9 and lesson 10 in the retrospective. A fresh instance (a row written as the editing
  time rather than the event time) was caught and corrected before this head shipped.
- Current-tree verification: `rg -n '^\| (18:08|18:13-18:15|after 18:02|19:43) ' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  matches four rows; `rg -n '18:16|after 17:52|18:40' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  matches only the cost-item and row text that quote the old values as wrong.
- Resolution reference: `docs(pr-reviews): source #2320 retrospective times and counts from git and the API`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086635319>

### F16 - Retrospective commit count one short at the head that ships it

- PR number: 2320
- Source review ID: 5295088674
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4085863338>
- Concern: "Commits on the branch | 11 (… 1 retrospective)" while `git log a110200d..HEAD`
  returned 12 at that head; the count omitted the commit that wrote the line.
- Solution: the row now states the command, the value it returned at the commit that writes the
  row (15), the breakdown, and that later audit-record commits are not counted, so it does not
  rot when this record is committed after it.
- Current-tree verification: `rg -n 'returned 15 at the commit that writes this row' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  matches line 46; `git rev-list --count torrust/develop..HEAD` at the retrospective fix commit
  was 15.
- Resolution reference: `docs(pr-reviews): source #2320 retrospective times and counts from git and the API`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086635594>

### F17 - Record claimed a completeness it did not have when committed

- PR number: 2320
- Source review ID: 5295966158
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086608965>
- Concern: the preamble said all fourteen threads were resolved when the record was committed at
  18:32, but round 5 (18:26) had opened two unresolved threads; the record had 16 threads to
  cover and rows for 14. Re-raised at 20:24 when the next push left the sentence unchanged while
  the table had grown to 16 rows against 20 threads.
- Solution: preamble rewritten to state when the record was first committed, that its first
  version was wrong, and that the table is current as of the last Processing Log entry; rows
  added for every reviewer finding through F17.
- Current-tree verification: `rg -c '^\| F[0-9]+ \|' docs/pr-reviews/pr-2320-review/PR-REVIEW.md`
  returns 21; the GraphQL `reviewThreads` connection returns 21 threads at the time of writing.
- Resolution reference: `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4087461114>

### F18 - Two Processing Log entries do not match the events they record

- PR number: 2320
- Source review ID: 5295966158
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086608973>
- Concern: the 17:52 entry says F5-F13 were resolved then "at the maintainer's request", but
  round 4 at 18:02 records them open and no GitHub comment contains a request; the 18:18 entry
  says the Copilot replies were posted "now" but they are stamped 18:27; round 5 (18:26) had no
  entry. Re-raised at 20:24 when the push left the entries unchanged while the retrospective's
  timeline had been corrected, so the two files disagreed. Same claim as F15 in this record.
- Solution: the Processing Log is append-only, so the entries stand and a dated correction entry
  names each wrong clause and gives the sourced value; an entry for review 5295088674 exists.
- Current-tree verification: `rg -n '^- 2026.*UTC - Correction:' docs/pr-reviews/pr-2320-review/PR-REVIEW.md`
  matches one Processing Log entry naming the 17:52 and 18:18 clauses; `rg -n '18:26 UTC - Human review 5295088674'`
  matches one entry.
- Resolution reference: `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4087461329>

### F19 - Retrospective Review Summary one round stale again

- PR number: 2320
- Source review ID: 5295966158
- Reviewer finding ID: F15
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086608977>
- Concern: the summary rows (rounds, findings, blocking, processed-through-skill, preamble ID
  range) and the timeline were refreshed at 19:43 and stale again at 19:46; the push at 20:12
  shipped them without a re-check. Same drift as F16 in this record.
- Solution: values recomputed from the `reviews` and `reviewThreads` connections at the time of
  writing; a "current as of" sentence added at the top of the retrospective naming the last round
  it covers, so later rounds do not make it wrong.
- Current-tree verification: `rg -n '^Current as of' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  matches one line; the `Review rounds` row reads `8 (7 human, 1 Copilot)` and the `Findings` row
  reads `21`, matching seven `da2ce7` reviews with bodies plus Copilot, and 21 threads.
- Resolution reference: `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4087461668>

### F20 - Root cause 7 attributed the author's own words to the reviewer

- PR number: 2320
- Source review ID: 5295966158
- Reviewer finding ID: F16
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086608989>
- Concern: root cause 7 quoted "without the `process-pr-review` skill" as the reviewer's round-2
  observation; review 5293432174 contains no such string. The words are the retrospective's own
  first-version preamble.
- Solution: the quotation and attribution removed; the sentence now says no review flagged the
  missing record and the gap was first written down by the retrospective's first version.
- Current-tree verification: `rg -n 'round-2 body observed' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  returns nothing; `rg -n 'first written down by this retrospective'` matches root cause 7.
- Resolution reference: `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4087461948>

### F21 - "the commit that writes this row" moved when the row was edited

- PR number: 2320
- Source review ID: 5296377033
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4086947986>
- Concern: the commit-count row anchored its value to "the commit that writes this row"; a later
  commit edited the row, so the anchor moved to a commit where the count is 16. The trailing
  exclusion clause rescued it, but only for a reader who classifies the editing commit. Advisory.
- Solution: the row names the commit by subject.
- Current-tree verification: `rg -n 'returned 15 at .docs.pr-reviews.: source #2320' docs/pr-reviews/pr-2320-review/review-retrospective.md`
  matches the `Commits on the branch` row.
- Resolution reference: `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4087462247>

## Processing Log

- 2026-09-23 13:09 UTC - Copilot review 5291390450 submitted (F1-F4).
- 2026-09-23 13:25 UTC - `fix(udp-server): address scrape review feedback` authored (F1, F3, F4).
- 2026-09-23 15:16 UTC - Human review 5292925383 submitted, `CHANGES_REQUESTED` (F5-F11).
- 2026-09-23 15:44 UTC - `docs(issues): address #2320 review findings on scrape evidence` authored (F5-F11).
- 2026-09-23 15:47 UTC - Replies posted on F5-F11.
- 2026-09-23 16:00 UTC - Human review 5293432174 submitted, `APPROVED` (F12 advisory).
- 2026-09-23 17:24 UTC - Human review 5294414950 submitted, `CHANGES_REQUESTED` (F13), after the rebase onto `a110200d`.
- 2026-09-23 17:49 UTC - `docs(issues): cite scrape evidence patches by subject, not branch id` and `docs(skills): forbid branch commit ids in mergeable evidence` authored (F12, F13).
- 2026-09-23 17:52 UTC - Replies posted on F12, F13; F5-F13 resolved at the maintainer's request.
- 2026-09-23 18:02 UTC - Human review 5294842958 submitted, `CHANGES_REQUESTED` (F14).
- 2026-09-23 18:13 UTC - `docs(skills): make the branch-id pre-push check clear its reference case` authored (F14).
- 2026-09-23 18:15 UTC - Reply posted on F14; F14 resolved.
- 2026-09-23 18:18 UTC - Started audit. Record created after all threads were resolved; the four Copilot threads had been resolved on 2026-09-23 without replies, so replies were posted now and each states that it was posted after resolution.
- 2026-09-23 18:26 UTC - Human review 5295088674 submitted, `CHANGES_REQUESTED` (F15, F16) against the retrospective.
- 2026-09-23 18:32 UTC - `docs(pr-reviews): add #2320 review audit record` authored.
- 2026-09-23 19:43 UTC - `docs(pr-reviews): source #2320 retrospective times and counts from git and the API` authored (F15, F16); a first signing attempt at 19:39 failed on an expired GPG agent cache.
- 2026-09-23 19:46 UTC - Human review 5295966158 submitted, `CHANGES_REQUESTED` (F17-F20) against this record and the retrospective. Not recorded until 21:20; the 19:49 and 20:12 events below happened without it having been read.
- 2026-09-23 19:49 UTC - Replies posted on F15, F16.
- 2026-09-23 20:12 UTC - `docs(pr-reviews): record round 5 in the #2320 audit and retrospective` authored (F15, F16 rows); F15, F16 resolved after the push. F17-F20 were open and unrecorded at this commit.
- 2026-09-23 20:24 UTC - Human review 5296377033 submitted, `CHANGES_REQUESTED` (F21; F17-F20 re-raised in their threads).
- 2026-09-23 21:20 UTC - Correction: the 17:52 entry's clause "F5-F13 resolved at the maintainer's request" is wrong on time and source. Round 4 (18:02) records those threads still open; they were resolved after 18:02, and the request was made by the maintainer in the editor chat session, which no GitHub comment shows. The 18:18 entry's clause "replies were posted now" is wrong: the four Copilot replies are stamped 18:27:17Z-18:27:22Z. Both entries stand as written; this entry supersedes those clauses.
- 2026-09-23 21:20 UTC - Rows F17-F21 added; replies posted on their threads after this commit, then resolved.
- 2026-09-23 21:49 UTC - Human review 5297189747 submitted, `CHANGES_REQUESTED` (reviewer F18-F23, audit F22-F27).
- 2026-09-23 21:49 UTC - Human review 5297190311 submitted: follow-up in the F19 thread, one Review Summary row still wrong.
- 2026-09-24 05:48 UTC - Correction (F22): the two 21:20 entries above and the 19:46 entry's "Not recorded until 21:20" carry the editing time, not an event time. `docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`, which wrote them, was authored at 21:27 (`git log --format=%aI`). The replies on F17-F21 were posted at 21:25 (`created_at` 21:25:25Z-21:25:32Z), before that commit, not after it; the threads were resolved after it (order from the editor session; GitHub records no resolution time). Read "21:20" in those three places as 21:27, and the last entry as "replies posted 21:25, then this commit, then resolved". The entries stand as written; this entry supersedes those clauses.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
