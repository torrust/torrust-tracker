---
doc-type: issue
issue-type: task
status: superseded
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1488-si-7-observable-shutdown-progress/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-01
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - src/bootstrap/jobs/manager.rs
    - src/main.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/analysis/20260716-shutdown-process/README.md
---

<!-- skill-link: create-issue -->

# Superseded Draft SI-7 — Fold Outcome Reporting into Issue #1586

> **Status**: Superseded for implementation planning. Its structured-outcome
> requirement is part of issue [#1586](../../open/1586-evaluate-job-manager-join-set/ISSUE.md);
> optional periodic progress is a later additive presentation task after
> operational feedback.

## Why This Draft Is Superseded

Structured outcomes and concurrent waiting share one data model and must land
together. Splitting them would produce either unstructured waiting or output
that cannot faithfully represent component state. Issue #1586 now owns
completed, failed, timed-out, and deliberately aborted named outcomes. Do not
implement this draft separately.

## Original Goal

During shutdown, report the named outcome of every top-level component:
completed, failed, timed out, or deliberately aborted. Optional periodic
"still waiting" output must be derived from the same concurrent-supervision
state, not from a separate unowned logging task.

## Background

`src/bootstrap/jobs/manager.rs` currently logs:

```text
INFO  Waiting for job to finish (timeout of 10 seconds) ... job=http_tracker_0
WARN  Job did not complete in time job=http_tracker_0
```

There is no visibility into what is happening while waiting — no active connection
count, no periodic "still waiting" message, no final summary. This makes it hard
to diagnose shutdown hangs in production or CI.

## Desired Output

```text
INFO  Torrust tracker shutting down (SIGTERM) ...
INFO  Waiting for 9 jobs to finish (timeout: 30s) ...
INFO  Waiting for jobs: http_tracker_0, http_tracker_1, udp_tracker_0 ... (6 others done)
INFO  All jobs finished. Shutdown complete.
```

Or when a job times out:

```text
WARN  Shutdown timeout reached. Jobs still running: http_tracker_0 (3 active connections)
INFO  Exiting.
```

## Implementation

Options:

1. **Periodic log loop** — spawn a task during `wait_for_all` that logs
   remaining job names every N seconds until all are done.
2. **Final summary** — after `join_all` completes, log which jobs finished vs
   timed out.
3. **Both** — periodic progress + final summary.

The minimal viable implementation is option 2 (final summary). Option 1 requires
issue #1586's concurrent waiting to be useful.

## Acceptance Criteria

- [ ] A final summary lists every top-level component and its outcome:
      completed, failed, timed out, or deliberately aborted.
- [ ] If any job times out, the log message includes the job name.
- [ ] The shutdown start message logs the total number of jobs and the timeout.
- [ ] `linter all` passes.

## Dependencies

- Depends on issue #1586's structured concurrent outcome collection.
- Q3 consumes these aggregate outcomes to define the process exit result.

## Manual Verification

Evidence of these steps must be recorded in `verification.md` in this folder
before the issue can be closed.

### Test 1: Shutdown start message includes job count and timeout

Start the tracker and send Ctrl+C. Confirm the first shutdown log line includes
the number of jobs and the grace period:

**Expected** (exact wording may differ):

```text
INFO  Torrust tracker shutting down ...
INFO  Waiting for 9 jobs to finish (timeout: 30s) ...
```

**Record in `verification.md`**: the exact log output.

### Test 2: Final summary lists all jobs

After all jobs complete, confirm a summary is logged:

**Expected** (clean shutdown):

```text
INFO  All jobs finished. Shutdown complete.
```

Or with timed-out jobs:

```text
WARN  Job did not complete in time job=http_instance_0_0.0.0.0:7070
INFO  Shutdown complete (1 job timed out).
```

### Test 3: Timed-out job is named

To trigger a timeout, artificially hold an HTTP connection open during shutdown
(while using a short grace period). Confirm the timeout warning names the job.

**Expected**:

```text
WARN  Job did not complete in time job=http_instance_0_0.0.0.0:7070
```

**Not acceptable**:

```text
WARN  Job did not complete in time
```

(job name missing)

**Record in `verification.md`**: the warning log line including the job name.
