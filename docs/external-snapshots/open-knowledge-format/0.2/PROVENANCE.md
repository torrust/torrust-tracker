---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/external-snapshots/README.md
    - docs/external-snapshots/open-knowledge-format/0.2/SPEC.md
    - docs/external-snapshots/open-knowledge-format/0.2/LICENSE.md
---

# Open Knowledge Format 0.2 Snapshot Provenance

This directory contains an unmodified local snapshot of the Open Knowledge Format (OKF) 0.2
specification for offline research and agent/human review. It is design input for Torrust's document
metadata work; it is not a Torrust convention and does not establish OKF conformance.

## Source

- Upstream repository: <https://github.com/GoogleCloudPlatform/open-knowledge-format>
- Canonical specification: <https://github.com/GoogleCloudPlatform/open-knowledge-format/blob/main/SPEC.md>
- Pinned commit: [`ad30107c31c06aec8a7d5636e0d1058118604e6f`](https://github.com/GoogleCloudPlatform/open-knowledge-format/commit/ad30107c31c06aec8a7d5636e0d1058118604e6f)
- Commit date: 2026-08-21 20:08:36 UTC
- Retrieved: 2026-09-18 UTC
- Upstream license: Apache License 2.0
- Upstream `NOTICE`: no `NOTICE` file existed at the pinned commit

## Integrity

| Local file | Upstream Git blob | SHA-256 |
| ---------- | ----------------- | ------- |
| [`SPEC.md`](SPEC.md) | `c06e3eede0c910d0ecf12524c34204156f8795ac` | `26aa5da029278939f914e578107242d9607d4f2dc5fe153272b82f9ed1030101` |
| [`LICENSE.md`](LICENSE.md) | `6b0b1270ff0ca8f03867efcd09ba6ddb6392b1e1` | `8c6db340475136df3c1201d458fa5755698eace76e510471ecc9d857d6083dac` |

Verify the local files with:

```bash
sha256sum \
  docs/external-snapshots/open-knowledge-format/0.2/SPEC.md \
  docs/external-snapshots/open-knowledge-format/0.2/LICENSE.md
```

## Preservation And Update Policy

`SPEC.md` and `LICENSE.md` are byte-identical upstream snapshots. Do not reformat, spell-correct, or
edit them locally. Repository-authored analysis belongs in the EPIC, its subissues, or this
provenance document. Their path-scoped linter exclusions and `.gitattributes` whitespace exception
exist only to preserve upstream bytes; they do not apply to this provenance file or other authored
documentation.

When evaluating a newer upstream revision:

1. Add it under a new version or revision directory.
2. Record its source revision, license, retrieval date, Git blob identities, and SHA-256 values.
3. Compare it with this snapshot and review effects on Torrust decisions.
4. Update repository references only after that review.
5. Keep this snapshot while specifications or decisions cite it as evidence.
