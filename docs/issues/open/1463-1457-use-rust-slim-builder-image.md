---
doc-type: issue
issue-type: task
status: open
priority: p3
github-issue: 1463
spec-path: docs/issues/open/1463-1457-use-rust-slim-builder-image.md
branch: "1463-1457-use-rust-slim-builder-image"
related-pr: null
last-updated-utc: 2026-07-20 00:00
semantic-links:
  skill-links:
    - create-issue
    - catalog-security-vulnerabilities
  related-artifacts:
    - Containerfile
    - .github/workflows/container.yaml
    - .github/workflows/security-scan.yaml
    - docs/security/docker/scans/torrust-tracker.md
    - docs/security/docker/scans/build-images.md
    - docs/security/docker/scans/README.md
    - docs/security/analysis/README.md
    - docs/security/analysis/non-affecting/2026-06-10_containerfile-trixie-cves.md
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
---

<!-- skill-link: create-issue -->
<!-- skill-link: catalog-security-vulnerabilities -->

# Issue #1463 - Minimize Containerfile build-stage images

## Goal

Replace the `chef` stage's `rust:trixie` base image with `rust:slim-trixie` if the
complete container build and test workflow needs only a small, explicit set of added
packages. Independently minimize the existing `tester` stage and evaluate whether the
separate `gcc` stage has a practical slimmer alternative. Reduce build-image size,
installed package inventory, vulnerability exposure, and maintenance burden without
weakening build or test coverage.

## Background

The Containerfile currently uses `rust:trixie` for the shared `chef` stage and
`rust:slim-trixie` for the separate `tester` stage. Because all dependency and build
stages inherit from `chef`, changing this one base image affects the complete Rust build
path. The final production image inherits from `gcr.io/distroless/cc-debian13:debug`, so
this change does not directly reduce the size or package inventory of the published
runtime image.

Issue #1463 originally reported that `cargo binstall` was unavailable after trying the
slim image. The current tester stage demonstrates the likely cause and remedy: slim does
not include `curl`, so the `cargo-binstall` installer must be preceded by a minimal package
installation. The issue's April 2026 comments also concluded that full and slim Trixie
images had the same vulnerabilities at that time. A later repository security analysis
and the fresh measurements below show that slim now has a materially smaller package and
scanner-finding inventory. Scanner results are time-sensitive and must be captured again
during implementation.

### Preliminary investigation

Measurements were taken on 2026-07-20 for fresh `linux/amd64` pulls:

| Metric                       | `rust:trixie`        | `rust:slim-trixie`  | Difference                   |
| ---------------------------- | -------------------- | ------------------- | ---------------------------- |
| Image digest                 | `sha256:9a2cd304...` | `sha256:5c6f46a...` | Different current images     |
| Docker image size            | 1,662.7 MB           | 921.0 MB            | 741.7 MB smaller (44.6%)     |
| Installed Debian packages    | 455                  | 119                 | 336 fewer packages (73.8%)   |
| Trivy vulnerability findings | 2,148                | 1,008               | 1,140 fewer findings (53.1%) |

The Trivy totals use Trivy 0.69.3 and its database as of the measurement date. They count
findings rather than unique CVEs and are evidence for comparison, not a permanent security
claim.

The slim image already contains `bash`, `cc`, `gcc`, and `perl`. It does not contain
`curl`, `make`, `g++`, `pkg-config`, `git`, or `xz`. An isolated probe installed only
`curl` with `--no-install-recommends`, then successfully installed and executed the exact
tools used by the current Containerfile:

- `torrust-cargo-chef` 0.1.78
- `cargo-nextest` 0.9.140

This resolves the tool-installation uncertainty but does not prove that every workspace
dependency compiles or links under slim. The complete multi-stage build remains the
decisive check.

## Scope

### In Scope

- Re-measure the current full and slim Rust image size, installed package count, and
  vulnerability findings using pinned image digests in the evidence.
- Change the `chef` stage from `rust:trixie` to `rust:slim-trixie`.
- Install only packages demonstrated to be necessary, using `--no-install-recommends` and
  removing APT index files in the same layer.
- Independently review and minimize the existing `rust:slim-trixie` tester stage, including
  its explicitly installed and transitive APT packages.
- Build and test every Containerfile target exercised by the container and testing CI
  workflows.
- Compare the resulting chef/build-stage package inventory and vulnerability findings with
  the baseline, including packages reintroduced by APT dependencies.
- Evaluate slimmer alternatives for the `gcc:trixie` stage and adopt one only if compiling
  `su-exec` remains simple and the resulting package inventory is clearly reduced.
- Update the existing Trixie vulnerability analysis with the new image digest, findings,
  and build-stage rationale.
- Re-scan the final production `release` image and append the result to
  `docs/security/docker/scans/torrust-tracker.md`, even if its distroless base is unchanged.
- Add `docs/security/docker/scans/build-images.md` as one consolidated history for the
  foundational `chef`, `tester`, and `gcc` stages, and link it from the scan index.
