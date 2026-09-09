# Implementation Retrospective

## Material Findings

Rust's native `reason = "..."` lint-attribute parameter is the policy mechanism. The focused
Rust validator enforces it prospectively by parsing only attributes whose spans overlap lines
changed from the Git merge base. This preserves the #2158 historical-remediation boundary.

The tool is intentionally a small crate with pure `syn` validation and a narrow Git command-line
adapter. It is reusable by a future #2003 harness but does not define that harness's architecture,
commands, or output contract.

`clippy::allow_attributes_without_reason` remains the intended compiler-aware end state. Enabling
it workspace-wide is deferred because it would immediately fail on historical allows, which #2158
must inventory and remediate.
