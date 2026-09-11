---
semantic-links:
  skill-links:
    - create-markdown-template
  related-artifacts:
    - docs/index.md
    - .github/skills/dev/planning/create-markdown-template/SKILL.md
---

<!-- skill-link: create-markdown-template -->

# Markdown Templates

`docs/templates/` contains the canonical reusable Markdown structures for repository documents and GitHub-surface bodies. Copy the relevant template, replace its placeholders, and keep concrete records in their documented destination.

Templates intended specifically for GitHub's contributor-facing template picker are an exception: place those in `.github/PULL_REQUEST_TEMPLATE/` only when that GitHub-native feature is required. Do not create an adapter merely to duplicate a canonical repository template.

| Template                                                           | Purpose                                                               | Intended destination                                                         | Primary workflow or agent                      |
| ------------------------------------------------------------------ | --------------------------------------------------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------------- |
| [ADR.md](ADR.md)                                                   | Records a significant architectural decision.                         | `docs/adrs/` or `packages/<package>/docs/adrs/` according to decision scope. | `create-adr`                                   |
| [AGENT-REVIEW-REPORTS.md](AGENT-REVIEW-REPORTS.md)                 | Records append-only independent review results.                       | `agent-review-reports.md` in a folder-style issue specification.             | Complexity Auditor, Task Reviewer, PR Reviewer |
| [CARGO-DEPENDENCY-UPDATE-PR.md](CARGO-DEPENDENCY-UPDATE-PR.md)     | Supplies the reusable GitHub PR body for Cargo dependency updates.    | GitHub PR description; do not commit a populated copy.                       | `update-dependencies`                          |
| [COPILOT-SUGGESTIONS-TEMPLATE.md](COPILOT-SUGGESTIONS-TEMPLATE.md) | Tracks Copilot review-suggestion decisions and resolutions.           | `docs/copilot-pr-reviews/pr-<number>-copilot-suggestions.md`                 | `process-copilot-suggestions`                  |
| [EPIC.md](EPIC.md)                                                 | Defines an epic issue specification and its subissues.                | `docs/issues/drafts/` or `docs/issues/open/` in a folder-style spec.         | `create-issue`                                 |
| [IMPLEMENTATION-RETROSPECTIVE.md](IMPLEMENTATION-RETROSPECTIVE.md) | Records material implementation discoveries and process improvements. | `implementation-retrospective.md` in a folder-style issue specification.     | `create-issue`                                 |
| [ISSUE.md](ISSUE.md)                                               | Defines a task, bug, feature, or enhancement specification.           | `docs/issues/drafts/` or `docs/issues/open/` in a folder-style spec.         | `create-issue`                                 |
| [MANUAL-VERIFICATION-EVIDENCE.md](MANUAL-VERIFICATION-EVIDENCE.md) | Records real human-oriented verification evidence.                    | `manual-verification-evidence.md` in a folder-style issue specification.     | `create-issue`                                 |
| [PR-REVIEW-FEEDBACK-TEMPLATE.md](PR-REVIEW-FEEDBACK-TEMPLATE.md)   | Tracks review findings, decisions, replies, and resolutions.          | `docs/pr-review-feedback/pr-<number>-review-feedback.md`                     | `process-pr-review-feedback`                   |
| [REFACTOR-PLAN.md](REFACTOR-PLAN.md)                               | Defines a refactor plan and ordered implementation items.             | `docs/refactor-plans/drafts/` or `docs/refactor-plans/open/`                 | `create-refactor-plan`                         |
| [SECURITY-ANALYSIS.md](SECURITY-ANALYSIS.md)                       | Analyzes an approved public security finding or vulnerability.        | `docs/security/analysis/production/`, `build/`, or `affecting/`              | `catalog-security-vulnerabilities`             |
| [SECURITY-REPORT.md](SECURITY-REPORT.md)                           | Records handled coordinated-disclosure reports.                       | `docs/security/analysis/reports/`                                            | `docs/security/vulnerability-remediation.md`   |
