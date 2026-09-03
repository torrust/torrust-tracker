---
semantic-links:
  skill-links:
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - tests/lifecycle/native_tracker.rs
    - tests/lifecycle/signals.rs
    - tests/AGENTS.md
    - docs/issues/open/2132-add-sigterm-to-main/native-shutdown-test-plan.md
    - docs/issues/open/2132-add-sigterm-to-main/ISSUE.md
---

# Native Tracker Fixture Incremental Refactor Plan

## Purpose

Refactor `tests/lifecycle/native_tracker.rs` into a clearer, maintainable
executable-lifecycle test fixture without changing its externally observable
contract. The fixture currently provides correct process isolation, readiness,
signal delivery support, graceful shutdown, and failure cleanup, but its
responsibilities have accumulated in one type.

This plan is deliberately incremental. Every subtask must leave the test suite
working, be independently reviewable, and be suitable for its own focused
commit. Do not combine the steps into one large refactor.

## Current State

`NativeTracker` currently owns all of these concerns:

1. Temporary workspace and tracker configuration creation.
2. Child command construction and process spawning.
3. Concurrent stdout and stderr draining plus retained diagnostic output.
4. Startup-log parsing and health-check address discovery.
5. Health polling and executable-boundary signal-handler readiness checks.
6. Child-exit checks, retry timing, startup deadlines, and failure diagnostics.
7. Graceful shutdown, forced termination after timeout, reaping, and panic-path
   cleanup observation.

The principal maintainability concern is `NativeTracker::wait_until_ready()`.
It combines output discovery, HTTP requests, readiness semantics, child status,
deadline handling, retries, and error reporting in one nested loop. That makes
its control flow hard to read, makes changes risky, and obscures the distinct
readiness requirements that protect the signal tests from races.

## Refactor Goals

- Preserve the current black-box test contract exactly unless a separately
  reviewed behavior change is needed.
- Keep `NativeTracker` as the small test-facing interface for a running tracker
  child process.
- Give temporary-workspace/configuration and output capture coherent owners.
- Make readiness orchestration short enough to understand without tracing
  nested `match` expressions.
- Retain deterministic proof that `main()` installed the signal handlers before
  SIGTERM or SIGINT is sent.
- Preserve isolated child configuration, port-zero binding, exact-PID signal
  delivery, concurrent output draining, time-bounded shutdown, and reaping.
- Add tests only where an extracted collaborator introduces independently
  testable behavior. Do not add tests that merely mirror implementation details.

## Constraints and Invariants

The refactor must retain the following properties:

| Area                    | Required invariant                                                                                                                          |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Executable boundary     | Tests start the Cargo-built `torrust-tracker` binary, never `cargo run` or an in-process application.                                       |
| Configuration isolation | Each child receives `TORRUST_TRACKER_CONFIG_TOML_PATH` through `Command::env`; the test process environment is never mutated.               |
| Filesystem isolation    | Each tracker owns a `TempDir` that remains alive until the child is reaped.                                                                 |
| Network isolation       | Listener configuration uses loopback port `0`; the assigned health address is discovered after startup.                                     |
| Readiness               | A child is ready only after `/health_check` reports `Status::Ok` and the output contains `Tracker shutdown signal handlers installed.`      |
| Signal correctness      | Tests signal the retained child PID with typed `nix` signals. Normal scenarios never use `Child::kill()`.                                   |
| Output capture          | stdout and stderr are drained concurrently for the entire child lifetime, preventing pipe backpressure.                                     |
| Cleanup                 | Graceful shutdown is bounded; timeout or panic cleanup force-kills and reaps the child; the drop observer reports the reaped signal result. |
| Platform scope          | Unix-only process and signal behavior remains correctly gated.                                                                              |

## Target Shape

The target design has a limited number of responsibility-based collaborators:

- `NativeTracker`: public interface and child lifecycle owner. It starts the child,
  exposes `wait_until_ready`, returns the exact PID, performs explicit
  shutdown, and retains panic-path cleanup ownership.
