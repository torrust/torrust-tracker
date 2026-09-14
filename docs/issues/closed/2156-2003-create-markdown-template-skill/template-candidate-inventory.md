---
semantic-links:
  skill-links:
    - create-markdown-template
  related-artifacts:
    - docs/templates/SECURITY-ANALYSIS.md
    - docs/security/analysis/README.md
    - .github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md
    - .github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md
---

# Markdown Template Candidate Inventory — Issue #2156

## Purpose

Record the template-like content reviewed for issue #2156 and the rationale for extracting or
retaining it. This inventory limits the change to reusable live authoring guidance and avoids broad
rewrites of examples or historical documents.

## Decisions

| Candidate                                                                                       | Classification                | Decision                                                  | Rationale                                                                                                                                                                         |
| ----------------------------------------------------------------------------------------------- | ----------------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/security/analysis/README.md` per-CVE authoring skeleton                                   | Reusable live template        | Extract to `docs/templates/SECURITY-ANALYSIS.md` and link | The README directed authors to a template but did not provide a separately discoverable canonical template. The shape applies to production, build, and future affecting records. |
| `.github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md` template reference   | Live workflow guidance        | Link to `SECURITY-ANALYSIS.md`                            | The skill should direct authors to the canonical template rather than a README-local authoring skeleton.                                                                          |
| `.github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md` frontmatter list      | Live workflow guidance        | Link to `SECURITY-ANALYSIS.md`                            | The reusable metadata contract belongs in the canonical template; the skill retains scan-specific process steps.                                                                  |
| `docs/issues/closed/1726-1840-workflow-performance-sccache/Q-and-A.md` inline skeleton          | Immutable historical research | Retain unchanged                                          | It documents a closed issue's research method and is not live reusable authoring guidance.                                                                                        |
| `docs/issues/closed/2041-migrate-runtime-service-registry-metadata/evidence.md` scenario record | Immutable historical evidence | Retain unchanged                                          | It documents that issue's evidence method; extracting it would alter historical context without a reuse case.                                                                     |
| Agent output-format sections                                                                    | Agent/chat response contracts | Retain unchanged                                          | They define response behavior, not reusable repository Markdown artifacts.                                                                                                        |
| `docs/copilot-pr-reviews/EXAMPLE-COMPLETED.md`                                                  | Completed illustrative record | Retain unchanged                                          | The canonical reusable template already exists at `docs/templates/COPILOT-SUGGESTIONS-TEMPLATE.md`.                                                                               |

## Verification

- Confirm the new canonical template is indexed in `docs/index.md`.
- Confirm live security-analysis authoring guidance and both related skills link to the template.
- Confirm retained examples and historical records are not modified by this issue.
