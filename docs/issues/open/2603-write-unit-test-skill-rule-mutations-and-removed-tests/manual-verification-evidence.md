---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2603-write-unit-test-skill-rule-mutations-and-removed-tests/ISSUE.md
last-updated-utc: "2026-10-21 12:25"
---

# Manual Verification Evidence

<!-- cspell:ignore remov delet -->
<!-- The two words are fragments of the grep pattern recorded verbatim in V2, step 5. -->

## Purpose

Record the two walk-throughs of #2603: whether the two new rules of the `write-unit-test` skill are
specific enough to prompt the missing action when applied to the real, merged artifacts of #2415.
Nothing here runs the tracker. The artifacts are Git history and the #2415 evidence, so the
commands are `git` and `grep`.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21 from 11:50.
- Artifact under test: the skill as changed by `docs(skills): [#2603] ask for a mutation per clause of a rule a test guards` and `docs(skills): [#2603] ask for a record when a refactor removes or merges a test`, on top of the merge `Merge torrust/torrust-tracker#2604: docs(issues): add issue specification for #2603`. The #2415 artifacts are on `develop` and are named by their subjects.
- Operating system / environment: Linux; `git` and `grep` only. No Rust toolchain was used: the root package was not built in this worktree, which is why V1 cites mutation outcomes instead of re-running them.
- Prerequisites and setup performed: none.

## Verification Processes

### V1 - M1: the per-clause rule on the #2415 release rule

- Goal: show that applying the new section "Prove a Test Guards Each Clause of a Rule" to the #2415 rule
  yields the mutation that the original test missed.
- Initial state: the rule is in the #2415 spec: "A dropped unspawned reservation must release its
  state without cancelling unrelated components."
- Status: `DONE`, with the limits below.

#### Steps Performed

1. Read the sentence in `docs/issues/closed/2415-2410-si-22-3-per-component-cancellation-tokens/ISSUE.md`
   (`sed -n 72,78p`).
2. Applied the four steps of the new section to that one sentence. The paragraph states other rules
   too (cancellation before registration, idempotent `cancel()`, a foreign reservation); step 4, "Bound
   it", keeps this walk-through to one rule.
3. Compared the result with the mutation tables in V1 and V4 of #2415's
   `manual-verification-evidence.md`.

#### Observed Result

Clauses (step 1):

| Clause | Observable |
| --- | --- |
| C1: the dropped reservation releases its state | after the drop, a later `cancel()` leaves the dropped reservation's token uncancelled, because the manager no longer holds it |
| C2: without cancelling unrelated components | the drop itself leaves every other component's token uncancelled |

One mutation per clause, each breaking only that clause (step 2), compared with #2415's tables:

| Clause | Mutation derived by the rule | Same mutation in #2415's evidence | Outcome recorded there |
| --- | --- | --- | --- |
| C1 | a dropped reservation is not released | V1, M4 | caught by `it_should_release_a_dropped_reservation_without_affecting_other_components` and `it_should_reject_a_reservation_issued_by_another_manager` |
| C2 | a drop cancels every other registered token | V4, M7b | passed the test as it was before the review (the gap); fails the strengthened test with "dropping one reservation must not cancel another component" |

The rule also gives the counter-example of step 2, a mutation that breaks both clauses: a drop that
cancels every registered token, its own included. #2415's V4 records it as M7: it failed the old test
through the C1 assertion, which is how a weak C2 assertion stayed hidden.

#### Conclusion

The section's steps are enough to produce the C2 mutation, M7b, which is the one the original test
missed, and to explain why M7 could not expose the gap. Two limits:

- I wrote both the skill text and the #2415 work, and I knew the reviewer's finding when I applied the
  rule. This shows the steps are specific enough to yield the mutation; it does not show that an author
  who did not know the finding would have written it.
- The mutation outcomes in the last column are cited from #2415's evidence, not re-run here.

### V2 - M2: the removed-test rule on the #2415 conversion

- Goal: show that the section "Remove or Merge a Test Only With a Record" would have flagged the test
  that the #2415 conversion removed, and names what must be recorded.
- Initial state: the conversion commit `refactor(bootstrap): [#2415] give every application component its own reservation token`, on `develop`.
- Status: `DONE`.

#### Steps Performed

1. Found the commit by its subject: `git log torrust/develop --format='%H' --grep='^refactor(bootstrap): \[#2415\] give every application component its own reservation token$'` printed one commit.
2. Listed the test functions that the commit's diff to `src` removes and adds:
   `git show <commit> -- src | grep -E '^-\s*(async )?fn it_should_'` and the same with `^\+`.
3. Counted the tests of `src/bootstrap/jobs/manager.rs` before and after the commit:
   `git show <commit>^:src/bootstrap/jobs/manager.rs | grep -cE '^\s*(async )?fn it_should_'`, and the same on `<commit>`.
4. Checked whether the replacement exists before the commit:
   `git show <commit>^:src/bootstrap/jobs/manager.rs | grep -c 'fn it_should_cancel_a_spawned_components_own_token_when_the_manager_is_cancelled'`.
   This checks that the replacement exists, not that it covers the same behavior. #2415's evidence
   (V2) states that the removed test became the same test as the replacement, and the Task Reviewer
   compared the two bodies and found the same Act and Assert (see `agent-review-reports.md`); they
   were not compared again here.
5. Checked the commit message for a record: `git log -1 --format=%B <commit> | grep -inE 'remov|delet|duplicate|count'`.

#### Observed Result

```text
removed: it_should_report_cooperatively_cancelled_jobs
added:   it_should_stop_every_started_component_cooperatively_when_the_manager_is_cancelled (in src/app.rs)
manager.rs tests before: 17  after: 16
replacement present before the commit: 1 (it_should_cancel_a_spawned_components_own_token_when_the_manager_is_cancelled)
commit message: one match, line 13, "removed, and the remaining direct `JobManager` users ..." (the adapter, not a test)
```

#### Conclusion

The rule applies to this commit: a refactor whose acceptance criterion was that the existing tests
pass removed one test. It prompts for what was missing: name the removed test
(`it_should_report_cooperatively_cancelled_jobs`), name the test that covers its behavior
(`it_should_cancel_a_spawned_components_own_token_when_the_manager_is_cancelled`, added earlier in the
change), and record the removal and the new count (17 to 16) in the evidence and the commit message.
The commit message's only match is about the removed adapter, so the record was absent, as the Task
Reviewer found. The test added by the commit is the application-level test, not a replacement, so the
replacement has to be found in an earlier commit; the rule's wording asks for it to be named, which
covers that case.

## Failures and Follow-up

None. The limits of V1 are stated in its conclusion. Whether agents follow the rules is not measured
here; the completion review of the next issue that states a multi-clause rule is the real check (see
the spec's Risks).
