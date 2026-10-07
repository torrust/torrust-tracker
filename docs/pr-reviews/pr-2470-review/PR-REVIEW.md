---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md
---

<!-- skill-link: process-pr-review -->

# PR #2470 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2470>.

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

Copilot review 5441412069 ("Lite" effort; its full agentic review timed out) left two inline
findings without IDs; its overview rates them High and Medium severity, recorded here as inferred
`Major` and `Minor`. They are F1 and F2. The overview lists only those two findings, so it adds
no row.

da2ce7 review 5441582812 (round 1, CHANGES_REQUESTED) left three inline findings numbered F1-F3 in
the reviewer's series. F1 and F2 are taken, so they become F3-F5, with their original IDs kept in
the detail entries. The reviewer's F3 asks for the same change as Copilot's F1, so F5 is a re-raise
of F1. The review body repeats the three findings and adds no other request.

da2ce7 review 5443352831 (round 2, CHANGES_REQUESTED) re-raised the three round-1 findings inline;
they are F6-F8. Its body also reports that the branch conflicts with `develop` after #2469; that
request has no thread and is F9.

da2ce7 review 5444749296 (round 3, CHANGES_REQUESTED) confirmed F3, F4, and F1 fixed and left one
new inline finding numbered F4 in the reviewer's series. F4 is taken, so it becomes F10, with its
original ID kept in the detail entry. Its body adds no other request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2470-f1` | Copilot | Major (inferred) | security | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2470-f2` | Copilot | Minor (inferred) | security | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2470-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2470-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2470-f5` | Human | Suggestion | security | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F6 | `review-finding:pr-2470-f6` | Human | Minor | documentation | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F7 | `review-finding:pr-2470-f7` | Human | Nit | documentation | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F8 | `review-finding:pr-2470-f8` | Human | Suggestion | security | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F9 | `review-finding:pr-2470-f9` | Human | Minor (inferred) | other | ORIGINAL | FIXED | NON_RESOLVABLE |
| F10 | `review-finding:pr-2470-f10` | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The cookie key is a public field of `UdpTrackerCoreServices`

- PR number: 2470
- Source review ID: 5441412069
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4206154106>
- Concern: as a `pub` field, `UdpTrackerCoreServices::cookie_cipher` lets any downstream holder of
  the services read the secret key, against the design goal of keeping it inside `udp-core`'s
  services.
- Solution: make the field `pub(crate)` and pin it with a `compile_fail,E0616` doctest that reads
  the field from outside the crate, plus a compiling companion that reads the public `event_bus`.
  The ADR now says the compiler enforces this.
- Current-tree verification: nothing outside `packages/udp-core/src` reads the field; workspace
  clippy is clean; both doctests pass on stable and nightly; making the field `pub` again turns the
  `compile_fail` doctest red (issue evidence, "Key Kept Inside `udp-core`").
- Resolution reference: `fix(udp-core): [#2458] keep the cookie key private to udp-core`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208633591>

### F2 - The raw key array is not wiped

- PR number: 2470
- Source review ID: 5441412069
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4206154176>
- Concern: `CookieCipher::random()` leaves the raw 32-byte key on the stack after building the
  cipher; only the expanded key schedule is wiped.
- Solution: already fixed before this audit, when the maintainer accepted the Task Reviewer's
  matching nit: the key is filled in place in a `zeroize::Zeroizing` buffer, wiped on return.
- Current-tree verification: `cookie_cipher.rs` creates `Zeroizing::new([0; KEY_LEN])`, fills it
  with `rand::rng().fill(&mut *key)`, and passes `&key` to `from_key`; da2ce7's round 2 confirmed
  it at the bytes.
- Resolution reference: `fix(udp-core): [#2458] wipe the raw cookie key bytes after building the cipher`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208633958>

### F3 - `src/AGENTS.md` still says global services set up a crypto seed

- PR number: 2470
- Source review ID: 5441582812
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4206297085>
- Concern: line 32 of the bootstrap diagram annotated `initialize_global_services()` with
  "logging, crypto seed", but the seed is deleted and the cookie key is created by
  `AppContainer::initialize`.
- Solution: annotate line 32 with "logging, clock" and line 33 with "builds all containers and the
  UDP cookie key".
- Current-tree verification: `initialize_global_services()` calls only `initialize_static()`
  (`torrust_clock::initialize_static()`) and `logging::setup`; `grep -n "crypto seed"
  src/AGENTS.md` finds nothing.
- Resolution reference: `docs(src): [#2458] stop describing a crypto seed in the bootstrap flow`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208634346>

### F4 - "ignores the code" reads as "ignores the doctest"

- PR number: 2470
- Source review ID: 5441582812
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4206297091>
- Concern: the `write-unit-test` note said stable rustdoc "ignores the code", but stable still
  fails a `compile_fail` doctest that compiles; it skips only the pinned error code.
- Solution: use the reviewer's wording: "Stable rustdoc still requires the doctest to fail to
  compile but accepts any error; only nightly checks the pinned error code."
- Current-tree verification: the skill now has that sentence; the ADR and the issue evidence
  already said stable "accepts any compile error", which is accurate.
- Resolution reference: `docs(skills): [#2458] say stable rustdoc still requires a compile_fail doctest to fail`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208634596>

### F5 - The cookie key field could be `pub(crate)` (re-raise)

- PR number: 2470
- Source review ID: 5441582812
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4206297099>
- Concern: the same change as F1, making `cookie_cipher` `pub(crate)` so the compiler enforces that
  `udp-server` never holds the key.
- Solution: no change of its own; the F1 fix covers it.
- Current-tree verification: same as F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208634882>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208634882>

### F6 - `src/AGENTS.md` crypto seed (round-2 re-raise)

- PR number: 2470
- Source review ID: 5443352831
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4207748485>
- Concern: the same stale "crypto seed" annotation as F3, re-checked at the round-2 head.
- Solution: no change of its own; the F3 fix covers it.
- Current-tree verification: same as F3.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635142>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635142>

### F7 - `write-unit-test` wording (round-2 re-raise)

- PR number: 2470
- Source review ID: 5443352831
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4207748504>
- Concern: the same wording as F4, re-checked at the round-2 head.
- Solution: no change of its own; the F4 fix covers it.
- Current-tree verification: same as F4.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635414>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635414>

### F8 - The cookie key field could be `pub(crate)` (round-2 re-raise)

- PR number: 2470
- Source review ID: 5443352831
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4207748517>
- Concern: the same change as F5 and F1, re-checked at the round-2 head.
- Solution: no change of its own; the F1 fix covers it.
- Current-tree verification: same as F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635765>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208635765>

### F9 - The branch conflicts with `develop`

- PR number: 2470
- Source review ID: 5443352831
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#pullrequestreview-5443352831>
- Concern: after #2469 merged, `git merge-tree` of the round-2 head onto `develop` reported a
  content conflict in the #2411 EPIC, so the branch needed a rebase before it could merge.
- Solution: rebased onto `develop` twice: first onto `4d8aa40d7`, resolving the EPIC's
  `last-updated-utc` conflict with the time of this branch's edit; then onto `a64d1497e` after
  #2468 merged, keeping both ADR index rows in timestamp order.
- Current-tree verification: the branch is 0 commits behind `develop`; `linter all` exits 0; the
  pre-push checks pass; all commits are signed; the EPIC row keeps this PR's citations.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2470#issuecomment-6040936531>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#issuecomment-6040936531>

### F10 - The #2411 EPIC cites moved lines for the cookie key's creation

- PR number: 2470
- Source review ID: 5444749296
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208912254>
- Concern: the EPIC's connection-ID row, rewritten by this PR, cited `cookie_cipher.rs:49` and
  `container.rs:171`; later commits in this PR moved both, so they no longer point at the key's
  creation, and the row matched neither the current tree nor the EPIC's review basis.
- Solution: name the key's creation by symbol (`CookieCipher::random`,
  `UdpTrackerCoreServices::initialize_from`), restore `connection_cookie.rs:159`, which resolves at
  the basis commit `7970cdf0a` (the earlier `:160` had departed from it), and say in the basis note
  why that row uses symbols.
