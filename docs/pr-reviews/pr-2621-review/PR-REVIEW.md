---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/ISSUE.md
    - docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2621 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2621>.

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

Four reviews were submitted and all findings are handled. Copilot's review 5603592801 (COMMENTED,
2026-10-21 17:24:59 UTC) read the draft at its first commit and has three inline threads, F1 to F3; its body
carries no further assertion. The maintainer's reviewer `da2ce7` submitted review 5604621746 (DISMISSED,
2026-10-21 18:49:37 UTC) with five inline threads, which carry the reviewer's own IDs F1 to F5. Those IDs collide
with the audit IDs already given to Copilot's findings, so they are audit F4 to F8 and the reviewer's ID is kept in
each detail entry. The same reviewer's second review, 5605012005 (DISMISSED, 2026-10-21 19:20:20 UTC), read head
`6ef5d4635` and has five inline threads (reviewer IDs F6 to F10, audit F9 to F13) and one review-body assertion that
a round-1 finding was only partly repaired (audit F14, a re-raise of F8, with no thread).

Both `da2ce7` reviews are DISMISSED because each push dismissed the earlier approval; their threads remain unresolved
and actionable, so they are processed like any other review.

F4, F5 and F6 re-raise F1, F3 and F2 respectively: each asked again for a change that the first fix had only
partly delivered.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2621-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2621-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2621-f3 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2621-f4 | Human | Minor | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F5 | review-finding:pr-2621-f5 | Human | Minor | testing | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F6 | review-finding:pr-2621-f6 | Human | Minor | testing | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F7 | review-finding:pr-2621-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2621-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2621-f9 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2621-f10 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2621-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2621-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2621-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2621-f14 | Human | Nit (inferred) | metadata | RE_RAISE_OF:F8 | FIXED | NON_RESOLVABLE |
| F15 | review-finding:pr-2621-f15 | Copilot | Minor (inferred) | security | ORIGINAL | NO_ACTION | SUPERSEDED |
| F16 | review-finding:pr-2621-f16 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2621-f17 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2621-f18 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

Copilot's second review, 5619053291 (COMMENTED, 2026-10-22 18:34:26 UTC), read head `8e38fe7be` and has four inline
threads (F15 to F18). Its body only warns that the agentic review did not start before the timeout. F15 asks to
replace the repository's published development token, which the maintainer accepted and which PR #2616 answered
the same way; F16 and F17 both ask that the evidence commands be runnable as written (two anchors, one concern each);
F18 asks for a fully qualified path.

## Finding Details

### F1 - Selecting only `client_address_ip_family` leaves the dual-stack mapped-address case unresolved

- PR number: 2621
- Source review ID: 5603592801
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4339842199>
- Concern: `event.rs` classifies an IPv4-mapped IPv6 client as `inet6`, so a client-family selection still counts
  such a client as `tcp6`. The draft must define the mapped-address semantics for both transports and require
  regression cases.
- Solution: Background now names the `Inet6` classification, and the Regression Test Strategy has case R3 for a
  mapped client. The maintainer later decided the rule (a mapped client counts as IPv6).
- Current-tree verification: `ISSUE.md` Background names the `ipv4_mapped_ipv6` case in `event.rs`, and R3 covers
  `::ffff:192.0.2.1`.
- Resolution reference: docs(issues): address review on tcp4/tcp6 counter draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351896966>

### F2 - The draft omits the recorded reproduction and separate red, fix and recheck tasks

- PR number: 2621
- Source review ID: 5603592801
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4339842273>
- Concern: `fix-bug/SKILL.md` requires a reproduction outcome in `manual-verification-evidence.md` and separate
  recorded failing-test, fix and recheck tasks.
- Solution: the reproduction was run and recorded as V1 with the configuration, the statistics and the log under
  `evidence/`, and the plan split the test, the fix and the recheck.
- Current-tree verification: `manual-verification-evidence.md` exists and classifies V1 as Reproduced;
  `evidence/` holds the configuration, the raw statistics and the log.
- Resolution reference: docs(issues): address review on tcp4/tcp6 counter draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351897321>

### F3 - The manual check cannot run: an IPv4-only listener accepts no IPv6 connection

- PR number: 2621
- Source review ID: 5603592801
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4339842320>
- Concern: a native IPv6 client cannot reach an IPv4 listener, so the check stops before the counters. Use an
  explicit reverse-proxy scenario with an IPv6 forwarded address.
