---
name: create-markdown-template
description: Create or maintain reusable Markdown document templates in the torrust-tracker repository. Use when creating a Markdown template, extracting a reusable template from documentation, adding a document skeleton, or deciding whether template-like Markdown belongs in docs/templates. Triggers on "create markdown template", "add markdown template", "document template", "extract template", or "reusable Markdown template".
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/templates/
      - docs/index.md
      - docs/AGENTS.md
      - .github/skills/add-new-skill/SKILL.md
      - .github/skills/dev/planning/write-markdown-docs/SKILL.md
---

# Creating Markdown Templates

Use this workflow for a reusable repository Markdown document shape. Templates are canonical
starting points, not generators and not replacements for concrete issue, evidence, or historical
records.

## Decide Whether a Template Is Needed

Create or extract a template only when all of these are true:

1. More than one current or expected document shares a stable structure.
2. The structure reduces repeated authoring decisions or prevents a recurring omission.
3. A concrete document can link to a canonical source without losing necessary local context.

Do not extract:

- short illustrative snippets that teach a local point;
- immutable issue, research, evidence, or GitHub-surface history;
- agent chat-response formats; or
- a one-off document with no expected reuse.

Record the classification and rationale in the related issue specification or other durable planning
artifact before moving embedded content. Do not perform broad snippet cleanup merely because it
resembles a template.

## Create or Update the Template

1. Place the reusable Markdown file in `docs/templates/`.
2. Use an uppercase descriptive filename, such as `SECURITY-ANALYSIS.md`.
3. Add YAML frontmatter with high-signal `semantic-links.related-artifacts` entries. Follow the
   [semantic skill-link convention](../../../../../docs/skills/semantic-skill-link-convention.md).
4. Use placeholders that describe required author input. Do not copy findings, secrets, tokens, raw
   logs, or stale concrete data into a reusable template.
5. State whether the template is intended for new concrete documents only and where those documents
   belong.
6. Add the template to the catalog in `docs/index.md` and update a relevant collection README when
   authors need local procedural context.

## Link Instead of Duplicating

When a canonical template exists, live documentation and skills should link to it rather than repeat
the full reusable body. Keep short excerpts when they demonstrate a local syntax or reader action.
Keep immutable historical records unchanged unless another scoped change independently requires an
update.

A collection README may retain its purpose, classification rules, and creation location. Replace only
the reusable authoring skeleton with a link to the canonical template.

## Validate

Before committing:

```bash
linter markdown
linter cspell
```

Run `linter all` when the template or related workflow changes affect more than Markdown. Validate
that:

- the template appears in `docs/templates/` and `docs/index.md`;
- the linked collection README and skills point to the template;
- the template has valid frontmatter and no accidental `#NUMBER` enumeration; and
- retained inline examples have a recorded rationale.

## Related Workflows

- [Writing Markdown documentation](../write-markdown-docs/SKILL.md)
- [Creating new agent skills](../../../add-new-skill/SKILL.md)
