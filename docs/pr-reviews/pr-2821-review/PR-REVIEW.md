---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2821 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2821.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is da2ce7's review `5857799147` (F1 to F11) and Copilot's review `5857834791` (F12 to F16, numbered after the reviewer-provided IDs). Both were processed together. Round 2 is da2ce7's review `5858134220` (F17 to F22, reviewer IDs F13 to F18 reassigned on collision) and Copilot's review `5858293840` (F23 to F26).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2821-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2821-f2 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2821-f3 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2821-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2821-f5 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2821-f6 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2821-f7 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2821-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2821-f9 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2821-f10 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2821-f11 | Human | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2821-f12 | Copilot | Minor | correctness | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2821-f13 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2821-f14 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2821-f15 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2821-f16 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2821-f17 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2821-f18 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2821-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2821-f20 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2821-f21 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2821-f22 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2821-f23 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2821-f24 | Copilot | Minor (inferred) | testing | RE_RAISE_OF:F17 | NO_ACTION | SUPERSEDED |
| F25 | review-finding:pr-2821-f25 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F18 | NO_ACTION | SUPERSEDED |
| F26 | review-finding:pr-2821-f26 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F21 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The printed-location count is 23, not 22

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138758
- Concern: The one-line grep missed the three-line attribute in peer_ip_resolver.rs, so the counts and the S2 list were short by one variant.
- Solution: Recounted with a multiline search, updated every derived count and the evidence command.
- Current-tree verification: Ran rg -U --multiline-dotall on packages: 23 matches over 10 files; read peer_ip_resolver.rs:117-120.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537222798

### F2 - The option-1 scope has no site to apply to

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138772
- Concern: P5a, P6 and P7 to P10 are reached through TryFrom or async blocks, so a track_caller chain ends at a library line.
- Solution: Added fact 6, removed option 1 from the recommendation and corrected the comparison row.
- Current-tree verification: Read the extractors (announce_request.rs, scrape_request.rs, authentication_key.rs) and the http-core handlers.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537223334

### F3 - The semver table ignores the first-publish plan

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138781
- Concern: The 0.1.0 crates are unpublished and primitives and configuration already owe 4.0.0; axum-server was missing and two rows named a LocatedError that is not in src.
- Solution: Restated the table against the publish plan, added axum-server, corrected the http-core and axum-http-server rows and decision 1.
- Current-tree verification: Read 2583-1669-first-publish-of-workspace-crates/ISSUE.md and grepped LocatedError per package.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537223848

### F4 - S2 lists registry-crate sites and a red test that cannot pass

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138788
- Concern: The torrust-info-hash sites cannot be changed in the workspace and the wrapping sites print a second location until S3.
- Solution: S2 excludes the three wrapping sites; S3 carries them with its LocatedError decision.
- Current-tree verification: Read announce.rs:543 and :560, scrape.rs:84 and inventory.md standalone crates.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821; docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537224443

### F5 - S5 red test exercises S4

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138800
- Concern: The UDP variants are built only in test code and the whitelist rejection is S4's change.
- Solution: S5 tests the TLS and configuration errors directly; S6 gives the UDP variants direct formatting tests.
- Current-tree verification: Checked where Internal and AuthRequired are built.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821; docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537224889

### F6 - Fact 3 overstates how many messages carry the input

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138806
- Concern: Only some of the 23 formats print the offending input.
- Solution: Recounted as 8 with input, 3 with free text and 12 with neither; churn row updated.
- Current-tree verification: Read all 23 attributes.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537225248

### F7 - Option 1 does not fix the LocatedError text

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138812
- Concern: The three existing track_caller impls still print the location.
- Solution: The cell now says No.
- Current-tree verification: Read inventory.md T2 table.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537225664

### F8 - PR description does not follow open-pull-request

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138819
- Concern: Wrong issue link and no validation section.
- Solution: Edited the PR body: Related to #2804 and a Validation section.
- Current-tree verification: Read the PR body after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537226087
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537226087