- Implement and validate the `chef`, `tester`, and `gcc` stage changes independently so
  each stage can be committed and reviewed separately.
- Keep the full image if slim requires enough added packages or special-case maintenance to
  erase the measured simplification benefit; document that decision with evidence.

### Out of Scope

- Replacing or changing the distroless runtime image.
- Removing containerized unit tests or reducing test coverage.
- Fixing vulnerabilities in upstream Debian or Docker Official Images.
- Optimizing application dependencies or Rust compilation time.

## Decision Rule

Adopt a slimmer image for a stage when all required builds and tests pass and the final
package additions remain a small, understandable build-tool set that preserves a material
reduction in package inventory and scanner findings. Review that package list qualitatively;
no fixed percentage or package cap is required. If compilation requires reconstructing most
of a full image's general-purpose toolchain, retain the current image and record the measured
blocker instead of adding a large maintenance list.

## Scan Recording Policy

The production and build-stage reports answer different questions and must remain separate:

- `docs/security/docker/scans/torrust-tracker.md` records the deployed `release` image's
  security posture. Re-scan it after these build-stage changes to prove the final artifact
  did not regress, even though its distroless base is unchanged.
- `docs/security/docker/scans/build-images.md` records one consolidated comparison of the
  foundational `chef`, `tester`, and `gcc` stages. Keeping these related ephemeral stages
  together makes package and finding differences easier to review without overstating them
  as production exposure.
- `docs/security/analysis/` remains the single catalog for durable CVE impact decisions.
  Scan reports should link to catalog entries rather than repeat full exploitability
  analyses.

Continue daily automated scanning for the published production image. Scan the foundational
build stages when their base images or installed packages change and during the quarterly
security review. Do not add daily build-stage SARIF uploads in this issue; these unpublished,
ephemeral stages have a lower risk and would mix build-chain findings into the production
security signal.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                                                                   |
| --- | ------ | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Establish a fresh base-image baseline     | Digests, sizes, package counts, tool inventory, and Trivy summaries recorded in this spec                                                 |
| T2  | DONE   | Probe minimal cargo-tool installation     | Exact pinned tools install and run on slim after adding only `curl`                                                                       |
| T3  | TODO   | Change and validate the chef stage        | Minimal APT layer; full dependent build/test path passes; delivered as an independent commit                                              |
| T4  | TODO   | Minimize and validate the tester stage    | Only demonstrated runtime test tools remain; test targets pass; delivered as an independent commit                                        |
| T5  | TODO   | Evaluate and validate a slimmer GCC stage | Candidate builds `su-exec` cleanly with a clearly smaller inventory, or evidence supports retaining `gcc:trixie`; delivered independently |
| T6  | TODO   | Measure the resulting build stages        | Post-change size, package, and Trivy comparison includes transitive APT packages                                                          |
| T7  | TODO   | Apply the decision rule                   | Each stage decision stands on its own evidence and can be reverted independently                                                          |
| T8  | TODO   | Record build-stage scan history           | New consolidated `build-images.md` records chef, tester, and GCC commands, digests, package counts, and findings                          |
| T9  | TODO   | Refresh production scan history           | Rebuilt release image is scanned and appended to `torrust-tracker.md`; scan index reflects both report types                              |
| T10 | TODO   | Update security analysis documentation    | Existing catalog summary records current digests, scan date, counts, commands, and conclusion without bulky raw scan output               |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted for the existing GitHub issue
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue number and parent EPIC added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-20 00:00 UTC - GitHub Copilot - Read issue #1463 and both comments; created the local issue branch and drafted this spec - local investigation results recorded above
- 2026-07-20 00:00 UTC - GitHub Copilot - Compared fresh full/slim images and verified the pinned cargo tools install on slim with only `curl` added - T1 and T2 completed
- 2026-07-20 00:00 UTC - User/maintainer - Approved the stage-by-stage scope, independent commits, and separate runtime/build-image scan reports - specification approved

## Acceptance Criteria

- [ ] AC1: The `chef` stage uses `rust:slim-trixie`, or evidence documents why the slim image fails the decision rule and the full image is retained.
- [ ] AC2: Every package explicitly added to the slim chef stage is tied to a reproducible build or tool-installation requirement.
- [ ] AC3: The tester stage is independently minimized and validated without reducing existing test scope.
- [ ] AC4: Before/after evidence records image digests, image sizes, installed package counts, and vulnerability findings using the same commands and scanner database.
- [ ] AC5: The adopted result has a materially smaller installed package inventory than `rust:trixie`; no target percentage is assumed before transitive dependencies are measured.
- [ ] AC6: The `gcc` stage uses a practical slimmer alternative, or measured evidence documents why `gcc:trixie` is retained.
- [ ] AC7: The chef, tester, and GCC changes are implemented, validated, and committed independently.
- [ ] AC8: `build-images.md` provides a consolidated scan history for the foundational build stages without mixing their lower-risk status into the production report.
- [ ] AC9: `torrust-tracker.md` contains a new post-change release-image scan proving the production artifact did not regress.
- [ ] AC10: The existing security analysis catalog summarizes the implemented images, current scan evidence, comparison commands, and the fact that these stages are build-time only.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant container workflow tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Build the Containerfile targets used by `.github/workflows/container.yaml`.
- Build the Containerfile targets used by the container-based test workflow.
- After each independent stage change, rerun the narrowest dependent Containerfile target
  before changing another stage.