- Solution: the scenario now uses `[http_trackers.network] on_reverse_proxy = true` and an IPv6
  `X-Forwarded-For`, and it was run.
- Current-tree verification: the run increments `tcp4_announces_handled` from 0 to 1 and leaves `tcp6` at 0
  (`evidence/before.json`, `evidence/after.json`).
- Resolution reference: docs(issues): address review on tcp4/tcp6 counter draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351897666>

### F4 - Selecting `client_address_ip_family` alone still counts dual-stack IPv4 clients as `tcp6`

- PR number: 2621
- Source review ID: 5604621746
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340667128>
- Concern: the draft left the mapped-address rule open. It asks for a decision for both transports and a
  regression case.
- Solution: the maintainer decided that a mapped client stays IPv6 and the docs say so (option (b)). The decision
  follows issue #1375 (commit `d4c43bd3a`), which exposed mapped clients through the separate label
  `server_binding_address_type`. That decision does not rule explicitly on the `tcp4`/`tcp6` aggregate, so the draft
  records the application as an inference.
- Current-tree verification: `ISSUE.md` Background and Open Decisions record the rule; R3 and T2 carry it.
- Resolution reference: docs(issues): record tcp4/tcp6 counter decisions and docs-first plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351897974>

### F5 - The manual check cannot run as written for an IPv4-bound listener

- PR number: 2621
- Source review ID: 5604621746
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340667137>
- Concern: the consequence in Background and the reproduction need a reachable route.
- Solution: M1 runs in reverse-proxy mode; Background states that the IPv6-over-IPv4 case is reachable only through
  `X-Forwarded-For`; M3 covers dual-stack.
- Current-tree verification: the M1 row and the Background paragraph in `ISSUE.md` name `on_reverse_proxy`.
- Resolution reference: docs(issues): revise tcp4/tcp6 counter draft for review findings F1-F5
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351898344>

### F6 - No recorded red run and no separate red, fix and recheck tasks

- PR number: 2621
- Source review ID: 5604621746
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340667151>
- Concern: `fix-bug` requires a recorded failing run before the fix, separate fix and recheck tasks, and has no
  documentation-only exception.
- Solution: the plan has a red run, a fix and a recheck as separate tasks (now T5, only if the behavior changes),
  and the documentation-first path is stated as a proposed reduced process that the maintainer accepted.
- Current-tree verification: the Implementation Plan and Bug-Fix Process sections of `ISSUE.md` say so.
- Resolution reference: docs(issues): revise tcp4/tcp6 counter draft for review findings F1-F5
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351898865>

### F7 - The sections `create-issue` requires before review are missing

- PR number: 2621
- Source review ID: 5604621746
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340667160>
- Concern: Progress Tracking, Acceptance Criteria, Acceptance Verification, Implementation Completion Review and
  tracked manual scenarios were absent.
- Solution: the sections were added in template order.
- Current-tree verification: `grep -n '^## ' ISSUE.md` lists all of them.
- Resolution reference: docs(issues): revise tcp4/tcp6 counter draft for review findings F1-F5
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351899217>

### F8 - The binding label is set in `event.rs`, not in `handler.rs`

- PR number: 2621
- Source review ID: 5604621746
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340667168>
- Concern: the analysis step and the related artifacts name the wrong file.
- Solution: both name `packages/http-core/src/event.rs`.
- Current-tree verification: `event.rs` lines 151–152 fill `server_binding_address_ip_family`; the draft cites
  that file in the analysis step and in the frontmatter.
- Resolution reference: docs(issues): revise tcp4/tcp6 counter draft for review findings F1-F5
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351899572>

### F9 - `before.txt` and `after.txt` are not the raw output of the `curl` commands

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340977431>
- Concern: the files held a filtered Python dictionary; `fix-bug` asks for the actual output of the reproduction.
- Solution: the reproduction was repeated and the raw responses are stored as `evidence/before.json` and
  `evidence/after.json`, with the exact `jq` filter shown next to the excerpts.
- Current-tree verification: `jq -c 'with_entries(select(.key|test("^(tcp|udp4_announces|udp6_announces)")))'`
  on both files prints the counters in the evidence file (`tcp4_announces_handled` 0 to 1, `tcp6` 0).
