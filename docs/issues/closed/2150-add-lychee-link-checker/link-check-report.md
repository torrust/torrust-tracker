# Lychee Local Link-Check Report

## Tooling

- Tool: [lychee](https://lychee.cli.rs/)
- Version: `0.24.2`
- Installation: already available locally at `~/.cargo/bin/lychee`; install with `cargo install lychee --version 0.24.2`
- Configuration: [`lychee.toml`](../../../../lychee.toml)

## Checked Markdown Set

```text
README.md SECURITY.md 'docs/**/*.md' '**/AGENTS.md' 'packages/*/README.md' 'console/**/*.md' 'contrib/**/*.md' 'share/**/*.md' '.github/**/*.md'
```

The configuration enables offline checking and complete fragment validation. Lychee therefore
checks local Markdown files and heading/text fragments without requesting external URLs. The
`share/**/*.md` input is intentionally retained to define the complete checked set. It currently
matches no files and lychee emits the non-fatal warning `No files found for this input source`.

## Baseline

| Status | Command                                                                                                                                                                               | Result                                                | Evidence                                                                                                                                       |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| DONE   | `lychee --no-progress README.md SECURITY.md 'docs/**/*.md' '**/AGENTS.md' 'packages/*/README.md' 'console/**/*.md' 'contrib/**/*.md' 'share/**/*.md' '.github/**/*.md' --format json` | 1,537 total, 588 successful, 123 errors, 826 excluded | The unmatched `share/**/*.md` glob emitted its documented non-fatal warning; JSON output was used for triage and is intentionally not retained |

## Finding Classification

All 123 baseline findings were genuine broken local file or fragment links and were fixed. The
following source-level inventory accounts for every baseline error. The configured
`docs/issues/closed/` exclusion accounts for the 826 historical records excluded from the
baseline; no new exclusions or false-positive classifications were added.

| Source                                                                                                                | Findings | Classification and concise resolution                                                  |
| --------------------------------------------------------------------------------------------------------------------- | -------- | -------------------------------------------------------------------------------------- |
| `.github/skills/add-new-skill/SKILL.md`                                                                               | 2        | Fix — corrected repository-root and missing-reference paths.                           |
| `.github/skills/dev/git-workflow/run-linters/SKILL.md`                                                                | 2        | Fix — corrected sibling skill paths.                                                   |
| `.github/skills/dev/planning/cleanup-completed-issues/SKILL.md`                                                       | 3        | Fix — corrected canonical skill and template paths.                                    |
| `.github/skills/dev/planning/create-adr/SKILL.md`                                                                     | 2        | Fix — corrected template and repository-root paths.                                    |
| `.github/skills/dev/planning/create-issue/SKILL.md`                                                                   | 2        | Fix — corrected template and repository-root paths.                                    |
| `.github/skills/dev/planning/create-refactor-plan/SKILL.md`                                                           | 1        | Fix — corrected the template path.                                                     |
| `.github/skills/dev/rust-code-quality/fix-clippy-warnings/SKILL.md`                                                   | 2        | Fix — corrected related-skill paths.                                                   |
| `README.md`                                                                                                           | 18       | Fix — replaced invalid local workflow badge targets with canonical workflow URLs.      |
| `console/tracker-client/README.md`                                                                                    | 3        | Fix — corrected root license and documentation path depths.                            |
| `contrib/dev-tools/experiments/sccache-docker/04-gha-workflow-experiments/REPORT.md`                                  | 3        | Fix — linked canonical archived issue artifacts.                                       |
| `docs/adrs/20260429000000_keep_database_as_aggregate_supertrait.md`                                                   | 3        | Fix — corrected archived-spec paths.                                                   |
| `docs/adrs/20260512102000_define_tracker_client_peer_id_convention.md`                                                | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`                                                | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`                                                  | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260617093046_reject_wildcard_external_ip.md`                                                             | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260629000000_adopt_independent_package_versioning.md`                                                    | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md`                                             | 1        | Fix — corrected the archived-spec path.                                                |
| `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`                                                      | 3        | Fix — corrected archived-spec paths.                                                   |
| `docs/architecture/events.md`                                                                                         | 1        | Fix — corrected a moved documentation path.                                            |
| `docs/benchmarking.md`                                                                                                | 1        | Fix — corrected a moved report path.                                                   |
| `docs/issues/drafts/1669-01-establish-baseline-analysis.md`                                                           | 1        | Fix — corrected the coupling-report path.                                              |
| `docs/issues/drafts/increase-main-app-integration-test-coverage.md`                                                   | 2        | Fix — corrected historical test-reference paths.                                       |
| `docs/issues/open/1347-overhaul-packages-testing/EPIC.md`                                                             | 4        | Fix — corrected issue-local evidence and archived issue paths.                         |
| `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md`                                   | 6        | Fix — corrected source and archived issue paths.                                       |
| `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md` | 2        | Fix — corrected issue-local evidence paths.                                            |
| `docs/issues/open/1669-overhaul-packages/DECISIONS.md`                                                                | 7        | Fix — corrected paths and replaced removed historic targets with canonical issue URLs. |
| `docs/issues/open/1669-overhaul-packages/EPIC.md`                                                                     | 28       | Fix — corrected paths, fragments, and removed historic targets.                        |
| `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md`                                     | 7        | Fix — corrected paths and decision fragment links.                                     |
| `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-proposed-merge.md`                                 | 1        | Fix — corrected the coupling-report path.                                              |
| `docs/issues/open/1768-refactor-update-dependencies-skill-automation.md`                                              | 2        | Fix — corrected an archived split-issue link.                                          |
| `docs/profiling.md`                                                                                                   | 1        | Fix — corrected a Markdown heading fragment.                                           |
| `docs/security/analysis/reports/README.md`                                                                            | 1        | Fix — corrected the security-policy path.                                              |
| `packages/rest-api-client/README.md`                                                                                  | 3        | Fix — corrected root license and documentation path depths.                            |
| `packages/swarm-coordination-registry/README.md`                                                                      | 1        | Fix — corrected the root license path.                                                 |
| `packages/torrent-repository-benchmarking/README.md`                                                                  | 1        | Fix — corrected the root MIT-0 license path.                                           |
| `packages/tracker-client/README.md`                                                                                   | 3        | Fix — corrected root license and issue-spec path depths.                               |
| `tests/AGENTS.md`                                                                                                     | 1        | Fix — corrected the root documentation path.                                           |
| **Total**                                                                                                             | **123**  | **Fix — all baseline errors resolved.**                                                |

## Fixes Applied

- Corrected all verified moved file paths and Markdown heading fragments.
- Replaced references whose local historical target no longer exists with canonical GitHub issue
  URLs, preserving reader-facing context without adding exclusions.
- Kept `lychee.toml` policy unchanged: `offline = true`, `include_fragments = "full"`, and only
  the documented `docs/issues/closed/` exclusion.

## Final Verification

| Status | Command                                                                                                                                                                               | Result                                                  | Evidence                                                                                                                             |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| DONE   | `lychee --no-progress README.md SECURITY.md 'docs/**/*.md' '**/AGENTS.md' 'packages/*/README.md' 'console/**/*.md' 'contrib/**/*.md' 'share/**/*.md' '.github/**/*.md' --format json` | 1,552 total, 671 successful, **0 errors**, 881 excluded | The unmatched `share/**/*.md` glob emitted its documented non-fatal warning; JSON output was verified and intentionally not retained |
