---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2402-1840-cancel-superseded-pr-runs/ISSUE.md
last-updated-utc: 2026-10-04 19:07
semantic-links:
  related-artifacts:
    - docs/issues/closed/2402-1840-cancel-superseded-pr-runs/ISSUE.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
---

# Manual Verification Evidence

## Purpose

Record real verification of the workflow concurrency policy delivered by
`ci(workflows): [#2402] cancel superseded PR runs`.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-02 to 2026-10-04.
- Artifact under test: the merged #2402 workflow concurrency policy in `Container` and `Testing`.
- Operating system / environment: maintainer Linux desktop with authenticated GitHub CLI and GitHub
  Actions.
- Prerequisites and setup performed: disposable draft PRs #2427 (verification) and #2426 (control)
  were opened for V1. V2 used the approved `josecelano/torrust-tracker` fork after its `develop`
  branch was fast-forwarded to merged #2419; the branch was restored to upstream after verification.

## Verification Processes

### V1 - Superseded Pull-Request Run (M1)

- Goal: prove a new pull-request commit cancels the older `Container` and `Testing` runs without
  cancelling a different pull request's runs.
- Initial state: draft verification PR #2427 commit A (`88d41eaa0c808a9313b78d6974394633687fcea6`)
  and draft control PR #2426 commit A (`a30ea9137b8a05c16aa8e816362b965c252f22d1`) each
  had active target workflows. The control `Container` run `37120097931`, verification `Container`
  run `37120102585`, and verification `Testing` run `37120102549` had all started when commit B
  was pushed.
- Status: `DONE`

#### Steps Performed

1. Pushed control commit A to draft PR #2426 and verification commit A to draft PR #2427.
2. Confirmed the control `Container` and verification `Container` and `Testing` runs were active.
3. Pushed signed, superseding verification commit B (`0ef73d9d043b2b11946af30492bd7b288a457854`).
4. Queried the four verification runs and both control runs with `gh run view`.

#### Observed Result

```text
verification A 88d41eaa: Container 37120102585 cancelled at 11:43:30 UTC
verification A 88d41eaa: Testing   37120102549 cancelled at 11:43:46 UTC
verification B 0ef73d9d: Container 37120568280 success at 12:10:51 UTC
verification B 0ef73d9d: Testing   37120568237 success at 11:57:26 UTC
control A a30ea913: Container 37120097931 success at 11:58:16 UTC
control A a30ea913: Testing   37120097978 success at 11:53:28 UTC
```

#### Attempted Queued-Run Follow-up

- Initial state: draft verification PR #2427 commit A
  (`e07983983764be55e16c818cc0ead98460a38d7d`) had `Container` and `Testing` workflow runs shown
  as queued behind the active `Container` run `37146648899` for the independent control PR.
- Action: pushed signed superseding commit B
  (`31f6af43388eb64e08b0bfc0893d5e241f689cd8`) while both A target workflows remained queued.

```text
verification A e0798398: Container 37146718438 cancelled at 19:10:22 UTC
verification A e0798398: Testing   37146718351 cancelled at 19:10:22 UTC
verification B 31f6af43: Container 37146927907 success at 19:31:19 UTC
verification B 31f6af43: Testing   37146927797 success at 19:28:37 UTC
control     275da262: Container 37146648899 success at 19:21:12 UTC
```

#### Conclusion

The first run proved cancellation of active target workflows and isolation from a concurrent
control pull request. Independent review of the queued-run follow-up found that the A target jobs
had started before B cancelled their workflow runs. It therefore confirms running-run cancellation
again, but cannot prove queued-job cancellation. M1 and AC1 remain in progress.

#### Queued-Container Scenario

- Initial state: control PR #2426 commit `99b47d677f6d58c0f27c8202b7953f2c6576de25` had its
  `Container` run `37192505398` in progress. Verification PR #2427 commit A
  (`be79b16a94fda5cfa72a61570ae9d51de28cfa04`) had `Container` run `37192733320` queued. Its
  `Test (Docker) (release)` job had `status: queued`, no assigned `runner_name`, and required the
  `self-hosted` and `torrust-hetzner` labels immediately before the superseding push.
- Action: pushed signed verification commit B
  (`4cbe7a8d7979f7583b4828be367b53b64468e79d`).

```text
verification A be79b16a: Container 37192733320 cancelled at 09:42:33 UTC
verification A be79b16a: Testing   37192733336 cancelled at 09:42:49 UTC
verification B 4cbe7a8d: Container 37193016174 success at 09:57:20 UTC
verification B 4cbe7a8d: Testing   37193016215 success at 09:57:26 UTC
control     99b47d67: Container 37192505398 success at 09:44:47 UTC
```

GitHub set `started_at` when dispatching the queued self-hosted job, so this scenario treats
`status: queued` and no assigned runner as the evidence that the job had not begun execution.

#### Final Conclusion

The queued-container scenario proves that a superseding commit cancels a queued `Container` run
before a runner is assigned. It also cancelled the active `Testing` run, scheduled successful B
replacements, and did not cancel the independent control `Container` run. Together with the first
running-run scenario, this completes M1, AC1, and AC3.

### V2 - Push Runs Are Kept (M2)

- Goal: prove overlapping `develop` push runs do not cancel or replace one another.
- Initial state: upstream did not have a suitable pair of merges to overlap. The approved fork
  fallback ran both workflows on GitHub-hosted runners. Commit A
  (`fdcfd310a1017e04b4085cd6de4be454ecb23ac3`) had `Container` and `Testing` in progress before
  commit B (`6856a8f72587dcc8a2642d76418684f0dc12789c`) was pushed.
- Status: `DONE`

#### Steps Performed

1. Fast-forwarded the fork's `develop` branch to the upstream commit containing #2419.
2. Pushed commit A with a non-documentation configuration-fixture comment, then confirmed its
   `Container` run `37114955803` and `Testing` run `37114955629` were active.
3. Pushed commit B with a second non-documentation fixture comment while both commit A target runs
   remained active.
4. Queried all four target runs with `gh run view`, then restored fork `develop` to the upstream
   tip with a lease-protected force push.

#### Observed Result

```text
commit A fdcfd310: Container 37114955803 success at 10:44:37 UTC
commit A fdcfd310: Testing   37114955629 success at 10:13:26 UTC
commit B 6856a8f7: Container 37115535710 success at 10:51:57 UTC
commit B 6856a8f7: Testing   37115535700 success at 10:22:56 UTC
```

The fork's `Publish (Development)` jobs were `skipped`: their workflow condition permits publishing
only from `torrust/torrust-tracker`. That limitation does not affect the concurrency result, which
is the behavior covered by AC2.

#### Conclusion

Both overlapping `develop` pushes completed their `Container` and `Testing` workflows successfully;
none was cancelled or replaced. This satisfies M2 and AC2. The fallback cannot independently prove
upstream image publication.

## Failures and Follow-up

- An empty successor commit did not schedule either workflow because the relevant path filters
  ignore an empty diff. The valid V2 evidence uses the later two commits that each changed the
  development configuration fixture.
- The initial M1 control run completed before the rerun and could not validate cross-pull-request
  isolation. Independent review identified this gap; V1 was rerun with concurrent control PR #2426.
- The corrected V1 rerun initially covered only running target workflows. The attempted queued-run
  follow-up also had already-started target jobs. The final queued-container scenario instead
  recorded GitHub's actual pre-execution evidence: `status: queued` with no assigned runner.
- The fork fallback cannot execute the repository-gated `Publish (Development)` jobs. The
  concurrency behavior was verified; the publication limitation is recorded in
  `implementation-retrospective.md`.