- Resolution reference: docs(issues): store the raw tcp4/tcp6 reproduction output and name its artifact
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351899950>

### F10 - An `http-core` regression case cannot cover the UDP counters, and the plan omits `udp-server`

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340977453>
- Concern: the UDP getters live in `udp-core` and `udp-server`; the REST `udp4_*`/`udp6_*` fields read
  `udp-server`.
- Solution: the Regression Test Strategy lists the seam per package, T3 lists per-package UDP cases, the
  Verification Plan adds `cargo test -p torrust-tracker-udp-server`, and scenario M4 covers the UDP dual-stack case.
- Current-tree verification: `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs` reads
  `udp_server_stats.udp4_*`, and `packages/udp-server/src/statistics/metrics.rs` selects on
  `server_binding_address_ip_family`; `ISSUE.md` now cites both.
- Resolution reference: docs(issues): cover udp-server, name the mapped-address tests and fix the route wording
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351900360>

### F11 - "the handler test `v4_mapped_v6`" names a label value, not a test

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340977471>
- Concern: the draft cites a test that does not exist by that name.
- Solution: the draft names the `ipv4_mapped_ipv6` case in `event.rs` and the
  `ipv6_scrape_of_an_ipv4_mapped_client_with_a_public_url_on_another_port` case in `handler.rs`.
- Current-tree verification: `grep -n 'ipv4_mapped_ipv6\|ipv6_scrape_of_an_ipv4_mapped_client' packages/http-core/src`
  finds both cases; `v4_mapped_v6` remains in `ISSUE.md` only as the #1375 label value.
- Resolution reference: docs(issues): cover udp-server, name the mapped-address tests and fix the route wording
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351900737>

### F12 - The 18:49 entry reports a reproduction that the log places at 18:50, and no artifact commit is named

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340977479>
- Concern: a stamp contradicts the log, and the evidence names no durable commit for the artifact under test.
- Solution: the Progress Log entry reads 18:50 UTC; the evidence file names the merge base `80c45b606`
  and records the repeat run's own times. The evidence part is in
  `docs(issues): store the raw tcp4/tcp6 reproduction output and name its artifact`.
- Current-tree verification: `git merge-base HEAD torrust/develop` prints `80c45b606…`, which the evidence file
  cites; `evidence/tracker.log` starts at 18:09:38 UTC and ends at 18:10:01 UTC, matching the stated times.
- Resolution reference: docs(issues): cover udp-server, name the mapped-address tests and fix the route wording
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351901130>

### F13 - "In that mode" follows the dual-stack sentence and the routes are mislabeled

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4340977494>
- Concern: the `X-Forwarded-For` sentence belongs to reverse-proxy mode, and the dual-stack route reaches the
  opposite mismatch only once mapped clients count as IPv4.
- Solution: the Background puts the `X-Forwarded-For` sentence with reverse-proxy mode, adds the IPv4-over-`inet6`
  reverse-proxy case, and states that a dual-stack binding does not diverge under the decided rule.
- Current-tree verification: `ISSUE.md` Background reads "A dual-stack `[::]` binding is a different situation …
  the two meanings do not diverge".
- Resolution reference: docs(issues): cover udp-server, name the mapped-address tests and fix the route wording
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4351901442>

### F14 - Round-1 finding F5 is only partly repaired: the frontmatter still lists `handler.rs`, not `event.rs`

- PR number: 2621
- Source review ID: 5605012005
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#pullrequestreview-5605012005>
- Concern: the review body says the analysis step names `event.rs` but the frontmatter artifacts still list
  `handler.rs` and not `event.rs`. It has no inline thread.
- Solution: the frontmatter related artifacts now list `packages/http-core/src/event.rs` and the statistics listener.
- Current-tree verification: the `related-artifacts` list in the frontmatter of `ISSUE.md` contains
  `packages/http-core/src/event.rs`.
- Resolution reference: docs(issues): record tcp4/tcp6 counter decisions and docs-first plan
- Follow-up PR URL: N/A
- Reply URL: N/A

### F15 - A working admin token value is committed in the evidence configuration

- PR number: 2621
- Source review ID: 5619053291
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352093629>
- Concern: `evidence/tracker.toml` embeds `admin = "MyAccessToken"`, which could be reused or copied into a real
  deployment; Copilot asks for a placeholder or a redaction.
