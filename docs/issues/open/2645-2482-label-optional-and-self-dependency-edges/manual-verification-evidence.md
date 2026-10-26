---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2645-2482-label-optional-and-self-dependency-edges/ISSUE.md
last-updated-utc: "2026-10-26 07:17"
---

# Manual Verification Evidence

## Purpose

Record the commands run to verify scenarios M1 and M2 of the [spec](ISSUE.md): the coupling tool
labels optional edges and skips self edges on a scratch workspace, and still reads this repository.
Every output below is the real output of the command shown; the repository path is replaced by
`<repo>`.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-25, 08:09 to 08:10.
- Artifact under test: the implementation branch of this issue after the commit whose subject is
  `feat(workspace-coupling): [#2645] label optional edges and skip self edges`; the baseline is the
  tool built from the tip of `develop` after the merge of PR #2646.
- Operating system / environment: Linux developer workstation; `cargo`/`rustc` 1.99 stable.
- Prerequisites and setup performed: the scratch workspace of V1 under `/tmp/2645-mv`, and a second
  checkout of `develop` (`git worktree add --detach /tmp/2645-base torrust/develop`, built with
  `CARGO_TARGET_DIR=/tmp/2645-base-target cargo build -p workspace-coupling`) for the baseline
  binary. Both are removed after the run.
- Each run wrote its report to a file in `/tmp`; nothing under `docs/` was regenerated.

## Verification Processes

### V1 - The tool labels a scratch workspace (M1)

- Goal: show that the tool labels an optional dependency enabled by `testing` as
  `optional (testing)`, another optional dependency with its feature, keeps `normal` and `dev`, and
  does not list a self dev-dependency.
- Initial state: a scratch workspace with members `a`, `b`, `c`, `d` and `o1c`:

  ```toml
  # a/Cargo.toml (dependencies and features)
  [dependencies]
  c = { path = "../c", optional = true }
  d = { path = "../d", optional = true }
  [features]
  testing = ["dep:c"]
  extras = ["dep:d"]

  # b/Cargo.toml
  [dependencies]
  c = { path = "../c" }
  [dev-dependencies]
  a = { path = "../a", features = ["testing"] }

  # o1c/Cargo.toml
  [features]
  testing = []
  [dev-dependencies]
  o1c = { path = ".", features = ["testing"] }
  ```

  `c`, `d` have no dependencies. The sources use `c::value` (in `a` and `b`) so the scan finds an
  import.
- Status: `DONE`

#### Steps Performed

1. At 08:10:01 UTC, in `/tmp/2645-mv`: the baseline binary, then (08:10:02) the new binary, each
   with a report path under `/tmp`.
2. `cargo metadata --format-version 1 --no-deps`, reduced to one line per dependency, to show the
   input the tool reads.
3. The headings and "Workspace deps" lines of each report.

#### Observed Result

