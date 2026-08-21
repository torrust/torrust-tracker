---
name: "Update Dependencies"
description: "Update Torrust Tracker Cargo dependencies using the repository's required workflow"
argument-hint: "Optional package name, version constraint, or update scope"
agent: "agent"
---

Update the Cargo dependencies in this workspace, strictly following the canonical [dependency update skill](../skills/dev/maintenance/update-dependencies/SKILL.md) and all applicable repository instructions.

Scope: ${input:dependency scope:Update all eligible dependencies}

Treat this as an end-to-end maintenance task. Inspect the current worktree and dependency graph, classify the update as trivial or breaking, then create the appropriate branch before changing any dependencies. Make only the necessary changes, run the required focused validation and repository quality checks, and report the exact updates, validation results, and any deferred breaking migrations.

After successful validation, make a GPG-signed commit, push it to the configured fork remote, and open a pull request targeting `torrust/torrust-tracker:develop`. Request sandbox or user approval whenever an operation requires it. Do not bypass required approval or GPG-signing protections.
