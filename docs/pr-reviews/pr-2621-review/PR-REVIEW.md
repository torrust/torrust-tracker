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

Ten reviews with a body had been submitted when this record was last updated, and all findings are handled. Copilot's review 5603592801 (COMMENTED,
2026-10-21 17:24:59 UTC) read the draft at its first commit and has three inline threads, F1 to F3; its body
carries no further assertion. The maintainer's reviewer `da2ce7` submitted review 5604621746 (DISMISSED,
2026-10-21 18:49:37 UTC) with five inline threads, which carry the reviewer's own IDs F1 to F5. Those IDs collide
with the audit IDs already given to Copilot's findings, so they are audit F4 to F8 and the reviewer's ID is kept in
each detail entry. The same reviewer's second review, 5605012005 (DISMISSED, 2026-10-21 19:20:20 UTC),
read the head with the subject `docs(issues): revise tcp4/tcp6 counter draft for review findings F1-F5` and has five inline threads (reviewer IDs F6 to F10, audit F9 to F13) and one review-body assertion that
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
| F19 | review-finding:pr-2621-f19 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2621-f20 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2621-f21 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2621-f22 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2621-f23 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2621-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2621-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2621-f26 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2621-f27 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2621-f28 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2621-f29 | Copilot | Nit | documentation | RE_RAISE_OF:F20 | FIXED | RESOLVED |
| F30 | review-finding:pr-2621-f30 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-2621-f31 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F32 | review-finding:pr-2621-f32 | Copilot | Nit | link-integrity | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F33 | review-finding:pr-2621-f33 | Copilot | Nit | testing | ORIGINAL | FIXED | NON_RESOLVABLE |

`da2ce7`'s fifth review, 5620094757 (APPROVED, 2026-10-22 19:59:20 UTC), approves the head and has two non-blocking
inline threads, F30 and F31, whose IDs are kept. Copilot's fifth review, 5620148855 (COMMENTED, 2026-10-22 20:04:03
UTC), recommends approval and opened no inline thread, but its body lists two independently actionable assertions under
"previously missed issues". It labels them F31 and F30, which collide with da2ce7's, so they are audit F32 (the
`closed/1349-...` path, reviewer ID F31) and F33 (scenario M3's configuration, reviewer ID F30), in the body's order.
Both are `NON_RESOLVABLE`: there is no thread, so their replies are one consolidated pull-request comment.

Copilot's fourth review, 5619691908 (COMMENTED, 2026-10-22 19:24:29 UTC), read the head carrying the third-round audit
and opened one inline thread. It labels its finding F20, which collides with the audit's F20, so it is audit F29 and
the reviewer's ID is kept in the detail entry. It re-raises F20: the UDP reworking left acceptance criterion AC2 asking
for mixed-family cases on both transports.

`da2ce7`'s fourth review, 5619631773 (submitted APPROVED at 2026-10-22 19:19:24 UTC and DISMISSED at 19:42:01 UTC by
the push of the next commit), approves the head and has three non-blocking
inline threads, F26 to F28, whose IDs are kept. Its body reports the repairs of F19 to F25 and checks, and carries no
other actionable assertion.

`da2ce7`'s third review, 5619336077 (CHANGES_REQUESTED, 2026-10-22 18:54:57 UTC), has seven inline threads, F19 to F25;
the reviewer's IDs do not collide with the audit's, so they are kept. F19 is Major and blocks: it shows that the draft
and the audit cited issue #1375 as precedent for a decision that #1375 does not make. Copilot's third review,
5619345600 (COMMENTED, 2026-10-22 18:55:46 UTC), recommends approval and has no inline thread or other actionable
assertion, so it adds no row. The review body of 5619336077 only renumbers its own inline findings and reports checks.