- Current-tree verification: `random()` is at `cookie_cipher.rs:55`; `initialize_from` starts at
  `container.rs:175` and calls `CookieCipher::random()` at `:192`; at `7970cdf0a`,
  `connection_cookie.rs:159` is `pub fn check` and `:66` is the fingerprint note; `linter
  markdown` and `linter cspell` pass.
- Resolution reference: `docs(issues): [#2458] cite the #2411 cookie-key creation by symbol`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2470#discussion_r4208962370>

## Processing Log

- 2026-10-07 15:20 UTC - Started the audit for Copilot review 5441412069 and da2ce7 reviews
  5441582812 (round 1) and 5443352831 (round 2). Rebased onto `develop` `4d8aa40d7` (#2411 EPIC
  stamp conflict); committed the F1, F3, and F4 fixes separately (15:01-15:02), with ADR and
  issue-evidence updates (15:03-15:04); the branch was then 16 commits behind after #2468, so
  rebased onto `a64d1497e` (ADR index conflict, both rows kept). Two pushes failed with "Internal
  Server Error" from GitHub; a probe push of `develop`'s tip to a temporary branch failed the same
  way until 15:17, so the fault was on GitHub's side; the probe branch was deleted, and the push
  with the pre-push checks then succeeded. Replied on all eight threads (15:19:41-15:19:54) and
  posted the consolidated response covering the three reviews and F1-F9 (15:20:10).
- 2026-10-07 15:21 UTC - Recorded this audit; threads are resolved after it is pushed.
- 2026-10-07 15:50 UTC - Resolved the eight threads after the audit was pushed (reply guard exited 0) and
  re-requested da2ce7's review; the GitHub checks then passed at that head. da2ce7 round 3
  (5444749296, 15:45:30) confirmed F1, F3, and F4 fixed and raised F10. Committed the F10 fix
  (15:47), pushed, and replied (15:50:13). The round-3 body also notes that Git reads the body of
  `fix(udp-core): [#2458] keep the cookie key private to udp-core` as a pseudo-trailer, because its
  one paragraph starts with `UdpTrackerCoreServices::cookie_cipher`; confirmed with
  `git log --format='%(trailers)'`. It is not a finding and would need a history rewrite, so the
  commit stays.

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
