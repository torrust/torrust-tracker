# Implementation Retrospective — Issue #2289 HTTP Tracker Token Lifecycle

## Outcome

The HTTP tracker now receives a child cancellation token, owns its server and
Axum drain-controller tasks, and reports a named component outcome to
`JobManager`. The legacy `HttpServer::start` and `HttpServer::stop` API remains
available for consumers that have not yet migrated.

Focused tests, complete touched-package tests, `linter all`, and pre-commit
passed on nightly Rust 1.100.0. Direct tracker-binary SIGTERM verification
confirmed the token-aware HTTP drain, clean process exit, and listener rebind.

## What Went Well

1. The additive server API kept the migration limited to the HTTP tracker while
   preserving legacy consumers.
2. Direct-PID verification exposed the production cancellation path and
   confirmed that the executable boundary, not a package, owns SIGTERM.

## What Changed During Implementation

The initial independent-runtime terminal path relied on `Drop` to abort a
waiting controller. Independent review identified that aborting is insufficient
for a supervised component because it does not join the child. The component now
cancels and joins the controller before reporting independent completion or
runtime failure.

The associated tests originally proved only that a controller observed
cancellation. They now hold controller completion behind a test-controlled
release and assert the supervisor remains pending until the controller finishes.
This proves joined ownership rather than cancellation alone.

## Root Cause

The initial test design conflated controller cancellation with controller
completion. The lifecycle plan required both child tasks to be awaited, but the
unexpected-runtime tests did not model a controller that could remain pending
after cancellation.

## Improvements for Future Work

1. For every supervised-child lifecycle branch, make tests distinguish
   cancellation requested, child completion, and parent outcome.
2. Run the first independent ownership review before marking acceptance
   evidence complete, especially where a `Drop` implementation can conceal an
   unjoined task.

## Avoiding Overcorrection

No shared lifecycle abstraction is justified by this one migration. The local
HTTP supervision helper remains small and directly expresses the component's
three terminal paths.

## Evidence

- [Issue specification](ISSUE.md)
- [Verification evidence](verification.md)
- [Manual verification evidence](manual-verification-evidence.md)
- `cargo test -p torrust-tracker-axum-http-server`
- `cargo test -p torrust-tracker`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
