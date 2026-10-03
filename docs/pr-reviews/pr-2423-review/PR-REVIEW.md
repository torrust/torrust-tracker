---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2406"
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore unreassigned -->

# PR #2423 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2423>.

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

The review overview rates F1, F2, and F3 `Medium severity` and F4 and F5 `Low severity` in its
badge markup; as in the PR #2397 audit, they are recorded as `Minor (inferred)` and
`Nit (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2423-f1` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2423-f2` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2423-f3` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2423-f4` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2423-f5` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2423-f6` | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2423-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2423-f8` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2423-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2423-f10` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2423-f11` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2423-f12` | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2423-f13` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2423-f14` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2423-f15` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2423-f16` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2423-f17` | Human | Minor | metadata | RE_RAISE_OF:F14 | FIXED | RESOLVED |
| F18 | `review-finding:pr-2423-f18` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F19 | `review-finding:pr-2423-f19` | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - UDP completion helper reuses one transaction id

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041437>
- Concern: Both announces in `udp_complete_download` used transaction id 2, so responses could not be told apart if delayed or reordered.
- Solution: The helper builds each announce with its own transaction id (2 and 3), asserts each response echoes its request id, and the scrape uses id 4. Request construction was split from sending to stay within Clippy's argument limit.
- Current-tree verification: `cargo test --test persistence-scrape-after-restart --test metrics-fixed-ports --test metrics-port-zero` passed; workspace pedantic Clippy clean.
- Resolution reference: `test(test-helpers): [#2406] use a distinct transaction id per UDP request`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169552847>

### F2 - Relative ADR link in ScrapeHandler rustdoc

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041513>
- Concern: The repository-relative `../../../docs/...` link does not resolve in rendered Rust documentation.
- Solution: The link now uses the absolute GitHub URL, the form already used in `packages/configuration/src/v3_0_0/mod.rs`.
- Current-tree verification: `grep -rn 20261002173716 --include=*.rs packages` shows only absolute URLs.
- Resolution reference: `docs(tracker-core): [#2406] link the scrape ADR by absolute URL in rustdoc`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553043>

### F3 - Relative ADR link in persisted_downloads module docs

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041582>
- Concern: The repository-relative `../../../../docs/...` link in the module docs does not resolve in rendered Rust documentation.
- Solution: Same change as F2, in the same commit.
- Current-tree verification: same `grep` as F2.
- Resolution reference: `docs(tracker-core): [#2406] link the scrape ADR by absolute URL in rustdoc`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553261>

### F4 - Repeated info-hashes inflate the batch lookup

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041649>
- Concern: A request repeating an info-hash absent from memory bound it more than once in the `IN (...)` query.
- Solution: Absent info-hashes are collected in a `BTreeSet` before the batch lookup; response order is unaffected. New unit test `it_should_look_up_a_repeated_info_hash_absent_from_memory_only_once` failed before the change (the mock received the hash twice) and passes after.
- Current-tree verification: `cargo test -p torrust-tracker-core` passed (151 unit, 15 doc, 8 integration).
- Resolution reference: `perf(tracker-core): [#2406] deduplicate scrape info-hashes before the persisted lookup`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553531>

### F5 - Typo in in-memory repository docs