Copilot's second review, 5619053291 (COMMENTED, 2026-10-22 18:34:26 UTC), read the head with the subject
`docs(pr-reviews): add the PR #2621 review audit` and has four inline
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
  rests on the maintainer's choice. An earlier version of this entry said it followed issue #1375; that was wrong,
  because commit `d4c43bd3a` added only the server-side label `server_binding_address_type` (see F19).
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
- Current-tree verification: `grep -rln MyAccessToken share src` prints seven files:
  `share/default/config/tracker.container.mysql.toml`, `tracker.container.postgresql.toml`,
  `tracker.container.sqlite3.toml`, `tracker.development.sqlite3.toml` and `tracker.e2e.container.sqlite3.toml` (all
  under `share/default/config/`), `src/console/ci/qbittorrent_e2e/tracker/config_builder.rs` and `src/lib.rs`.
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
- Solution: an earlier version copied the committed configuration to `.tmp/tcp-family-repro/` and gave a reason for the
  copy that did not hold (see F24). Step 1 now starts the tracker from the committed configuration and explains the
  real need: the SQLite driver resolves the configured database path against the working directory and does not
  create the folder, so `mkdir -p .tmp/tcp-family-repro` is required.
- Current-tree verification: `grep -n 'config-toml-path\|mkdir -p' manual-verification-evidence.md` shows the committed
  `evidence/tracker.toml` path and the `mkdir -p` line; the commands ran as written in two separate shells (see F23).
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

