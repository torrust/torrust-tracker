---
doc-type: frontmatter-contract
status: approved
github-issue: 2265
spec-path: docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-v1-contract.md
last-updated-utc: "2026-09-21 14:33"
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - issue #2265
    - docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-inventory.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates/ISSUE.md
    - docs/templates/EPIC.md
    - docs/external-snapshots/open-knowledge-format/0.2/SPEC.md
---

# Frontmatter V1 Contract

This is the approved input for issue #2266. Rust types are the canonical executable model; this
document explains policy and compatibility that types alone cannot convey.

## Scope And Versioning

The contract parses every present Markdown frontmatter block. It does not require frontmatter on
every Markdown document and does not define a closed schema for every document class.

Strict issue and EPIC profiles carry `schema-version: 1` as an unquoted YAML integer. This explicit
opt-in makes profile evolution observable and lets a validator distinguish prospective v1 records
from historical records without using file age or Git history. The contract document version and
the future Rust crate version are not substitutes for this field.

The value is fixed at `1`. A future incompatible profile must use a new integer and define its own
compatibility policy. New templates must emit the field. Existing documents are not rewritten only
to add it.

## Universal Envelope

Every parsed frontmatter block is a YAML mapping. A non-mapping document is invalid YAML frontmatter.
The only universal repository-owned extension is an optional `semantic-links` mapping:

```yaml
semantic-links:
  skill-links: [write-markdown-docs]
  related-artifacts: [docs/AGENTS.md]
```

When present, `skill-links` and `related-artifacts` are sequences of strings. The keys may be
omitted independently. Other top-level fields are interpreted only by a recognized profile or an
externally owned schema.

For repository-owned profiles such as issue and EPIC documents, `semantic-links` is read only from
the top level. For Agent Skill and agent-profile frontmatter, the external schema remains
authoritative and the v1 validator reads only `metadata.semantic-links`; it must neither require nor
reject other top-level keys. If an external document contains both top-level `semantic-links` and
`metadata.semantic-links`, the validator validates only the nested value, ignores the top-level value
for v1 semantics, and emits no conflict diagnostic.

## Scalar Policy

YAML values are not coerced across contract types. Integers must be YAML integers, `null` must be
the YAML null literal, lists must be YAML sequences, and mappings must be YAML mappings. Strings
that look like an integer, date, boolean, or null value must be quoted when a string is required.

`last-updated-utc` is a required double-quoted string in `YYYY-MM-DD HH:MM` UTC form for strict
profiles. The format intentionally follows existing templates and lifecycle tooling. A future
schema version may adopt RFC 3339 after an explicit migration decision.

## Strict Profiles

Strict profiles apply only when `schema-version: 1` is present and `doc-type` is `issue` or `epic`.
The future validator must reject an unknown unprefixed field in either strict profile. An `x-` field
is an experimental extension: preserve it, emit a warning, and do not give it contract semantics.

### Document-Type Coverage

The inventory records document classes beyond the two strict v1 profiles. This table defines their
current boundary so a validator does not accidentally apply issue/EPIC rules to unrelated records.

| Document class or `doc-type` family | V1 treatment | Profile status |
| --- | --- | --- |
| `issue` | Strict when `schema-version: 1` is present. | Addressed by v1. |
| `epic` | Strict when `schema-version: 1` is present. | Addressed by v1. |
| Agent Skills and agent profiles | Inspect only `metadata.semantic-links`; preserve their externally governed top-level metadata. | Externally governed; not a Torrust v1 profile. |
| Test/refactor plans | Preserve fields and apply only universal envelope rules. | Deferred. |
| Manual-verification records | Preserve fields and apply only universal envelope rules. | Deferred. |
| Review, security, analysis, research, and other evidence records | Preserve fields and apply only universal envelope rules. | Deferred. |
| Unrecognized or one-off document classes | Preserve fields and emit no profile diagnostic. | Permissive; future profile candidate. |

`semantic-links` remains the only universal repository-owned extension. The deferred classes do not
inherit the issue/EPIC required fields, lifecycle enum, or unknown-field policy.

### Issue

| Field | Required | Type and accepted value |
| --- | --- | --- |
| `schema-version` | Yes | Integer `1`. |
| `doc-type` | Yes | String `issue`. |
| `issue-type` | Yes | String: `task`, `bug`, `feature`, or `enhancement`. |
| `status` | Yes | String: `draft`, `planned`, `in-progress`, `blocked`, `in-review`, or `done`. |
| `priority` | Yes | String: `p0`, `p1`, `p2`, or `p3`. |
| `epic` | Yes | Positive integer or `null`. |
| `github-issue` | Yes | Positive integer or `null`. |
| `spec-path` | Yes | Non-empty repository-relative path string. |
| `branch` | Yes | Non-empty string. |
| `related-pr` | Yes | Positive integer or `null`. |
| `last-updated-utc` | Yes | Quoted UTC minute string. |
| `semantic-links` | Yes | Universal envelope mapping. |

### EPIC