- PR number: 2423
- Source review ID: 5395864035
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169041714>
- Concern: "underling" should be "underlying".
- Solution: Corrected all ten occurrences in `in_memory.rs`, including the pre-existing ones and a stray "error.s".
- Current-tree verification: `grep -c underling packages/tracker-core/src/torrent/repository/in_memory.rs` returns 0.
- Resolution reference: `docs(tracker-core): [#2406] fix 'underling' typo in in-memory repository docs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4169553774>

### F6 - Restart test races the asynchronous persistence write

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391139>
- Concern: `udp_complete_download` returns when the announce response arrives, but the persistence listener writes the row later, and `restart()` cancels the listener without draining queued events, so the root restart test was order-dependent.
- Solution: Before restarting, the test waits under a 5-second deadline until the torrent metrics store holds 1 download for the completed torrent (`wait_for_persisted_downloads`), mirroring `wait_for_global_downloads_persisted` in tracker-core.
- Current-tree verification: `cargo test --test persistence-scrape-after-restart` passed 20 of 20 runs; workspace pedantic Clippy clean.
- Resolution reference: `test(tracker): [#2406] wait for the persisted download before restarting`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172937527>

### F7 - Stability note claims a shutdown guarantee

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391146>
- Concern: The evidence inferred from 20 passing runs that the persistence write completes before graceful shutdown, which the listener's biased cancellation does not guarantee.
- Solution: The note now names the wait the test uses, explains why it is needed, and records that the earlier claim was a sample.
- Current-tree verification: `manual-verification-evidence.md` T8 stability paragraph inspected; `linter all` passed.
- Resolution reference: `docs(issues): [#2406] correct the restart-test stability claim and the principle's location`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172937646>

### F8 - Disabled-mode contract ignores peerless cleanup

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391149>
- Concern: The contract said disabled mode counts completions since the tracker started, but default peerless cleanup removes the swarm and its count.
- Solution: The `ScrapeHandler` Rustdoc, `policy.rs`, research section 5, and the spec decision bullet now say the count covers completions while the swarm has been in memory and is reset by peerless cleanup.
- Current-tree verification: `grep` for "since the tracker started" in the touched files returns no contract text; library docs build clean.
- Resolution reference: `docs(tracker-core): [#2406] state that peerless cleanup resets the in-memory downloaded count`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172937731>

### F9 - F2's recorded grep no longer reproduces

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391154>
- Concern: After `docs: [#2406] separate the scrape behavior contract from the lookup ADR`, F2's recorded `grep` also matches three plain `// adr:` comments, so its "only absolute URLs" result is stale.
- Solution: A processing-log entry records the narrowed Rustdoc-only command; F2's original verification is left as recorded at the time.
- Current-tree verification: `grep -rnE '//[!/].*20261002173716' --include=*.rs packages` prints only the two absolute URLs.
- Resolution reference: `docs(pr-reviews): [#2406] audit Cameron's review on #2423`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938010>

### F10 - Spec points the principle to the ADR

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391162>
- Concern: After the ADR split, the spec still said the scrape principle was recorded in the ADR.
- Solution: The decision paragraph and the 17:40 log entry now point the principle to the `ScrapeHandler` Rustdoc and the ADR to the lookup design.
- Current-tree verification: `ISSUE.md` decision paragraph and 17:40 log entry inspected; `linter lychee` passed.
- Resolution reference: `docs(issues): [#2406] correct the restart-test stability claim and the principle's location`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938348>

### F11 - PR body omissions and commit count

- PR number: 2423
- Source review ID: 5399817905
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172391170>
- Concern: The PR body omitted touched packages and files and the draft follow-up, and said Copilot's five findings were fixed in separate commits although F2 and F3 share one. Round 2 noted the omissions grew with the rustdoc commit.
- Solution: The PR body now lists every touched package and file group, the draft specs, and the rustdoc, hook, CI, and template commits, and states that F1-F5 were fixed in four commits.
- Current-tree verification: `gh pr view 2423 --json body` matches `git diff --name-only torrust/develop...HEAD` grouped by package.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938486>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938486>

### F12 - Library rustdoc is not gated

- PR number: 2423
- Source review ID: 5400241273
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172733545>
- Concern: The pre-push hook documents only `--bins --examples` and no workflow builds rustdoc, so broken library links can land again.
- Solution: The hook has a separate `--lib` documentation step (a combined `--lib --bins` run collides on dev-tool crates whose lib and bin share a name), and the nightly unit job in `testing.yaml` builds library docs. Skills and `AGENTS.md` updated.
- Current-tree verification: pre-push passed 5 of 5 steps; `linter yaml` clean.
- Resolution reference: `ci(testing): build library rustdoc on the nightly unit job`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938592>

### F13 - De-linked factory names a removed function

- PR number: 2423
- Source review ID: 5400241273
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172733551>
- Concern: The `udp-core` and `udp-server` statistics docs described a `factory` function that no longer exists.
- Solution: Both docs now link `UdpTrackerCoreServices::initialize_from` and `UdpTrackerServerServices::initialize`, and the stale factory text and example are removed.
- Current-tree verification: no `factory` text remains in either file; `cargo +nightly doc --no-deps --lib` for both crates is clean.
- Resolution reference: `docs(udp-core,udp-server): name the real constructors in the statistics service docs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172938699>

### F14 - Audit lacks rows and replies for the human review

- PR number: 2423
- Source review ID: 5400353319
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172827202>
- Concern: The audit carried only Copilot's F1-F5 and no human-review thread had a reply, although a commit already acted on F12.
- Solution: The audit records F6-F15 with detail entries, and every human-review thread has a reply citing its resolution.
- Current-tree verification: rows and detail entries for F6-F15 are present; `validate-audit-record.py --pr-number 2423 --base torrust/develop` reports 0 failures; `reply-status` reports a reply on every thread.
- Resolution reference: `docs(pr-reviews): [#2406] record round-3 findings F14 and F15 on #2423`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172974805>

### F15 - Drafts omit the completion-review conditions

- PR number: 2423
- Source review ID: 5400353319
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172827203>
- Concern: Both draft specs kept only "Retrospective: Not yet assessed" instead of the template's retrospective conditions that `create-issue` requires.
- Solution: Both drafts now carry the template's Implementation Completion Review guidance.
- Current-tree verification: both drafts' completion-review sections match `docs/templates/ISSUE.md`; `linter markdown` passed.
- Resolution reference: `docs(issues): add the completion-review conditions to both draft specs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4172974889>

### F16 - Reviewer finding ID filled without a reassignment

- PR number: 2423
- Source review ID: 5400576772
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4173005959>
- Concern: F6-F15 recorded the reviewer's IDs as `Reviewer finding ID`, although that field is for IDs reassigned on collision and none was reassigned.
- Solution: `Reviewer finding ID` is `N/A` for F6-F15, and a Processing Log entry corrects the 08:52 entry.
- Current-tree verification: `grep -cE '^- Reviewer finding ID: N/A$'` returns 16 (F1-F16); F17, the only reassigned ID, records F14.
- Resolution reference: `docs(pr-reviews): [#2406] cite the commit that recorded F14 and clear unreassigned reviewer IDs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174460195>

### F17 - F14 reply claimed rows that were not on the branch

- PR number: 2423
- Source review ID: 5400576772
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4173005956>
- Concern: Re-raise of F14. At the round-4 head, `docs(issues): add the completion-review conditions to both draft specs`, the F14 reply said `FIXED` while the audit had no F14 or F15 rows, no Processing Log entry for review 5400353319, and the cited commit did not contain the fix.
- Solution: The F14 and F15 rows landed in `docs(pr-reviews): [#2406] record round-3 findings F14 and F15 on #2423`. F14 now cites that commit, the Processing Log has the 10:40 UTC round-3 entry, and a correction records that the reply preceded the rows. The reviewer's ID F14 collides with the original finding, so this re-raise is F17.
- Current-tree verification: F14 and F15 rows, detail entries, and Resolution references inspected; `validate-audit-record.py --pr-number 2423 --base torrust/develop` reports 0 failures.
- Resolution reference: `docs(pr-reviews): [#2406] cite the commit that recorded F14 and clear unreassigned reviewer IDs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174460116>

### F18 - Record lines identify branch heads by commit id

- PR number: 2423
- Source review ID: 5402386212
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174518634>
- Concern: The 10:40 and 11:43 log entries and the F17 Concern named review heads by commit id, which a rebase rewrites.
- Solution: The F17 Concern names the round-4 head by subject, and an appended log correction names the round-3 and round-4 heads by subject; the two log entries are left as written.
- Current-tree verification: the only commit ids left in the record are in the 10:40 and 11:43 entries, and the 19:47 correction covers both.
- Resolution reference: `docs(pr-reviews): [#2406] name review heads by subject and scope the spelling exception to the audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174716719>

### F19 - Ordinary word added to the project dictionary

- PR number: 2423
- Source review ID: 5402386212
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174518641>
- Concern: `unreassigned` is ordinary English, not a technical term, so it does not belong in `project-words.txt`.
- Solution: The word is removed from `project-words.txt`, and this record carries a file-local `cspell:ignore` directive, as the PR #2344 audit does.
- Current-tree verification: `grep -c unreassigned project-words.txt` returns 0; `linter cspell` passed.
- Resolution reference: `docs(pr-reviews): [#2406] name review heads by subject and scope the spelling exception to the audit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2423#discussion_r4174716823>

## Processing Log

- 2026-10-02 19:21 UTC - Copilot review 5395864035 submitted five inline findings.
- 2026-10-02 20:28 UTC - Fixed all five findings in separate commits, rebased onto the latest `develop`, pushed after the pre-push suite passed, replied to each thread, and started this audit.
- 2026-10-03 08:52 UTC - Human review 5399817905 (da2ce7, round 1) requested changes with findings F6-F11; the reviewer's IDs continue this audit's numbering and are kept.
- 2026-10-03 10:17 UTC - Human review 5400241273 (da2ce7, round 2) kept the request and added F12 and F13.
- 2026-10-03 10:40 UTC - Human review 5400353319 (da2ce7, round 3, at `ae318f38e`) kept the request and added F14 and F15.
- 2026-10-03 11:21 UTC - Correction to the 20:28 entry: F2 and F3 were fixed in one commit, so the five findings took four commits. Correction to F2's verification (F9): it now also matches three plain `// adr:` comments; the Rustdoc-only check is `grep -rnE '//[!/].*20261002173716' --include=*.rs packages`. Fixed F6-F13, pushed after the pre-push suite passed, updated the PR body (F11), and replied to each thread.
- 2026-10-03 11:32 UTC - Human review 5400353319 (da2ce7, round 3, at an earlier head) added F14 and F15. Fixed F15, recorded both, replied to both threads.
- 2026-10-03 11:43 UTC - Human review 5400576772 (da2ce7, round 4, at `cdbd2c5ed`) kept the request, re-raised F14, and added F16.
- 2026-10-03 12:13 UTC - Correction to the 11:32 entry: the F14 reply said `FIXED` and cited `docs(pr-reviews): [#2406] audit Cameron's review on #2423`, but the F14 and F15 rows landed only in `docs(pr-reviews): [#2406] record round-3 findings F14 and F15 on #2423`, pushed at 12:01 UTC after a pre-push failure caused by a full disk. F14 now cites that commit. Correction to the 08:52 entry: no reviewer ID was reassigned, so `Reviewer finding ID` for F6-F15 is now `N/A` (F16).
- 2026-10-03 19:06 UTC - Recorded round 4 as F16 and F17 (re-raise of F14, renumbered because the ID is taken). Pushed the fix before replying to either thread.
- 2026-10-03 19:47 UTC - Human review 5400661243 (da2ce7, round 5, 12:09 UTC, at `docs(pr-reviews): [#2406] record round-3 findings F14 and F15 on #2423`) raised no new finding; its pre-merge list is the work recorded as F16 and F17, and the 19:05 push dismissed it. Human review 5402386212 (da2ce7, round 7, 19:25 UTC) approved and added F18 and F19. Correction to the 10:40 and 11:43 entries (F18): the round-3 head is `docs(issues): draft ADR task for prose-style tests as the executable specification` and the round-4 head is `docs(issues): add the completion-review conditions to both draft specs`; heads are named by subject because a rebase rewrites commit ids.
- 2026-10-03 20:24 UTC - Recorded F18 and F19. Pushed the fix before replying to either thread.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