### F9 - Frontmatter last-updated-utc is stale

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138824
- Concern: The field was older than the latest progress-log entry.
- Solution: Updated the field with the entry.
- Current-tree verification: Read ISSUE.md frontmatter.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821; docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537226456

### F10 - The T4 row is not padded

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138826
- Concern: Unlike the other rows of the plan table.
- Solution: Padded to the column widths.
- Current-tree verification: Compared row lengths (equal).
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537226810

### F11 - options.md lacks frontmatter

- PR number: 2821
- Source review ID: 5857799147
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537138834
- Concern: Siblings carry doc-type, issue-spec and last-updated-utc.
- Solution: Added the block.
- Current-tree verification: Read the file head.
- Resolution reference: docs(issues): rework the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537227151

### F12 - Multi-line attribute missed (same as F1)

- PR number: 2821
- Source review ID: 5857834791
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537167723
- Concern: Same finding as F1.
- Solution: Superseded by F1.
- Current-tree verification: Compared with F1.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537227515
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537227515

### F13 - An additive constructor is not a breaking change

- PR number: 2821
- Source review ID: 5857834791
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537167771
- Concern: Option 2 cell assigned a break to a constructor that adds no field change.
- Solution: Cell now says none for an additive constructor.
- Current-tree verification: Reasoned from the enum literal rules; no field changes in option 2.
- Resolution reference: docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537227927

### F14 - configuration prints four LocatedError sources

- PR number: 2821
- Source review ID: 5857834791
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537167816
- Concern: The table said none for configuration.
- Solution: Row lists the four variants and the patch versus breaking distinction; axum-server has its own row.
- Current-tree verification: Read configuration/src/lib.rs:312-337.
- Resolution reference: docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537228373

### F15 - InvalidPeerIdParam wrapping site missing

- PR number: 2821
- Source review ID: 5857834791
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537167857
- Concern: announce.rs:560 wraps with Located(err).into().
- Solution: Added to the S3 wrapping sites with a short peer_id case.
- Current-tree verification: Read announce.rs:555-562.
- Resolution reference: docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537228774

### F16 - S5 red test exercises S4

- PR number: 2821
- Source review ID: 5857834791
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537167896
- Concern: Same area as F5; asked for direct formatting and startup-error tests.
- Solution: S5 and S6 use direct tests.
- Current-tree verification: Same checks as F5.
- Resolution reference: docs(issues): add the Copilot findings of PR #2821 to the T4 options
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537229220

### F17 - Half of S5's red test is already green

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408388
- Concern: A missing certificate gives MissingTlsConfig, whose text holds no location.
- Solution: S5 tests invalid PEM files (BadTlsConfig) and a bad configuration file; MissingTlsConfig is left to the field decision.
- Current-tree verification: Read tls.rs:10-55.
- Resolution reference: docs(issues): correct the S3 and S5 slices of the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537594073

### F18 - S5 version step is wrong for configuration

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408394
- Concern: configuration is published at 3.0.0 and planned at 4.0.0; only axum-server is a first publish.
- Solution: S5 gives a step per crate.
- Current-tree verification: Read the publish plan row for configuration.
- Resolution reference: docs(issues): correct the S3 and S5 slices of the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537594549

### F19 - Line range starts inside another variant

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408400
- Concern: The four LocatedError variants span lib.rs:312-337, not 305-337.
- Solution: Corrected in options.md and in the F14 audit entry.
- Current-tree verification: Read configuration/src/lib.rs:300-340.
- Resolution reference: docs(issues): correct the S3 and S5 slices of the T4 options after review of PR #2821; docs(pr-reviews): correct the relationships, IDs and stamps in the PR #2821 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537594939

