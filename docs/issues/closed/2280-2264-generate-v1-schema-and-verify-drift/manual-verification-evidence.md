---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2280-2264-generate-v1-schema-and-verify-drift/ISSUE.md
last-updated-utc: "2026-09-24 09:36"
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

### V3 - Re-verification Against the Final Branch State

- Goal: Repeat M1-M3 against the finished implementation, which changed the drift wording, added
  exit code `2`, made artifact replacement atomic, and regenerated the artifact for AC5, none of
  which V1-V2 exercised.
- Initial state: HEAD `4c58b5a9` (now `412facad` after the rebase onto `torrust/develop`; the crate,
  schema, and issue-folder trees are identical) on `2026-09-23 15:22 UTC`; clean worktree; tracked
  artifact SHA-256 `88540c823de57134925e875ab50acbd107e98c10dbc59f92918e3958ca6fe8c5`.
- Status: `DONE`

#### Steps Performed

1. M1: ran `cargo run --offline --quiet --package frontmatter-validator --bin frontmatter-schema -- generate`
   and recorded the exit code and the tracked artifact's SHA-256 afterwards.
2. M2: ran the same binary with `check` and confirmed `git status` reported no change under
   `docs/schemas/`.
3. M3: copied the artifact to `.tmp/frontmatter-v1.schema.json.manual-copy`, appended one newline,
   and ran `check --artifact .tmp/frontmatter-v1.schema.json.manual-copy`.
4. Ran `frontmatter-schema bogus` to exercise the invalid-invocation exit code.
5. Ran `check` with stderr discarded and counted stdout bytes.
6. Removed the disposable copy.

#### Observed Result

```text
--- generate ---
exit=0
--- check ---
exit=0
sha_after=88540c823de57134925e875ab50acbd107e98c10dbc59f92918e3958ca6fe8c5
--- drift copy ---
frontmatter-schema: .tmp/frontmatter-v1.schema.json.manual-copy differs from the deterministic v1 schema output; run the documented generator with `--artifact <path>` and the artifact path above
exit=1
--- usage error ---
frontmatter-schema: usage: frontmatter-schema <generate|check> [--artifact <path>]
exit=2
--- stdout must be empty ---
0
```

#### Conclusion

Regeneration is idempotent (identical SHA-256 before and after, no tracked change). Drift on the
disposable copy exits `1` with the shipped shell-safe instruction, which no longer embeds the caller
path in a command. An unknown action exits `2` with the usage line. Stdout stays empty in every
case, matching the `no-stdout-result` CLI output class. The V2 output above is historical; its
quoted hint reflects the binary before the schema-command refactor plan.

## Failures and Follow-up

No manual scenario failed or remained blocked.
