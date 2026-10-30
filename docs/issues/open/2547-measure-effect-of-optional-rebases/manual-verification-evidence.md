---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
last-updated-utc: 2026-10-30 11:15
---

# Manual Verification Evidence

## Purpose

Record the spot-checks M1 and M2 of the issue #2547 specification. The measurement writes no code, so
the checks re-derive a sample of the figures in `post-change-evidence.md` by a second route.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-30 11:09 to 11:10.
- Artifact under test: `post-change-evidence.md` of this issue folder, sections 2 and 4.
- Operating system / environment: Linux host, GitHub CLI `gh` authenticated as `josecelano`.
- Deviation from the specification: M1 and M2 ask for a browser and the GitHub Actions UI. This
  session had no browser, so both were cross-checked through the GitHub API instead. The M1 check uses
  the REST timeline, where the evidence used GraphQL, so it is a second data path. The M2 check
  (`gh run view --json jobs`) is the same job data as the evidence read, through a different command.
  Neither is a visual check in the web UI; a maintainer who wants that can open the PR conversation pages and the workflow runs listed below.

## Verification Processes

### V1 - Spot-Check Force-Push Counts (M1)

- Goal: the force-push count of three merged PRs from the window matches the per-PR table of
  `post-change-evidence.md` section 2. A fourth PR with no force-pushes checks the zero case.
- Initial state: the evidence recorded 2 force-pushes for #2678, 1 for #2559, 1 for #2430, and none
  for #2677.
- Status: `DONE` (API cross-check, not a browser check)

#### Steps Performed

1. Started at 11:09:49 UTC. For each PR, counted `head_ref_force_pushed` events in the REST timeline
   and read the GraphQL count from the collected data:

```sh
gh api repos/torrust/torrust-tracker/issues/<n>/timeline --paginate \
  --jq '[.[]|select(.event=="head_ref_force_pushed")]|length'
```

#### Observed Result

```text
#2678 timeline_events=2 table_graphql=2
#2559 timeline_events=1 table_graphql=1
#2430 timeline_events=1 table_graphql=1
#2677 timeline_events=0 table_graphql=0
```

Finished at 11:09:53 UTC.

#### Conclusion

All four counts match the evidence table. Met.

### V2 - Spot-Check a CI Sample (M2)

- Goal: the summed job duration of one sampled run per workflow matches the job-minutes recorded for
  the run in `post-change-evidence.md` section 4.
- Initial state: the per-run minutes were computed on 2026-10-30 from the runs' job
  `started_at`/`completed_at`. The recorded values are the per-run
  job-minutes of `post-change-evidence.md` Appendix A.5; V2 used the first listed run of each workflow.
- Status: `DONE` (API cross-check, not the Actions UI)

#### Steps Performed

1. Started at 11:09:53 UTC. For the first sampled run of each workflow, summed job durations with:

```sh
gh run view <id> --repo torrust/torrust-tracker --json jobs \
  --jq '[.jobs[]|((.completedAt|fromdateiso8601)-(.startedAt|fromdateiso8601))]|add/60'
```

#### Observed Result

```text
Workflow                         run          recorded  gh run view
Generate Coverage Reports (PR)   39594972871  17.32     17.32
Testing                          39594972851  21.47     21.47
OS Compatibility                 39594972937  25.88     25.88
Container                        39594972858  12.48     12.48
Docs Lint                        39929995064   1.07      1.07
Database Compatibility           39253735217  14.33     14.33
Database Benchmarking            39253735180   5.37      5.37
Security Scan                    40648337098  40.40     40.40
Copilot Setup Steps              40229361313   2.73      2.73
```

Finished at 11:10 UTC.

#### Conclusion

All nine sums match. Met. This confirms the arithmetic on the sampled runs; it says nothing about
how representative six runs per workflow are (see the caveats in `post-change-evidence.md`).

## Failures and Follow-up

None. A real browser check of the PR timelines and the Actions UI was not performed (see the
deviation above).