### F20 - Reviewer finding ID is inverted and severities are marked inferred

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408405
- Concern: The ID is recorded only when reassigned; bracketed severities are not inferred.
- Solution: F1 to F11 carry N/A, F12 to F16 their original IDs; F12 to F16 are Minor.
- Current-tree verification: Read the skill's Required Audit Fields.
- Resolution reference: docs(pr-reviews): correct the relationships, IDs and stamps in the PR #2821 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537595240

### F21 - F12 is a re-raise of F1 recorded as ORIGINAL

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408409
- Concern: A later request for the same change is RE_RAISE_OF.
- Solution: F12 is RE_RAISE_OF:F1 with NO_ACTION and SUPERSEDED kept.
- Current-tree verification: Read skill step 3.
- Resolution reference: docs(pr-reviews): correct the relationships, IDs and stamps in the PR #2821 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537595650

### F22 - Log stamps precede the replies they follow

- PR number: 2821
- Source review ID: 5858134220
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537408415
- Concern: Replies were posted 11:18:23Z to 11:18:59Z, before the 11:19 entries.
- Solution: Added a correction entry naming both stamps.
- Current-tree verification: Read the replies' created_at with the forge API.
- Resolution reference: docs(pr-reviews): correct the relationships, IDs and stamps in the PR #2821 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537595976

### F23 - Option (b) cannot fix the info-hash paths

- PR number: 2821
- Source review ID: 5858293840
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537536719
- Concern: torrust_info_hash::ConversionError prints its own location, so S3's test cannot pass under (b) alone.
- Solution: Recommendation, decision 2 and S3 require (a) or an upstream fix for those paths.
- Current-tree verification: Read inventory.md T2 row for the standalone crates.
- Resolution reference: docs(issues): correct the S3 and S5 slices of the T4 options after review of PR #2821
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537596366

### F24 - S5 missing-certificate case is already green

- PR number: 2821
- Source review ID: 5858293840
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537536775
- Concern: Same finding as F17.
- Solution: Superseded by F17.
- Current-tree verification: Compared with F17.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537596725
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537596725

### F25 - S5 version step is wrong for configuration

- PR number: 2821
- Source review ID: 5858293840
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537536827
- Concern: Same finding as F18.
- Solution: Superseded by F18.
- Current-tree verification: Compared with F18.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537597092
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537597092

### F26 - F12 relationship is ORIGINAL

- PR number: 2821
- Source review ID: 5858293840
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537536876
- Concern: Same finding as F21.
- Solution: Superseded by F21.
- Current-tree verification: Compared with F21.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537597501
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2821#discussion_r4537597501

## Processing Log

- 2026-11-11 11:19 UTC - Started audit for round 1 (both reviews).
- 2026-11-11 11:19 UTC - Replied on the sixteen threads and resolved them; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-11 11:56 UTC - Correction of the two entries above, which are stamped 11:19: the sixteen replies were posted between 11:18:23Z and 11:18:59Z (forge `created_at`), before the audit record was started, because the record holds the reply URLs; the entry order shows the start first. The thread resolution was read back at 11:19; the individual resolution times were not captured.
- 2026-11-11 11:56 UTC - Corrected the `Reviewer finding ID` of F1 to F16, the severity of F12 to F16 and the relationship of F12 after the second review (F20, F21 and F22 of the round 2 findings below).
- 2026-11-11 12:00 UTC - Started round 2 (reviews 5858134220 and 5858293840); replied on the ten threads (forge `created_at` 11:59:50Z to 12:00:14Z) and resolved them; a GraphQL refresh showed 0 unresolved threads.
- 2026-11-11 12:23 UTC - Correction of the 12:00 entry for round 2: round 2 started by 11:55:55Z, the commit time of `docs(issues): correct the S3 and S5 slices of the T4 options after review of PR #2821`; the second fix commit, `docs(pr-reviews): correct the relationships, IDs and stamps in the PR #2821 audit`, was committed at 11:57:01Z. Only the replies (11:59:50Z to 12:00:14Z) and their resolution fit the 12:00 stamp.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