- `NativeTrackerWorkspace` (or a similarly clear name): owns the temporary
  workspace, creates storage, writes the configuration, and supplies the
  configuration path used by the child command.
- `TrackerOutputCapture`: owns concurrent reader tasks and captured output. It
  deliberately remains passive: it drains streams, waits for readers, and
  returns retained text without interpreting tracker-specific log messages.
- `NativeTracker`: interprets tracker-specific startup facts from captured
  output and combines them with process lifecycle and readiness policy.
- `HealthCheckClient`: owns deadline-bounded interaction with the tracker
  health-check REST endpoint and classifies each probe outcome.
- A readiness collaborator is **not pre-approved as an automatic extraction**.
  A mandatory assessment step determines whether a small `ReadinessProbe` makes
  the remaining loop clearer than intent-level methods on `NativeTracker` and
  `TrackerOutput`. The assessment must record the decision and evidence before
  adding a new type.

`NativeTracker` must not become a generic process framework. The fixture serves
one executable and should keep test call sites simple:

```text
let mut tracker = NativeTracker::start();
tracker.wait_until_ready().await?;
send_signal(tracker.pid()?, Signal::SIGTERM)?;
let output = tracker.shutdown().await?;
```

## Deliberately Discarded Designs

The following are not part of this refactor. Reconsider them only when a new
concrete requirement provides evidence that the smaller design is insufficient.

### Generic external-process framework

**Discarded.** The repository currently has one native tracker child fixture.
Generalizing command construction, process management, output collection, and
signal policies now would create an abstraction with no demonstrated second
consumer.

### Signal-delivery hierarchy

**Discarded.** SIGTERM and SIGINT are small scenario-level differences. A type
hierarchy, strategy objects, or separate signal-delivery service would obscure
the direct typed `nix` calls without reducing meaningful complexity.

### Separate graceful-shutdown policy abstraction

**Discarded.** Shutdown deadlines and forced cleanup are fixture-owned behavior
with one implementation. Extract only if several executable fixtures need
materially different, reusable policies.

### Process-group or descendant-tree management

**Discarded for now.** The tracker does not intentionally spawn children. Exact
PID ownership is the narrow correct behavior. If later evidence finds managed
descendants, design and document a process-group policy separately.

### Replacing startup-log discovery with a new production interface

**Discarded from this refactor.** A machine-readable bound-address interface may
be valuable later, but it changes production contracts. This plan refactors the
existing fixture while preserving its explicit startup-log parsing contract.

### Changing startup or shutdown semantics

**Discarded.** This work improves fixture design only. Signal registration,
readiness markers, job shutdown order, deadlines, output messages, and tracker
exit behavior remain owned by their respective feature work.

## Incremental Implementation Plan

### Step 1: Establish the fixture contract before moving code

**Goal:** Make the behavior that must survive extraction explicit in the
existing suite.

**Changes:**

- Review `tests/lifecycle/signals.rs` and the fixture-local parser test against
  the invariants in this plan.
- Add narrowly scoped tests only for currently untested pure behavior that the
  first extraction needs, such as configuration rendering or output-address
  parsing edge cases.
- Do not alter fixture ownership or move production code in this step.

**Verification:**

1. Run `cargo test --test lifecycle-signals`.
2. Run the smallest applicable formatting and lint checks.
3. Confirm the existing SIGTERM, SIGINT, parser, and drop-cleanup assertions
   remain unchanged in meaning.

**Commit boundary:** One `test(lifecycle): characterize native tracker fixture`
commit, only if new characterization tests are actually needed. If the current
suite already expresses the extraction contract sufficiently, record that fact
in the implementation notes and make no empty commit.

### Step 2: Extract temporary workspace and configuration ownership

**Goal:** Remove filesystem/configuration construction from `NativeTracker`.

**Changes:**

- Introduce `NativeTrackerWorkspace` in `tests/lifecycle/native_tracker.rs`
  unless a small sibling module is demonstrably clearer.
