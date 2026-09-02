# `docs/analysis/` — Analysis Documents

This directory contains analysis documents that study a concrete feature, component, or
aspect of the application in depth. Analyses are typically produced **before** defining a
refactoring plan, introducing a new feature, or making architectural decisions.

## Purpose

An analysis document answers questions like:

- How does this part of the system work today?
- What are the pain points, risks, and gaps?
- What options exist for improvement?
- What is the current state of the code?

Analyses are the **input** to further work: they feed into feature definitions, EPIC
specifications, issue specs, and ADRs.

## Timestamp Prefix Convention

Analysis folders use a **timestamp prefix** like ADRs to make it clear when the analysis
was written:

```text
docs/analysis/
├── AGENTS.md
├── 20260716-shutdown-process/
│   └── README.md
└── ...
```

## Lifecycle

- **Analyses may become outdated** if the object of their analysis has changed since they
  were written. Always check the timestamp and verify against the current code before
  relying on an old analysis.
- **Analyses may stay relevant** if the code has not changed in the area they study.
- **Old analyses can be cleaned up** when they are no longer relevant. The timestamp
  prefix makes it easy to identify which ones are older.
- **Consider reviewing** analyses older than 6 months before using them as a basis for
  decisions.

## Related

- [Research documents](../research/) — external investigations of technologies and patterns
- [Feature definitions](../features/) — product-oriented descriptions of desired features
- [Issue specs](../issues/) — concrete task breakdowns linked to GitHub issues
- [ADRs](../adrs/) — architectural decision records
