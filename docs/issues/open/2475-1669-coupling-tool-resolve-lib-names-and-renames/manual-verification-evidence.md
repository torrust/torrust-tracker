---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2475-1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md
last-updated-utc: 2026-10-07 16:39
---

<!-- cspell:ignore splitlines startswith -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

The commands and the relevant output are recorded inline below. The generated report under
`/tmp/` is scratch output; the excerpts that matter are copied here.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-07 08:48
- Artifact under test: `contrib/dev-tools/analysis/workspace-coupling/` on branch
  `2446-1669-establish-baseline-analysis` (PR #2462), before any fix
- Operating system / environment: Linux; stable Rust toolchain (`rustc 1.99.0 (b940084d7 2026-09-28)`)
- Prerequisites and setup performed: none beyond a workspace checkout

## Verification Processes

### R1 - Initial reproduction

- Goal: observe the tool reporting "no references" for dependency edges whose crate is used.
- Initial state: unmodified tool.
- Status: `DONE` — outcome: **Reproduced**

#### Steps Performed

1. Run the real tool on the workspace (stable Rust toolchain):

   ```sh
   cargo run -q -p workspace-coupling -- /tmp/repro-2446.md
   ```

2. List every edge the report marks as having no references:

   ```python
   import re
   txt = open("/tmp/repro-2446.md").read()
   pkg = dep = None
   for line in txt.splitlines():
       m = re.match(r"^### `(.+)`", line)
       if m: pkg = m.group(1); continue
       m = re.match(r"^#### `(.+)` \[(.+)\]", line)
       if m: dep = m.group(1); continue
       if "No `" in line and "references found" in line:
           print(f"{pkg} -> {dep}: {line.strip()}")
   ```

3. Show that each dependency is used in source, and how its module name differs:

   ```sh
   grep -rn "^use torrust_tracker_client::" console/tracker-client/src | head -1
   grep -rn "torrust_tracker_client::" packages/udp-server/src | head -1
   grep -rn "^use torrust_tracker_client::" packages/test-helpers/src | head -1
   grep -rln "torrust_tracker_client::" packages/axum-http-server/tests | head -1
   grep -rn "torrust_tracker_lib::" packages/e2e-tools/src | head -1
   grep -n "torrust-tracker-client" console/tracker-client/Cargo.toml packages/udp-server/Cargo.toml packages/test-helpers/Cargo.toml packages/axum-http-server/Cargo.toml
   grep -n -A1 "^\[lib\]" packages/tracker-client/Cargo.toml Cargo.toml
   ```

#### Observed Result

Step 1 exited with code 0. Step 2:

```text
torrust-tracker-axum-http-server -> torrust-tracker-client-lib: _No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._
torrust-tracker-client -> torrust-tracker-client-lib: _No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._
torrust-tracker-e2e-tools -> torrust-tracker: _No `torrust_tracker::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._
torrust-tracker-test-helpers -> torrust-tracker-client-lib: _No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._
torrust-tracker-udp-server -> torrust-tracker-client-lib: _No `torrust_tracker_client_lib::` references found in source — may be used only in `Cargo.toml` feature flags or `build.rs`._
```

Step 3:

```text
console/tracker-client/src/console/clients/udp/mod.rs:5:use torrust_tracker_client::udp;
packages/udp-server/src/server/launcher.rs:16:use torrust_tracker_client::udp::client::check;
packages/test-helpers/src/http.rs:5:use torrust_tracker_client::http::client::Client;
packages/axum-http-server/tests/server/v1/contract/using_ipv6_v6only.rs
packages/e2e-tools/src/bin/e2e_tests_runner.rs:2:use torrust_tracker_lib::console::ci::e2e;
console/tracker-client/Cargo.toml:29:torrust-tracker-client = { package = "torrust-tracker-client-lib", version = "0.1.0", path = "../../packages/tracker-client" }
packages/udp-server/Cargo.toml:23:torrust-tracker-client = { package = "torrust-tracker-client-lib", version = "0.1.0", path = "../tracker-client" }
packages/test-helpers/Cargo.toml:22:torrust-tracker-client-lib = { version = "0.1.0", path = "../tracker-client" }
packages/axum-http-server/Cargo.toml:54:torrust-tracker-client-lib = { version = "0.1.0", path = "../tracker-client" }
packages/tracker-client/Cargo.toml:20:[lib]
packages/tracker-client/Cargo.toml-21-name = "torrust_tracker_client"
Cargo.toml:18:[lib]
Cargo.toml-19-name = "torrust_tracker_lib"
```

#### Conclusion

**Reproduced.** Five edges are reported with no references although each dependency is used:
two through a `Cargo.toml` rename (`torrust-tracker-client`), two through the custom library
name `torrust_tracker_client`, and one through the root library name `torrust_tracker_lib`.
The tool searched for `torrust_tracker_client_lib::` and `torrust_tracker::`, which never
appear. The `axum-http-server` edge is a dev dependency, so the 2026-10-06 report's finding
(which counted only normal edges) listed four.

### R2 - Final recheck

- Goal: after the fix, the same command lists no false "no references" edge.
- Status: `TODO`

## Failures and Follow-up

None yet.