### F19 - #1375's label describes the server binding, not a mapped client

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294718>
- Concern: the draft (and F4's Solution in this audit) said that #1375 kept a mapped client out of the IPv4 count and
  exposed it through `server_binding_address_type`. That label is computed from the server's binding, and #1671 says
  #1375 had no client-side counterpart, so the precedent does not exist and a reader would take a server label for a
  client one.
- Solution: the draft cites #1375 only as the server-side label, states that option (b) rests on the maintainer's
  decision, and names `client_address_ip_type` (#1671) as the client-side label. F4's Solution is restated.
- Current-tree verification: `git show d4c43bd3a` adds `server_binding_address_type` from
  `connection_context.server.service_binding.bind_address_type()`, and
  `docs/issues/closed/1671-ipv4-ipv6-client-metrics/ISSUE.md` line 47 says #1375 "did not include a client-side
  counterpart"; `grep -n 1375 ISSUE.md` lists only the restated mentions.
- Resolution reference: docs(issues): restate the mapped-address precedent in the tcp4/tcp6 draft
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352372848>

### F20 - UDP's dual-stack case is the mapped case, not a mixed one

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F20
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294726>
- Concern: UDP delivers a dual-stack client as a mapped address, so UDP has no case where the binding and client
  families differ; R2 and M4 treated the dual-stack case as a mixed one.
- Solution: Background states the UDP behavior, R2 is HTTP only, the UDP dual-stack case is under R3, and M4 says it
  cannot tell the two meanings apart. T3 adds a case for the family of a mapped UDP client.
- Current-tree verification: `packages/udp-core/src/event.rs` derives `client_address_ip_family` from
  `client_socket_addr.ip().into()` and the type `V4MappedV6` from `to_ipv4_mapped()`; the only mapped-client test
  there pins the type, which `ISSUE.md` now says.
- Resolution reference: docs(issues): align the UDP dual-stack cases with the mapped-address rule
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352373131>

### F21 - The audit cites branch commits by id

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F21
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294736>
- Concern: the audit named the heads that two reviews read by abbreviated commit id; no durable reference may cite a
  branch commit by id.
- Solution: the three mentions now use the commit subjects. One Processing Log entry (18:34 UTC) and two head
  references in the Findings introduction were edited for this; a correction entry in the log names them.
- Current-tree verification: a scan of every 7 to 40 character hexadecimal token in the draft folder and the audit
  against `torrust/develop` leaves only `80c45b606` and `d4c43bd3a`, both on `develop`.
- Resolution reference: docs(pr-reviews): record the third review round and drop branch commit ids from the PR #2621 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352373461>

### F22 - Two stamps are later than the commits that carry them

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F22
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294752>
- Concern: the draft's 18:15 entry and the audit's 18:20 entry postdate their commits, and the evidence file's
  `last-updated-utc` predates its edit.
- Solution: the draft's entry reads 18:13 UTC and the evidence stamp was re-stamped. The audit's 18:20 entry is not
  rewritten because the Processing Log is append-only; a correction entry states that the entry was committed at
  18:17 UTC. The one in-place edit to a log entry (the 18:34 entry's head reference, which F21 required) is recorded
  in its own correction entry.
- Current-tree verification: `git log --format='%aI %s'` lists the commit carrying the draft's 18:13 entry at
  18:13:11Z and the commit that carried the audit's resolution entry at 18:17:09Z.
- Resolution reference: docs(issues): correct the draft's progress-log stamp and record the third review round
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352373714>

### F23 - Steps 2 to 4 cannot run as written

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F23
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294760>
- Concern: `E` is set only in step 1's shell, which the foreground tracker holds, so `"$E/before.json"` expands to
  `/before.json` in the second shell.
- Solution: step 1 runs in terminal A, steps 2 to 4 in terminal B, and every block that uses `E` sets it.
- Current-tree verification: I ran step 1 in one shell and steps 2 to 4 in another, as written: health 200,
  announce 200, `tcp4_announces_handled` 0 to 1, `tcp6` 0, identical to the stored files.
- Resolution reference: docs(issues): split the reproduction steps by shell and state why the database folder is created
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352374102>

### F24 - The runtime copy's reason does not explain the copy

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F24
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294767>
- Concern: the SQLite path resolves against the working directory, so the copy was unnecessary and the `diff` proved
  nothing; F17's verification could not run on a clean tree.
- Solution: step 1 starts the tracker from the committed configuration and states the real need (`mkdir -p`); F17's
  entry is rewritten accordingly.
- Current-tree verification: `packages/tracker-core/src/databases/driver/sqlite/mod.rs` opens the path with
  `SqliteConnectOptions::new().filename(db_path).create_if_missing(true)`, and step 1 ran from the committed file in
  F23's verification.
- Resolution reference: docs(issues): split the reproduction steps by shell and state why the database folder is created
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352374432>

### F25 - F15's recorded grep output is three of seven files

- PR number: 2621
- Source review ID: 5619336077
- Reviewer finding ID: F25
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352294774>
- Concern: the row recorded three of the seven files that the command prints.
- Solution: F15's verification lists all seven.
- Current-tree verification: `grep -rln MyAccessToken share src | sort` prints seven paths, all listed in F15.
- Resolution reference: docs(pr-reviews): record the third review round and drop branch commit ids from the PR #2621 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352374678>

### F26 - The F21 edit doubled a phrase

- PR number: 2621
- Source review ID: 5619631773
- Reviewer finding ID: F26
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352532013>
- Concern: the Findings introduction ended one line with "read head" and began the next with "read the head with the
  subject", so the sentence repeated itself.
- Solution: the trailing "read head" is dropped.
- Current-tree verification: `sed -n 46,50p PR-REVIEW.md` shows the line ending "(DISMISSED, 2026-10-21 19:20:20 UTC),"
  followed by "read the head with the subject".
- Resolution reference: docs(pr-reviews): fix the doubled phrase and the log-edit account in the PR #2621 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352752363>

### F27 - The F21 row's account of the log edits disagrees with the log

- PR number: 2621
- Source review ID: 5619631773
- Reviewer finding ID: F27
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352532029>
- Concern: F21 said two log lines were edited while only the 18:34 entry changed, and F22's append-only reason
  disagreed with that in-place edit.
- Solution: F21 now says one log entry and two introduction references were edited, F22 records that this is the one
  in-place log edit and that it has its own correction entry, and the log gains that entry.
- Current-tree verification: the F21 Solution, the F22 Solution and the 19:43 correction entry in the Processing Log
  all name the same single edited entry (18:34 UTC).
- Resolution reference: docs(pr-reviews): fix the doubled phrase and the log-edit account in the PR #2621 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352752696>

### F28 - F21 and F25 were reported fixed before the commit that fixes them

- PR number: 2621
- Source review ID: 5619631773
- Reviewer finding ID: F28
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352532038>
- Concern: the draft's 19:02 progress-log entry listed F20 to F25 as addressed, and the replies for F21 and F25 said
  FIXED, although the audit commit that fixes them came later. The replies were not re-derived against the pushed
  tree.
- Solution: the draft entry lists F20, F22, F23 and F24, and a 19:23 entry records the correction. The F26 to F28
  replies were posted after the fixing commits were pushed, but the F27 correction entry reached the audit only in the
  following commit (F30). The replies already posted stay on GitHub.
- Current-tree verification: `grep -n 'F20, F22, F23 and F24' ISSUE.md` finds the corrected entry.
- Resolution reference: docs(issues): correct the progress-log entry that listed audit fixes as addressed early
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352753013>

### F29 - AC2 still asks for mixed-family coverage on both transports

- PR number: 2621
- Source review ID: 5619691908
- Reviewer finding ID: F20
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352581933>
- Concern: after F20, the regression strategy limits R1 and R2 to HTTP and gives UDP plain-family and mapped-address
  cases, but AC2 still required mixed-family coverage on both transports, so completion would demand UDP cases the
  plan excludes.
- Solution: AC2 names the `http-core` R1–R3 tests and the plain-family and mapped-client cases in `udp-core` and
  `udp-server`; the Acceptance Verification row matches.
- Current-tree verification: `grep -n 'AC2' ISSUE.md` shows the criterion and the verification row with the
  per-package wording, and the regression strategy says UDP has no case where the binding and client families differ.
- Resolution reference: docs(issues): align acceptance criterion AC2 with the per-package UDP cases
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352777635>

### F30 - Three statements about this round disagree with the forge

- PR number: 2621
- Source review ID: 5620094757
- Reviewer finding ID: F30
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352912017>
- Concern: the audit recorded review 5619631773 as APPROVED although the push of the next commit dismissed it, still
  said "Four reviews were submitted", and said F28's fixing commits were pushed before the replies although the F27
  correction entry came later.
- Solution: the review is recorded as submitted APPROVED and dismissed at 19:42:01 UTC, the count reads ten reviews
  with a body, and F28's Solution is qualified.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/issues/2621/timeline` lists the dismissal of
  5619631773 at 2026-10-22T19:42:01Z, and `gh api repos/torrust/torrust-tracker/pulls/2621/reviews --paginate`
  lists ten reviews with a non-empty body at the time of the fix.
- Resolution reference: docs(pr-reviews): correct the review states, the review count and the F28 account in the PR #2621 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4357061829>

### F31 - The 19:23 entry names the review that did not raise F28

- PR number: 2621
- Source review ID: 5620094757
- Reviewer finding ID: F31
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4352912034>
- Concern: the draft's 19:23 progress-log entry cited review 5619336077 for F28, which came from review 5619631773.
- Solution: the entry cites 5619631773, and a later entry records the correction.
- Current-tree verification: `grep -n 'review 5619631773, F28' ISSUE.md` finds the corrected entry.
- Resolution reference: docs(issues): cite the review that raised F28 in the draft's progress log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#discussion_r4357050056>

### F32 - The `closed/1349-...` path does not exist in this tree

- PR number: 2621
- Source review ID: 5620148855
- Reviewer finding ID: F31
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#pullrequestreview-5620148855>
- Concern: the draft's Background cites `docs/issues/closed/1349-1347-add-tests-http-core/`, which the review says is
  absent; the specification is under `open/`.
- Solution: no change. The branch is based on `80c45b606`, before the archive pull request, so the path is absent
  here, but PR #2620 moved the specification to `closed/` on `torrust/develop`. Citing `open/` would be wrong once
  this pull request merges.
- Current-tree verification: `ls -d docs/issues/closed/1349* docs/issues/open/1349*` on the branch finds only the
  `open/` path; `git ls-tree -d --name-only torrust/develop docs/issues/closed/1349-1347-add-tests-http-core` finds
  the `closed/` path, and `git merge-base --is-ancestor 29e1f7ce3 HEAD` fails because the archive is not in the branch.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2621#issuecomment-6359572112>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#issuecomment-6359572112>

### F33 - Scenario M3's configuration is not explicit

- PR number: 2621
- Source review ID: 5620148855
- Reviewer finding ID: F30
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2621#pullrequestreview-5620148855>
- Concern: starting M3 from V1's configuration leaves `on_reverse_proxy = true`, so the client address would come from
  `X-Forwarded-For`, and V1's `info` logging omits the counter labels, which are logged at debug level.
- Solution: M3 specifies its own configuration with a dual-stack bind address, `on_reverse_proxy = false` and
  `trace_filter = "debug"`.
- Current-tree verification: `resolve_remote_client_addr` in
  `packages/http-protocol/src/v1/services/peer_ip_resolver.rs` reads `X-Forwarded-For` only for
  `ReverseProxyMode::Enabled`, and `packages/http-core/src/statistics/event/handler.rs` logs the labels with
  `tracing::debug!`; the M3 row of `ISSUE.md` names all three settings.
- Resolution reference: docs(issues): make the dual-stack scenario M3 configuration explicit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2621#issuecomment-6359572112>

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
- 2026-10-22 18:34 UTC - Copilot submitted review 5619053291 on the head with the subject `docs(pr-reviews): add the PR #2621 review audit` with four inline threads (F15 to F18).
- 2026-10-22 18:38 UTC - Verified each claim against the current tree. Fixed F16 to F18 in two signed commits and
  pushed; answered F15 with the prescribed `Superseded by F15` reply (the development token is the repository's
  published default, accepted by the maintainer). Replied on all four threads.
- 2026-10-22 18:39 UTC - Resolved the four threads after the audit validated; GraphQL data shows 0 unresolved
  threads.
- 2026-10-22 18:54 UTC - `da2ce7` submitted review 5619336077 (CHANGES_REQUESTED) with seven inline threads (F19 to
  F25). Copilot submitted review 5619345600 at 18:55 UTC, which recommends approval and has no inline thread.
- 2026-10-22 19:04 UTC - Verified each claim against the current tree and fixed F19 to F24 in four signed commits
  (F19, F20, F22 and F23 with F24, in the draft folder) plus this audit update for F21 and F25; replied on all seven
  threads. F19 was a real error of mine: issue #1375 added only a server-side label, so the draft and F4's Solution
  no longer cite it as precedent.
- 2026-10-22 19:04 UTC - Correction: the 18:20 UTC entry earlier in this log reports threads resolved, but it shipped in
  a commit made at 18:17:09Z, so its stamp is wrong. Its text is not rewritten. The 18:34 UTC entry and two head
  references in the Findings introduction were edited to replace branch commit ids with commit subjects (F21).
- 2026-10-22 19:05 UTC - Resolved the seven threads after the audit was checked; GraphQL data shows 0 unresolved
  threads.
- 2026-10-22 19:19 UTC - `da2ce7` approved the head (review 5619631773) with three non-blocking inline threads (F26
  to F28). Copilot had recommended approval on the previous head and was asked to review the new one.
- 2026-10-22 19:24 UTC - Copilot submitted review 5619691908 with one inline thread, reviewer ID F20 (audit F29, a
  re-raise of F20 about acceptance criterion AC2). The 19:20 poll had ended before it arrived.
- 2026-10-22 19:43 UTC - Verified each claim against the current tree and fixed F26 to F28 in two signed commits
  (one in the draft folder, one in this audit); pushed them, then replied on the three threads. The F21 and F25
  replies of the previous round had been posted before their fixing commit was pushed (F28).
- 2026-10-22 19:43 UTC - Correction: the 18:34 UTC entry in this log was edited in place, to replace its abbreviated
  head id with the commit subject (F21); that is the only in-place edit to a log entry. The 19:04 entry above names it.
- 2026-10-22 19:47 UTC - Fixed F29 in one signed commit, pushed it, replied on the thread and resolved it, together
  with the three threads of review 5619631773 (F26 to F28). GraphQL data shows 0 unresolved threads.
- 2026-10-22 19:59 UTC - `da2ce7` approved the head (review 5620094757) with two non-blocking inline threads (F30 and
  F31).
- 2026-10-22 20:04 UTC - Copilot submitted review 5620148855, which recommends approval and has no inline thread; its
  body lists two actionable assertions (audit F32 and F33). The session ended before they were processed.
- 2026-10-23 07:16 UTC - Verified each claim against the current tree after the restart. Fixed F31 and F33 in two signed
  commits, F30 in one audit commit, and answered F32 with a no-action explanation (the path exists on `develop`);
  pushed the commits, then replied on the F30 and F31 threads and in one pull-request comment for F32 and F33, and
  resolved the two threads. GraphQL data shows 0 unresolved threads.

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