- Move `TempDir` ownership, storage-directory creation, TOML rendering, and
  configuration-file writing into that collaborator.
- Expose only the configuration path needed to configure the child command.
- Keep the workspace alive through the `NativeTracker` lifecycle by storing the
  collaborator in the public interface.
- Keep child-only `Command::env` behavior in `NativeTracker::start`, because
  the child command is still owned there.

**Verification:**

1. Add or retain a focused test for configuration creation if the collaborator
   exposes testable filesystem behavior.
2. Run `cargo test --test lifecycle-signals`.
3. Confirm a parallel-safe port-zero configuration and child-specific config
   path are unchanged.

**Commit boundary:** `refactor(lifecycle): extract tracker workspace fixture`.

### Step 3: Extract output capture and startup facts

**Goal:** Give pipe draining, retained output, and startup-log queries one
coherent owner.

**Changes:**

- Introduce `TrackerOutputCapture` or `TrackerOutput`; choose the name that
  makes its retention and reader-task ownership clear.
- Move the shared output buffer, reader task handles, `drain_output`, reader
  completion, and retained-output access into the collaborator.
- Keep health-check address discovery and the signal-handler marker lookup on
  `NativeTracker`. They interpret tracker-specific output and therefore do not
  belong to the passive capture collaborator.
- Preserve concurrent stdout/stderr draining immediately after spawning.
- Preserve the existing policy that assertions require message presence, not
  cross-stream ordering.
- Keep parser tests next to the parsing behavior and extend them only for
  meaningful malformed/unrelated log cases.

**Verification:**

1. Run unit tests in the fixture module through `cargo test --test lifecycle-signals`.
2. Run the complete lifecycle target and confirm output-based SIGTERM/SIGINT
   assertions still pass.
3. Intentionally review failure messages to ensure they still include retained
   child output after reader completion.

**Commit boundary:** `refactor(lifecycle): extract tracker output capture`.

### Step 4: Simplify readiness orchestration without adding a new type

**Goal:** Make `NativeTracker::wait_until_ready()` a short deadline/retry
orchestrator using intent-level collaborator methods.

**Changes:**

- Refactor the readiness loop into small private operations with names that
  reveal the condition being evaluated, for example health address discovery,
  health report retrieval, readiness satisfaction, child-exit detection, and
  deadline failure construction.
- Reduce nested `match` structures where straightforward early returns or
  dedicated result helpers describe the paths more clearly.
- Retain the precise readiness definition: successful HTTP response,
  deserializable `Report`, `Status::Ok`, and installed signal-handler marker.
- Retain early child-exit diagnostics, retry interval, startup deadline, and
  output-rich error messages.

**Verification:**

1. Run `cargo test --test lifecycle-signals`.
2. Run `cargo clippy --test lifecycle-signals -- -W clippy::cognitive_complexity -D warnings` when workspace-wide unrelated diagnostics permit it; otherwise record the fixture-specific result and the external blocker.
3. Inspect the resulting method: its main loop should read as readiness polling,
   not as log, HTTP, process, and diagnostic implementation details interleaved.

**Commit boundary:** `refactor(lifecycle): simplify tracker readiness polling`.

### Step 5: Mandatory readiness-collaborator assessment

**Goal:** Decide deliberately whether a `ReadinessProbe` is justified after the
simpler collaborator boundaries are in place.

This is a required implementation step, not an optional future idea. Its
outcome may be either to add the collaborator or to document why the reduced
existing design is clearer without it.

**Assessment questions:**

1. Does `wait_until_ready()` still mix more than one stable responsibility after
   Steps 2 through 4?
2. Would a probe own coherent dependencies (output capture, HTTP client, and
   health readiness rules) without needing child lifecycle ownership?
3. Does the probe reduce the public interface's complexity and make readiness
   semantics easier to test or explain?
4. Is there a near-term second readiness consumer that validates the
   abstraction, or is the type merely moving a single method elsewhere?

