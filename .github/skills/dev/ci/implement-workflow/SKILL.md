---
name: implement-workflow
description: "Use when: adding or materially changing GitHub Actions workflows, CI jobs, workflow scripts, artifacts, matrices, permissions, or provider integration."
metadata:
  author: torrust
  version: "1.0"
---

# Implement CI Workflows

## Rule

Keep CI workflow definitions as thin platform adapters. Put non-trivial
decisions, data transformation, validation, and report rendering in
repository-owned tools with a documented command-line interface and automated
tests. Prefer Rust for that logic unless another implementation language is
justified. Workflow files may retain only provider-specific triggering,
permissions, runner setup, checkout, scheduling, artifact transport, and the
small adapter calls needed to invoke the tool. Document unavoidable
provider-specific behavior and the practical portable alternative.

## Design The Boundary

Before editing a workflow, identify which behavior is provider-specific and
which is repository behavior.

- Keep triggering, permissions, runner selection, pinned action setup,
  immutable revision checkout, matrix scheduling, artifact transport, and
  provider outputs in the workflow.
- Move decisions, parsing, validation, aggregation, numeric calculations, and
  report formatting into a repository-owned tool when they are more than simple
  orchestration.
- Give repository tools ordinary command-line arguments, standard input or JSON
  files, and standard output. Do not require GitHub environment variables or
  GitHub output files in tool interfaces.
- Document unavoidable provider behavior and name the practical portable
  alternative in the owning tool, workflow, or skill.

## Implement Safely

1. Preserve the existing trust boundary before changing workflow behavior.
   Use least-privilege permissions and do not expose secrets to untrusted pull
   request code.
2. Pin actions to the repository's established versioning convention. Use
   immutable pull request revisions when base/head behavior is compared.
3. Keep shell only for simple adapter work such as command invocation, output
   assignment, and artifact paths. Move substantive shell loops, `jq`/`awk`
   transformations, or presentation logic into tested repository tooling.
4. Add focused automated tests for repository-tool behavior. Test the tool's
   portable contract separately from provider orchestration.
5. Validate the workflow YAML locally, then obtain hosted evidence for
   provider-only behavior such as permissions, matrix expansion, artifacts, and
   fork pull requests.

## Validate

Run the narrowest checks applicable to the changed surface first:

```bash
linter yaml
git diff --check
```

For repository tooling, also run its focused formatter, tests, and strict
linting. Before committing, run the repository pre-commit gate. Record hosted
workflow evidence separately when it cannot be reproduced locally.

## Related Guidance

- [Root repository instructions](../../../../../AGENTS.md) define the
  repository-wide shell-versus-Rust threshold.
- [Workflow-local instructions](../../../../workflows/AGENTS.md) define local
  GitHub Actions constraints.