The metadata the tool reads (dependency, kind, optional, and the enabling features table or the
dependency's own `features`):

```text
a -> c kind= None optional= True features= {'extras': ['dep:d'], 'testing': ['dep:c']}
a -> d kind= None optional= True features= {'extras': ['dep:d'], 'testing': ['dep:c']}
b -> c kind= None optional= False features= []
b -> a kind= dev optional= False features= ['testing']
o1c -> o1c kind= dev optional= False features= ['testing']
```

Baseline tool (`develop`): the optional edges show as `normal` and the self edge is listed.

```text
Packages with no workspace dependencies: `c`, `d`
### `a`    Workspace deps: 2    #### `c` [normal]   #### `d` [normal]
### `b`    Workspace deps: 2    #### `c` [normal]   #### `a` [dev]
### `o1c`  Workspace deps: 1    #### `o1c` [dev]
```

New tool:

```text
Packages with no workspace dependencies: `c`, `d`, `o1c`
### `a`    Workspace deps: 2    #### `d` [optional (extras)]   #### `c` [optional (testing)]
### `b`    Workspace deps: 2    #### `c` [normal]              #### `a` [dev]
```

(the headings are shown on one line per package here; the reports put each on its own line, and
`c::value` is listed under `a` > `c` and `b` > `c` in both).

#### Conclusion

`a` -> `c` is labelled `optional (testing)` and `a` -> `d` `optional (extras)`; `b` -> `c` stays
`normal` and `b` -> `a` stays `dev`. `o1c` has no entry and is a leaf, so the self edge is skipped
in the details, the counts and the leaf detection. M1 holds (AC1, AC2).

#### Second run: the implicit feature syntax

The first run used `testing = ["dep:c"]`. Cargo also accepts `testing = ["y"]` for an optional
dependency `y`, and then adds the implicit feature `y = ["dep:y"]` to the features table. This holds
at edition 2024, the edition of every workspace member: the scratch workspace below declared
`edition = "2024"` and `cargo 1.99.1` (`rustc 1.99.1`) accepted the manifest and printed the
implicit feature (the edition is also in the `cargo metadata` output; V4 shows it per package).
A reviewer expected edition 2024 to create no implicit feature; this run shows it still does, so the
implicit-feature handling is not limited to older editions.

```text
$ cargo metadata --no-deps --format-version 1   # scratch workspace of x (optional y, testing = ["y"]) and y
x {'testing': ['y'], 'y': ['dep:y']}
y {}
```

The first implementation counted that implicit feature as an enabling feature and labelled the edge
`optional (testing, y)`; the unit test written from a hand-made fixture had missed it. The fix is
the commit `fix(workspace-coupling): [#2645] ignore the implicit feature of an optional dependency`,
tested by two new tests with the real shape of the table. At 08:12:15 UTC the new tool printed, in
that scratch workspace:

```text
### `x`
Workspace deps: 1
#### `y` [optional (testing)]
```

#### Third run: a default feature that enables `testing`

Review finding F1 of PR #2648 (Copilot): a dependency a default build enables is not test-only. In
a scratch workspace with `default = ["testing"]` and `testing = ["y"]` the metadata is:

```text
x {'default': ['testing'], 'testing': ['y'], 'y': ['dep:y']}
y {}
```

The tool, which up to that point followed no feature references, printed `optional (testing)`. The
fix is the commit `fix(workspace-coupling): [#2645] follow feature references when labelling an optional edge`.
At 08:34:49 UTC the new tool printed:

```text
### `x`
Workspace deps: 1
#### `y` [optional (default, testing)]
```

### V2 - The tool still reads this workspace (M2)

- Goal: show that the tool still reads this repository and that nothing but the legend changes,
  because no workspace optional or self edge exists yet.
- Initial state: this branch, from the root of the workspace.
- Status: `DONE`

#### Steps Performed

1. `<baseline binary> /tmp/2645-repo-old.md` and `<new binary> /tmp/2645-repo-new.md` from the
   repository root, then `diff` of the two reports.

#### Observed Result

```text
$ diff /tmp/2645-repo-old.md /tmp/2645-repo-new.md
14a15,16
> - **Optional dep** (`optional (FEATURES)`) — a normal dependency that the listed features enable, directly or through other features.
>   `optional (testing)` is a test-only edge, not a runtime edge; `optional` means no feature enables it.
16a19
> - A package's own dev-dependency on itself (used to enable its `testing` feature) is not listed.
```

Both reports list 32 workspace packages with identical package sections.

#### Conclusion

The only difference is the three added legend lines. No edge, count or leaf changed, so M2 holds.

### V3 - Automatic checks (AC4)

- Goal: the tool's own tests pass.
- Status: `DONE`

`cargo test -p workspace-coupling`: `tests/edge_labels.rs` 20 passed (14 failed to compile before
the change: `workspace_dependencies` and `edge_kind` did not exist; the two implicit-feature tests
failed before the first fix and four of the five transitive-feature tests before the second),
`tests/parse_imports.rs` 9 passed, `tests/resolve_module_name.rs` 11 passed.
The pre-commit gate (including `cargo machete --with-metadata`, `cargo deny check bans` and
`linter all`) passed on each commit, and the pre-push script (`pre-push.sh`, 6 steps) passed.

## Summary

| ID | Scenario | Status | Result |
| --- | --- | --- | --- |
| V1 | Scratch workspace labels (M1) | DONE | Labels and self-edge skip as specified |
| V2 | This repository (M2) | DONE | Only the legend differs |
| V3 | Tool tests | DONE | 40 tests pass |

Validated by the AI assistant that ran the commands; the maintainer reviews it in the pull request.
