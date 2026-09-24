# Implementation Retrospective

## Owner Created Before the Component Is Returned

Independent review found that the first version wrapped the server and
drain-controller handles in `TokenAwareServerTask` inside the returned
component future. If `JobManager` dropped or aborted that future before its
first poll, the raw `JoinHandle`s were dropped, which detaches Tokio tasks: the
server kept running and the listener stayed bound. SI-12 did not have this gap
because it builds the owner eagerly.

The owner is now created before `start_job` returns. A regression test drops
the unpolled component and waits, under a 5-second bound, for the listener to
become bindable. Moving the owner back into the future made that test fail.

Lesson: when a starter returns a future that will own spawned tasks, move the
handles into their drop-safe owner before returning, not inside the future.

## Cancelled-Path Join Order

On the cancellation path, a server join error was mapped with `?` before the
drain controller was joined, so a panicking server would let the owner's drop
abort the controller without awaiting it. The health-check supervisor now joins
both children before mapping errors. The SI-11 and SI-12 supervisors keep the
original order; consolidating the three supervisor copies (deferred to the
legacy-removal phase) should adopt the join-both-first order.

## Test Design Review

Package tests own registration, token drain, and registration rollback.
Component tests own child-join and outcome semantics, and the drop-before-run
listener release. The application test owns `JobManager` token propagation.
Each test keeps its causal state, production action, and observable result
visible.

## Follow-ups

- `docs/features/shutdown-process/task-inventory.md` still describes the HTTP
  tracker and REST API with the pre-SI-11/SI-12 `NestedServerTask`/`Halted`
  bridge and detached drain controllers. Only the health-check rows were
  updated here.
- The #2234 and #2274 specifications remain in `docs/issues/open/` although
  both issues are closed; archive them in a separate chore.