- Run the repository's pre-push checks when the implementation is ready for review.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                             | Command/Steps                                                                                                                                         | Expected Result                                                                            | Status | Evidence                                                              |
| --- | ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ | ------ | --------------------------------------------------------------------- |
| M1  | Compare fresh base images            | Pull both images by tag, record resolved digests, inspect `.Size`, and count the Debian package-query output                                          | Reproducible baseline shows the exact size and package-inventory delta                     | DONE   | Preliminary investigation table in this spec                          |
| M2  | Verify minimal cargo tooling         | On `rust:slim-trixie`, install only `curl` with `--no-install-recommends`, run the existing `cargo-binstall` installer, then install the pinned tools | `cargo chef --version` and `cargo nextest --version` succeed                               | DONE   | Preliminary investigation and progress log in this spec               |
| M3  | Validate chef change independently   | Build the dependent debug and release paths without relying on host artifacts before changing tester or GCC                                           | Chef-dependent compilation succeeds and the change is ready for its own commit             | TODO   | Build logs or CI run URL                                              |
| M4  | Validate tester change independently | Run the complete containerized test paths after changing tester and before changing GCC                                                               | Existing tests execute successfully and the tester change is ready for its own commit      | TODO   | Build/test logs or CI run URL                                         |
| M5  | Inspect added package closure        | List explicit and transitive packages after the APT install and compare them with the full image                                                      | Every explicit package is necessary and the resulting inventory remains materially smaller | TODO   | Package comparison output                                             |
| M6  | Evaluate a slimmer GCC stage         | Compare practical candidate images, compile `su-exec`, and inspect the resulting package closure                                                      | Adopt a clearly simpler candidate or document why the current GCC image remains preferable | TODO   | Build and package comparison output                                   |
| M7  | Scan foundational build stages       | Build tagged `chef`, `tester`, and `gcc` targets, then scan all three with the same Trivy version/database                                            | Consolidated report shows comparable findings and preserves their build-time risk context  | TODO   | `docs/security/docker/scans/build-images.md`                          |
| M8  | Scan and smoke-test release image    | Build and scan `release`, start it, and exercise its configured health check                                                                          | Production scan history is refreshed; runtime starts and becomes healthy                   | TODO   | `docs/security/docker/scans/torrust-tracker.md` plus container status |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.
- Scanner totals are comparable only when the scanner version and vulnerability database are
  held constant for both images.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                      |
| ----- | ---------------------- | ------------------------------------------------------------- |
| AC1   | TODO                   | Containerfile diff or documented decision report              |
| AC2   | TODO                   | Package-to-requirement table from M5                          |
| AC3   | TODO                   | Tester build/test logs from M4                                |
| AC4   | TODO                   | Before/after measurement table                                |
| AC5   | TODO                   | Package inventory comparison from M5                          |
| AC6   | TODO                   | GCC-stage comparison from M6                                  |
| AC7   | TODO                   | Independent commit history and stage-specific validation logs |
| AC8   | TODO                   | Consolidated build-image scan report from M7                  |
| AC9   | TODO                   | Updated production scan report from M8                        |
| AC10  | TODO                   | Updated security catalog entry                                |

## Risks and Trade-offs

- The full and slim images are mutable tags. Record resolved digests with every comparison
  so later scans can explain changed results.
- APT-installing missing tools can gradually recreate the full image and transfer
  maintenance from the upstream image to this Containerfile. The decision rule prevents
  adopting slim when that trade-off is poor.
- Fewer packages and scanner findings reduce potential build-stage exposure, but do not
  directly harden the published runtime image because chef is discarded after the build.
- Slim may expose undeclared native-tool assumptions in transitive Rust dependencies. Treat
  those failures as useful dependency evidence and add only tools required by reproducible
  failures.
- Base-image download and cold-build time should improve, while package installation adds a
  network-dependent APT step. Compare cold builds if the net CI effect is material.
- The current cargo tool probe was performed on `linux/amd64`; CI and supported build
  platforms must also succeed before closing the issue.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/1457>
- Original issue and comments: <https://github.com/torrust/torrust-tracker/issues/1463>
- Related security-scanning issue: <https://github.com/torrust/torrust-tracker/issues/1459>
- Trixie upgrade PR: <https://github.com/torrust/torrust-tracker/pull/1629>
- Security analysis process issue: <https://github.com/torrust/torrust-tracker/issues/1898>
