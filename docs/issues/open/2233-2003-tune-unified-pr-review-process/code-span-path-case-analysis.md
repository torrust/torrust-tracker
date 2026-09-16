# Code-Span Path Case Analysis

Generated: 2026-09-16 12:53 UTC

This file records the first-pass taxonomy for Markdown inline code spans that look like repository
paths but do not resolve in the current tree. It supports the T2 decision in this issue: whether to
add a checker beyond Lychee, and if so which cases should be strict, ignored by syntax, or allowed
with explicit review evidence.

The complete case list is in
`docs/issues/open/2233-2003-tune-unified-pr-review-process/code-span-path-case-inventory.tsv`.

## Scan Scope

The scan inspected Git-tracked Markdown files and collected inline code spans beginning with one of
these repository-relative prefixes:

- `docs/`
- `.github/`
- `contrib/`
- `packages/`
- `src/`

A case appears in the inventory only when the code span does not resolve as a current repository
path. This is intentionally different from Lychee: Lychee validates Markdown links, while this scan
looks at plain inline code text.

## Observed Case Types

| Case type | Count | What it appears to represent | Possible approach |
| --------- | ----- | ---------------------------- | ----------------- |
| `historical-closed-doc` | 493 | Closed issue specs and closed refactor plans that preserve old paths from earlier repository states. | Usually do not rewrite. Either exclude historical folders or require coarse historical-retention policy. |
| `directory-reference` | 118 | Directory-like spans ending in `/`; some are deleted directories, some are intended future or category directories. | Treat current directories strictly only outside historical/template contexts. |
| `historical-review-record` | 79 | Immutable PR review audit records preserving the reviewed path at review time. | Exclude immutable review records from strict current-tree validation. |
| `angle-placeholder` | 39 | Template paths containing placeholders such as `<PR_NUMBER>` or `<id>`. | Exclude by syntax as non-literal examples. |
| `glob-pattern` | 35 | Path patterns such as `.github/agents/*.agent.md`. | Exclude by syntax or validate with glob semantics separately. |
| `retired-source-path` | 31 | Old source or package paths referenced by ADRs or migration notes. | Keep only when historical context is explicit; otherwise fix or allowlist. |
| `rust-source-example` | 22 | Example Rust paths or source paths from older layouts. | Examples should use obvious placeholder syntax or be explicitly allowed. |
| `other-missing-literal` | 16 | Literal-looking paths that do not fit another category. | Review individually; likely the highest-value strict-check target. |
| `brace-placeholder` | 13 | Template or compact path patterns using braces. | Exclude by syntax as non-literal, or validate with pattern semantics separately. |
| `historical-open-issue-reference` | 11 | Paths under `docs/issues/open/` that no longer resolve, often because issue specs moved or changed shape. | Review individually unless source is historical or the path is intentionally illustrative. |
| `ellipsis-placeholder` | 10 | Path sketches containing `...`. | Exclude by syntax as non-literal examples. |
| `historical-legacy-issue-path` | 9 | Older issue-path convention references under `docs/issues/<number>...`. | Treat as historical convention references; fix only if the document is current guidance. |
| `dated-template` | 3 | Naming templates using timestamp placeholders. | Exclude by syntax as non-literal examples. |
| `html-comment` | 3 | Code spans inside Markdown HTML comments. | Ignore for user-facing path validation unless comments are treated as normative metadata. |
| `retired-workflow-or-experiment` | 2 | Experimental workflow files that were intentionally removed or moved. | Keep as historical experiment evidence or allowlist. |
| `security-analysis-path` | 1 | Security-analysis document path that no longer resolves. | Review individually; may indicate stale security documentation. |

## Initial Observations

The largest groups are not ordinary documentation mistakes. They are historical records, closed
implementation notes, templates, and examples. A strict checker over every non-resolving code span
would require a large baseline exception set before it could catch new mistakes.

The most promising strict target is narrower: literal-looking paths in current guidance and open
issue specs, excluding placeholder syntax, globs, HTML comments, immutable review records, and closed
historical documentation. That approach would complement Lychee without turning historical evidence
into churn.

## Questions for Group Analysis

- Should closed issue folders and PR review records be excluded entirely because they are historical
  records?
- Should placeholder syntax such as `<id>`, `{number}`, `*`, and `...` be excluded before path
  validation?
- Should directory references be checked with different rules from file references?
- Should examples be required to use unmistakable placeholder names instead of realistic missing
  paths such as `src/bar.rs`?
- Should current guidance files under `.github/`, `AGENTS.md`, and `docs/` be the first strict
  enforcement target?
