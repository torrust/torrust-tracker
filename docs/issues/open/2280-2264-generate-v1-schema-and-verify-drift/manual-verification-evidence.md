---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md
last-updated-utc: "2026-09-22 16:37"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-09-22 16:37
- Artifact under test: `docs/schemas/frontmatter-v1.schema.json`
- Operating system / environment: Linux workspace with cached Rust dependencies
- Prerequisites and setup performed: Cached Rust dependencies were available; all commands ran from
  the repository root with Cargo's `--offline` flag.

## Verification Processes

### V1 - Offline Regeneration and Clean Drift Check

- Goal: Confirm the documented generator and non-mutating drift check run without network access.
- Initial state: The generated schema artifact existed at its tracked documentation path.
- Status: `DONE`

#### Steps Performed

1. Ran `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate`.
2. Ran `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check`.

#### Observed Result

```text
Running `target/debug/frontmatter-schema generate`
Running `target/debug/frontmatter-schema check`
```

#### Conclusion

The generator wrote the canonical artifact without network access and the drift check succeeded without modifying it.

### V2 - Intentional Drift Detection and Restoration

- Goal: Confirm a changed generated artifact fails deterministically and regeneration restores it.
- Initial state: A disposable byte-for-byte copy of the generated artifact was created in ignored
  `.tmp/`; the tracked artifact's SHA-256 was recorded before the copy changed.
- Status: `DONE`

#### Steps Performed

1. Added one trailing newline to `.tmp/frontmatter-v1.schema.json.manual-copy` and ran
  `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- check --artifact .tmp/frontmatter-v1.schema.json.manual-copy`.
2. Compared the tracked artifact's before-and-after SHA-256 values and removed the disposable copy
  and captured output from `.tmp/`.

#### Observed Result

```text
frontmatter-schema: .tmp/frontmatter-v1.schema.json.manual-copy differs from the deterministic v1 schema output; run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate --artifact .tmp/frontmatter-v1.schema.json.manual-copy`
tracked schema SHA-256 unchanged: af561993078f67a968b99d30c8671494388a629760b762f3a5cf5edd87d3d6ea
```

#### Conclusion

The drift check failed with the documented deterministic instruction for the disposable copy. The
tracked artifact was never modified; its SHA-256 stayed unchanged.

## Failures and Follow-up

No manual scenario failed or remained blocked.
