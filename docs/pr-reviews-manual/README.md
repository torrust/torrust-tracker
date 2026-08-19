# Manual PR Reviews

This directory contains manual (human-assisted) reviews of pull requests on the
[torrust/torrust-tracker](https://github.com/torrust/torrust-tracker) repository.

Each review lives in its own subfolder named `pr-<NUMBER>/` and may contain
multiple Markdown files — one per review pass or analysis task (e.g. code review,
protocol compliance audit, spec comparison).

## Structure

```text
docs/pr-reviews-manual/
├── README.md                  ← this file
└── pr-2050/
    ├── review-pass-1.md       ← first-pass findings, actions, and questions
    ├── protocol-compliance.md ← optional: I2P spec compliance audit
    └── ...
```

## Differences from `docs/pr-reviews/`

| Aspect       | `docs/pr-reviews/`            | `docs/pr-reviews-manual/`                           |
| ------------ | ----------------------------- | --------------------------------------------------- |
| Scope        | Copilot suggestion processing | Manual code review, design analysis, spec audits    |
| Workflow     | Automated suggestion threads  | Human-driven analysis with AI assistance            |
| Output       | Suggestion tracker files      | Multi-file review packages per PR                   |
| Final report | N/A                           | Single consolidated report with actions + questions |
