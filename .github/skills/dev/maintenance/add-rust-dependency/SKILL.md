---
name: add-rust-dependency
description: Guide for safely adding a new Rust crate dependency in torrust-tracker, starting from the latest stable crates.io version, minimizing features, documenting version rationale, and validating with cargo machete and repository quality gates. Use when introducing a new dependency, selecting a crate version, or justifying why an older version is required.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - AGENTS.md
      - .github/agents/implementer.agent.md
      - .github/skills/dev/maintenance/update-dependencies/SKILL.md
---

# Adding a Rust Dependency

Use this workflow when introducing a new crate to `Cargo.toml`.

## Goal

Add only necessary dependencies, prefer the latest stable version, and keep the resulting change
reviewable, justified, and maintainable.

## Skill Links

- `AGENTS.md`
- `.github/agents/implementer.agent.md`
- `.github/skills/dev/maintenance/update-dependencies/SKILL.md`

## Workflow

### Step 1: Confirm a new dependency is necessary

Before adding a crate, check whether the need can be met by:

- the Rust standard library,
- an existing workspace dependency,
- a small local implementation with lower long-term cost.

If one of these options is sufficient, do not add a new crate.

### Step 2: Check the latest stable version first

Identify the latest stable crates.io version before choosing a version.

```bash
cargo search <crate-name> --limit 1
```

Start from the latest stable version by default.

If you must choose an older version, document the reason in the PR/issue spec and, when useful,
in a nearby code comment.

### Step 3: Choose the minimal feature set

Prefer `default-features = false` when appropriate and enable only required features.

```toml
[dependencies]
example-crate = { version = "<latest-stable>", default-features = false, features = ["needed-feature"] }
```

Avoid broad feature enables without a concrete need.

### Step 4: Apply and verify

After editing `Cargo.toml`/`Cargo.lock`:

```bash
cargo update -p <crate-name>
cargo machete
./contrib/dev-tools/git/hooks/pre-commit.sh
```

If checks fail, resolve issues or revert the dependency addition.

### Step 5: Document rationale

In commit/PR/issue notes, record:

- why this crate is needed,
- why alternatives were not selected,
- why a non-latest version is used (if applicable),
- any noteworthy feature-flag choices.

## Constraints

- Do not introduce a dependency without checking latest stable first.
- Do not keep a non-latest version without explicit rationale.
- Do not add dependency bloat when existing dependencies already solve the problem.
- Do not skip `cargo machete` and pre-commit validation.

## Related Skills

- Update existing dependencies: `.github/skills/dev/maintenance/update-dependencies/SKILL.md`
- Commit workflow: `.github/skills/dev/git-workflow/commit-changes/SKILL.md`