**Decision rule:**

- Extract a small `ReadinessProbe` only when the answers show a coherent
  boundary and a measurable readability improvement.
- Do not extract it when `NativeTracker::wait_until_ready()` is already a clear
  bounded loop over well-named operations. Record the no-extraction rationale
  in the implementation commit or review notes.

**If extraction is justified:**

- Make the probe own only readiness facts and HTTP polling.
- Keep child state, shutdown, exact PID, and drop cleanup on `NativeTracker`.
- Add focused tests for the probe's pure or controllable behavior where useful.

**Verification:**

1. Run `cargo test --test lifecycle-signals`.
2. Recheck that readiness still proves signal-handler installation before tests
   send a signal.
3. Review the diff to ensure no generic process abstraction has appeared.

**Commit boundary:** Either `refactor(lifecycle): extract tracker readiness probe`
or a small documentation/review-note update that records the evidence-based
no-extraction decision. Do not make a cosmetic commit solely to satisfy this
step.

#### Step 5 Assessment Decision

**Decision: do not extract `ReadinessProbe`.** The completed
`NativeTracker::wait_until_ready()` is a 14-line bounded polling orchestrator
with complexity 6 and nesting 2. It now owns only the inseparable lifecycle
concerns of retrying readiness, detecting an early child exit, enforcing the
shared startup deadline, and applying the retry interval.

The assessment reached this decision for these reasons:

1. The remaining readiness operation does not mix multiple stable
   responsibilities. `HealthCheckClient` owns deadline-bounded health API
   requests, response decoding, and probe outcome classification;
   `TrackerOutputCapture` owns retained child output.
2. A `ReadinessProbe` would only partially own coherent dependencies. It would
   need the captured output for endpoint discovery and the signal-handler
   marker, while `NativeTracker` would still own child lifecycle, timeout
   diagnostics, and retry policy.
3. A new probe would not reduce the fixture's public interface or make its
   readiness rule clearer. The current rule remains explicit: discover the
   health endpoint, receive a successful and deserializable `Report` with
   `Status::Ok`, and observe the installed-signal-handlers marker.
4. There is no independent second consumer. The SIGTERM and SIGINT scenarios
   both use `NativeTracker::wait_until_ready()`, while drop cleanup does not
   require readiness.

The existing `HealthCheckClient` is the appropriate API-boundary collaborator.
Adding `ReadinessProbe` now would only distribute a small cohesive lifecycle
operation across another private type. Reassess this decision if a future
executable fixture needs readiness independently from `NativeTracker`.

### Step 6: Final cleanup and independent review

**Goal:** Confirm the completed small refactors retain a narrow, readable
fixture and do not leave incidental duplication or stale documentation.

**Changes:**

- Apply only cleanup directly supported by the previous steps: names, Rust docs
  for non-obvious ownership or cleanup invariants, import ordering, and local
  duplication removal.
- Update `native-shutdown-test-plan.md` only if it describes a fixture structure
  that changed materially.
- Do not merge deferred designs into this cleanup step.

**Verification:**

1. Run `cargo fmt --check`.
2. Run `cargo test --test lifecycle-signals`.
3. Run the applicable root test and lint gates required by the current branch.
4. Independently review the changed fixture against this plan's invariants and
   rejected designs.

**Commit boundary:** `refactor(lifecycle): clarify native tracker fixture`, only
if a focused cleanup remains after prior commits. Otherwise no additional commit
is necessary.

## Commit and Recovery Strategy

- Perform steps strictly in order. Do not start a later extraction while the
  current step has unreviewed failures.
- Commit only one completed, validated responsibility change at a time.
- If a step changes behavior unexpectedly, revert only that step's working
  changes or commit; do not attempt to repair it by layering the next planned
  extraction on top.
- Preserve existing scenario tests throughout. A failing signal scenario is a
  blocker, not an invitation to weaken readiness, cleanup, or output assertions.
- Keep production `src/main.rs` unchanged unless a separate issue establishes a
  needed executable contract change.

