---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1860
spec-path: docs/issues/open/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/tsl.rs
    - packages/configuration/src/v2_0_0/tls.rs
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1860 — Evaluate moving `TslConfig` from `torrust-tracker-configuration` into `torrust-tracker-axum-server`

## Goal

Decide whether `TslConfig` should be moved out of `torrust-tracker-configuration`
into `torrust-tracker-axum-server`, where it is its only production consumer.

Record a decision entry in `DECISIONS.md`. Implement the chosen approach if it is
beneficial.

This is **FU-2** from the analysis in issue
[#1856](https://github.com/torrust/torrust-tracker/issues/1856) (DEC-07).

This issue is a subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md).

## Background

`TslConfig` is currently defined in `torrust-tracker-configuration`
(`packages/configuration/src/v2_0_0/tls.rs`). Its only production consumer is
`torrust-tracker-axum-server` (`packages/axum-server/src/tsl.rs`). No other production
code in the workspace depends on `TslConfig` directly.

This makes `torrust-tracker-axum-server` depend on the full configuration package for a
two-field struct (`ssl_certificate_file_path` and `ssl_private_key_file_path`) that has
no relationship to the config file schema or TOML deserialization.

The EPIC.md already flags this as a temporary coupling:

> `TslConfig` remains the temporary tracker-specific dependency: it is a small two-field
> struct with no tracker-specific logic and could be moved to a generic package. Once that
> change lands, the package could move to the `torrust-` group as a generic
> `torrust-axum-server` reusable across the Torrust organisation.

### Options

| Option | Description                                                            | Benefit                                                                    |
| ------ | ---------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| A      | Move `TslConfig` to `torrust-tracker-axum-server`                      | Removes `axum-server`'s config dependency                                  |
| B      | Move `TslConfig` to a new generic location (e.g. `torrust-server-lib`) | Enables `axum-server` → `torrust-axum-server` extraction to org-level repo |
| C      | Keep as-is                                                             | Document why the gain is too small to act on                               |

## Proposed Analysis Steps

### Step 1 — Audit `TslConfig` usage

Confirm all usages of `TslConfig` across the workspace:

```bash
grep -rn "TslConfig" packages/ src/ --include="*.rs"
```

Verify that `torrust-tracker-axum-server` is the only non-test, non-config consumer.

### Step 2 — Evaluate dependency direction impact

- If Option A: check whether `torrust-tracker-configuration` deserializes TLS config from
  `[tls]` in `tracker.toml`. If yes, a re-export or mapping step is needed so deserialization
  still constructs the moved type.
- If Option B: identify whether `torrust-server-lib` is the right home or whether a dedicated
  `torrust-axum-tls` micro-package is warranted.

### Step 3 — Record decision

Add a decision entry (e.g. DEC-08) to `DECISIONS.md`.

### Step 4 — Implement (if Option A or B chosen)

Move the type, update import sites, update Cargo manifests, run tests.

## Acceptance Criteria

- [ ] A decision entry is added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
      with chosen approach and rationale
- [ ] If Option A or B chosen: `TslConfig` is moved, all import sites updated, and
      `torrust-tracker-axum-server`'s `Cargo.toml` no longer depends on
      `torrust-tracker-configuration`
- [ ] All tests pass; no new clippy warnings

## Out of Scope

- Extracting `torrust-tracker-axum-server` to a standalone repo (tracked separately in EPIC)
- Moving `TrackerPolicy` or `PrivateMode` (FU-1, #1859)
- Changing `EnvContainer::initialize` (FU-3, #1861)

## Layer Impact

Option A removes the edge `axum-server → configuration`. This does not introduce any
forbidden dependency edges per the EPIC layer guardrails. It makes `axum-server` a
pure framework-integration layer with no domain-level config coupling.

## Related

- Parent EPIC: #1669 — [EPIC.md](../1669-overhaul-packages/EPIC.md)
- Decision to be added: DECISIONS.md DEC-08
- Analysis: #1856 — [ISSUE.md](../1856-1669-analyse-configuration-package-coupling/ISSUE.md)
- EPIC note: see "Note on `torrust-tracker-axum-server`" in EPIC.md
- Follow-ups: FU-1 (#1859), FU-3 (#1861)