| Field | Required | Type and accepted value |
| --- | --- | --- |
| `schema-version` | Yes | Integer `1`. |
| `doc-type` | Yes | String `epic`. |
| `status` | Yes | Issue lifecycle string. |
| `epic` | Yes | Positive integer or `null`; identifies the parent EPIC when applicable. |
| `github-issue` | Yes | Positive integer or `null`. |
| `spec-path` | Yes | Non-empty repository-relative path string. |
| `epic-owner` | Yes | Non-empty string or `null`. |
| `last-updated-utc` | Yes | Quoted UTC minute string. |
| `semantic-links` | Yes | Universal envelope mapping. |

The `status` lifecycle is profile-specific. The location invariant is repository-aware: a v1 draft
spec uses `draft`; an open spec does not use `draft` or `done`; a closed spec uses `done`.

## Provisional Semantic References

`skill-links` accepts only names matching `[a-z0-9]+(-[a-z0-9]+)*`. A repository-aware layer may
check that a name identifies a tracked skill.

`related-artifacts` accepts this frozen union:

1. A non-empty repository-relative file or directory path, including a root-level path such as
   `Cargo.toml`.
2. `issue #<positive-integer>`.
3. `review-finding:pr-<positive-integer>-<lowercase-id>`.

Absolute URLs, bare filenames intended as local references, externally qualified issue text, and
new tagged forms are invalid in a newly authored v1 strict profile. A document's own GitHub issue
and EPIC relationship belong in dedicated fields, not in this list.

Syntax belongs to the structural layer. Path existence, skill discovery, issue lifecycle/path
consistency, and review-finding resolution belong to the repository-aware layer. The latter must
not change YAML deserialization outcomes.

## Enforcement And Compatibility

| Location or class | Mode | Initial diagnostic severity |
| --- | --- | --- |
| Any present frontmatter | Parse YAML | Error for an unclosed delimiter, malformed YAML, or non-mapping root. |
| `docs/issues/drafts/` and `docs/issues/open/` v1 issue/EPIC specs | Strict | Error for profile, scalar, allowed-value, and reference-syntax violations. |
| Draft/open issue/EPIC records without `schema-version: 1` | Compatibility | Warning for legacy shape; no rewrite is required by this contract. |
| `docs/issues/closed/` | Advisory | Warning for profile or reference incompatibility; syntax remains an error. |
| Unknown repository-owned class | Permissive | Preserve fields; emit no profile diagnostic. |
| Externally governed skill/agent top-level metadata | Extension-only | Validate only `metadata.semantic-links`; unsupported external keys receive no diagnostic. |

`x-` experimental fields in strict profiles receive an `experimental-field` warning. Unknown
unprefixed fields receive an error in strict profiles and are preserved without a profile diagnostic
elsewhere. The validator is read-only. Historical records remain evidence and are never rewritten
solely to remove advisory diagnostics.

## Generated Schema Boundary

Issue #2266 generates JSON Schema Draft 2020-12 from the canonical Rust v1 types. The tracked
schema is for editor configuration, agent discovery, and non-Rust structural consumers. Generation
and drift verification must be deterministic and offline.

JSON Schema expresses field presence, primitive types, enums, nullability, list/mapping shape, and
the provisional string patterns. Rust validation retains YAML extraction, exact scalar behavior,
cross-field rules, profile dispatch, location/lifecycle consistency, repository existence, skill
discovery, and review-finding resolution. The generated schema must document that boundary rather
than pretending those invariants are encoded in JSON Schema.

## OKF V0.2 Disposition

Torrust selectively adopts compatible practices without claiming OKF conformance. Both formats use
human-readable Markdown and YAML frontmatter, preserve unknown data outside strict profiles, and
benefit from explicit versioning and structured metadata. Torrust does not adopt an OKF profile or
projection in v1 because the material contracts differ:

| OKF v0.2 | Torrust v1 decision |
| --- | --- |
| Requires `type` on every concept except index/log conventions. | Uses optional `doc-type`; requires it only for strict issue/EPIC profiles. |
| Uses path-derived concept identity and reserves `index.md` and `log.md`. | Uses stable GitHub issue identifiers and existing README/index conventions; reserves no filenames. |
| Lifecycle is `draft`, `stable`, or `deprecated`. | Uses operational issue lifecycle values. |
| Requires explicit-offset ISO 8601 timestamps for timestamp fields. | Retains the established quoted UTC-minute representation for v1. |
| Standard Markdown links are untyped and may be broken. | Leaves ordinary links to Lychee and validates a narrow typed frontmatter union separately. |
| Defines provenance, trust, freshness, and attested-computation families. | Defers those concepts until a Torrust document profile needs them. |

The migration consequence is deliberate: no existing document must be transformed to look like
OKF, and no external consumer should assume that the repository is an OKF bundle. Future profiles
may selectively adopt additional OKF-compatible fields only through a versioned Torrust contract.

## Deferred Profiles

ADRs, Agent Skills, agent profiles, refactor plans, review records, security analysis, research,
manual verification evidence, and other evidence records remain known but non-strict document
classes. Their profile definitions belong to the EPIC's later profile and convention-split work.