- Solution: no change. `MyAccessToken` is the repository's published default development token. The tracker ran on
  loopback and was stopped after the run, and a placeholder would make the recorded commands non-reproducible as
  run. The maintainer accepted the value for this draft, and PR #2616 answered the same finding the same way.
- Current-tree verification: `grep -rln MyAccessToken share src` lists
  `share/default/config/tracker.development.sqlite3.toml`, `src/lib.rs` and
  `src/console/ci/qbittorrent_e2e/tracker/config_builder.rs`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352131428>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352131428>

### F16 - The evidence commands write to relative paths that do not land in the committed folder

- PR number: 2621
- Source review ID: 5619053291
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352093713>
- Concern: the file says the commands run from the repository root but writes `evidence/before.json` and
  `evidence/after.json` relatively, which is not the committed folder.
- Solution: the commands use a variable `E` for `docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence`
  and write the responses to `"$E/before.json"` and `"$E/after.json"`; the announce body goes to `.tmp/`.
- Current-tree verification: `grep -n 'evidence/before.json' manual-verification-evidence.md` finds only the Markdown
  links; the `curl` and `jq` lines use `"$E/..."`.
- Resolution reference: docs(issues): make the tcp4/tcp6 reproduction commands runnable from the repository root
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352131778>

### F17 - The tracker is started from a `.tmp` config although the committed artifact is `evidence/tracker.toml`

- PR number: 2621
- Source review ID: 5619053291
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352093789>
- Concern: the runtime configuration path differs from the committed artifact without explanation.
- Solution: step 1 copies the committed configuration to `.tmp/tcp-family-repro/` and explains that the database
  path lives in the git-ignored `.tmp/` folder.
- Current-tree verification: `diff docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/evidence/tracker.toml
  .tmp/tcp-family-repro/tracker.toml` prints nothing.
- Resolution reference: docs(issues): make the tcp4/tcp6 reproduction commands runnable from the repository root
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352132084>

### F18 - The Background mixes a fully qualified path with a shortened one

- PR number: 2621
- Source review ID: 5619053291
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352093876>
- Concern: `statistics/event/handler.rs` is shortened while the other paths are fully qualified.
- Solution: the Background cites `packages/http-core/src/statistics/event/handler.rs`.
- Current-tree verification: `ls packages/http-core/src/statistics/event/handler.rs` succeeds, and the Background
  line uses that path.
- Resolution reference: docs(issues): fully qualify the handler path in the tcp4/tcp6 draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352132396>

## Processing Log

- 2026-10-21 17:24 UTC - Copilot submitted review 5603592801 with three inline threads (F1 to F3).
- 2026-10-21 18:49 UTC - `da2ce7` submitted review 5604621746 with five inline threads (audit F4 to F8).
- 2026-10-21 18:53 UTC - Fixed Copilot's three findings in a signed commit and pushed.
- 2026-10-21 18:57 UTC - Fixed the five round-1 findings in a signed commit and pushed.
- 2026-10-21 19:20 UTC - `da2ce7` submitted review 5605012005 with five inline threads (audit F9 to F13) and one
  review-body assertion (audit F14).
- 2026-10-22 18:07 UTC - Recorded the maintainer's decisions (mapped-address rule, docs-first sequence) in a signed
  commit and pushed; the commit also repaired the frontmatter assertion of F14.
- 2026-10-22 18:15 UTC - Repeated the V1 reproduction, fixed F9 to F13 in two signed commits, pushed, and replied on
  all thirteen inline threads. GraphQL listed thirteen unresolved threads at that point.
- 2026-10-22 18:20 UTC - Resolved all thirteen threads after the audit validated; GraphQL data shows 0 unresolved
  threads.
- 2026-10-22 18:34 UTC - Copilot submitted review 5619053291 on head `8e38fe7be` with four inline threads (F15 to F18).
- 2026-10-22 18:38 UTC - Verified each claim against the current tree. Fixed F16 to F18 in two signed commits and
  pushed; answered F15 with the prescribed `Superseded by F15` reply (the development token is the repository's
  published default, accepted by the maintainer). Replied on all four threads.
- 2026-10-22 18:39 UTC - Resolved the four threads after the audit validated; GraphQL data shows 0 unresolved
  threads.

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