## Completion Criteria

- [x] `NativeTracker` is a concise interface for tracker process lifecycle.
- [x] Temporary workspace/configuration and output capture have coherent,
      separately named owners.
- [x] `wait_until_ready()` clearly expresses bounded readiness orchestration and
      no longer contains the current dense mixture of concerns.
- [x] The mandatory `ReadinessProbe` assessment has a recorded,
      evidence-based extract-or-do-not-extract decision.
- [x] The fixture retains all process, readiness, isolation, and cleanup
      invariants listed in this plan.
- [x] `cargo test --test lifecycle-signals` passes after every committed step.
- [x] Formatting, relevant linting, and required branch quality gates pass
      before the final commit.
- [x] No generic process framework, signal hierarchy, process-group policy, or
      production startup-contract change was introduced without a separate
      approved need.

## Post-Completion Improvement Proposals

The refactor above is complete. The following proposals were identified during
the final review and are intentionally not part of its completion criteria.
They are small, independently verifiable follow-ups. Implement only a proposal
that remains valuable when it is reviewed again; do not reopen the completed
refactor merely to add speculative abstraction.

### Proposal 1: Characterize rejected health-check startup logs

**Why it may be worthwhile:** `parse_health_check_address` is the intentional
source of the ephemeral health-check address. The existing positive test proves
the expected startup line is accepted, but does not characterize rejected input.
Small negative cases guard against future false positives that could make the
readiness loop probe an unintended address.

**Small scope:** Add table-driven fixture-local tests for these inputs:

1. A line from an unrelated log target.
2. A health-check log line without the `Started on: http://` prefix.
3. A health-check startup line with a malformed or non-socket address.

Do not introduce a log-parsing service or move tracker-specific parsing into
`TrackerOutputCapture`; it deliberately remains a passive output component.

**Verification:** Run `cargo test --test lifecycle-signals` and the applicable
formatting/lint checks.

**Independent commit boundary:**
`test(lifecycle): cover malformed health startup logs`.

### Proposal 2: Align native shutdown-plan output-capture wording

**Why it may be worthwhile:** `native-shutdown-test-plan.md` describes retaining
each output stream and concatenating after exit, while the fixture deliberately
drains stdout and stderr concurrently into one retained buffer. The actual
contract is message presence, not stream identity or cross-stream ordering.

**Small scope:** Update only the output-handling wording in
`native-shutdown-test-plan.md` to describe the shared retained output buffer and
the no-cross-stream-order assertion policy. Do not change output capture code,
split the streams, or add stream-order assertions.

**Verification:** Run `linter markdown` and `linter cspell`.

**Independent commit boundary:**
`docs(lifecycle): clarify native tracker output capture`.

### Improvements Explicitly Rejected for Now

- Further log-parsing extraction or generic log-query APIs: one small
  tracker-specific parser does not justify a framework.
- A `ReadinessProbe`: the mandatory assessment already found no coherent
  independent boundary or second consumer.
- A richer health-probe state model or typed fixture-error hierarchy: the
  current outcomes and output-rich `String` diagnostics remain adequate for one
  fixture.
- Removing `Option` ownership from child, workspace, or output fields:
  `Option::take` is required to move those values into exclusive explicit or
  drop-path cleanup.
- Separate stdout/stderr buffers, process-group policy, broader fixture API,
  or additional internal scheduling tests: none improve the asserted
  executable-boundary contract enough to justify their complexity.

## References

- [Native executable shutdown test plan](native-shutdown-test-plan.md)
- [Issue specification](ISSUE.md)
- [Manual verification evidence](verification.md)
- [Integration-test guidelines](../../../../tests/AGENTS.md)
- [Native tracker fixture](../../../../tests/lifecycle/native_tracker.rs)
- [Lifecycle signal scenarios](../../../../tests/lifecycle/signals.rs)
- [Shutdown EPIC](../1488-overhaul-tracker-shutdown/ISSUE.md)
