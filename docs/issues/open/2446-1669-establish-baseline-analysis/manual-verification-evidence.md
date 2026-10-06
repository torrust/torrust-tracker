---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2446-1669-establish-baseline-analysis/ISSUE.md
last-updated-utc: 2026-10-06 16:17
---

<!-- cspell:ignore cdir isdir listdir startswith -->

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. The commands, the helper script and the
relevant output are recorded inline below.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-06 16:17
- Artifact under test: branch `2446-1669-establish-baseline-analysis` up to
  "docs(issues): [#2446] link coupling findings to their draft subissues", on top of
  `develop` after the merge of PR #2447
- Operating system / environment: Linux 7.0.0-34-generic; stable Rust toolchain
  (`rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478 2026-08-27)`)
- Prerequisites and setup performed: clean working tree (`git status --short` printed
  nothing). MV1, MV2 and MV4 were also run before their task commits at 12:55 UTC with the
  same results.

## Verification Processes

### MV1 - The new report covers the current workspace

- Goal: the 2026-10-06 coupling report lists exactly the workspace members from `cargo metadata`.
- Initial state: `workspace-coupling-report-2026-10-06.md` committed.
- Status: `DONE`

#### Steps Performed

1. List the workspace member names (stable Rust toolchain):

   ```sh
   cargo metadata --no-deps --format-version 1 \
     | python3 -c 'import json,sys;[print(p["name"]) for p in json.load(sys.stdin)["packages"]]' \
     | sort > .tmp/mv1-metadata.txt
   ```

2. List the packages in the report (leaf list plus detail sections):

   ```sh
   R=docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
   { sed -n '/^## Packages with no workspace dependencies/,/^## Package coupling/p' $R \
       | grep -oP '^- `\K[^`]+'; grep -oP '^### `\K[^`]+' $R; } | sort > .tmp/mv1-report.txt
   ```

3. Compare them, and count `rest-api-core` in the generated part (before `## Observations`):

   ```sh
   diff .tmp/mv1-metadata.txt .tmp/mv1-report.txt && echo identical
   sed -n '/^## Observations/q;p' $R | grep -c "rest-api-core"
   ```

#### Observed Result

```text
31
31
identical
0
```

Both lists have 31 entries, including `torrust-tracker-rest-api-protocol`,
`torrust-tracker-rest-api-application`, `torrust-tracker-rest-api-runtime-adapter`,
`torrust-tracker-e2e-tools` and `torrust-tracker-persistence-benchmark`.

#### Conclusion

Met. The report covers the current workspace and its generated part does not mention
`rest-api-core`. The two mentions in the Observations section are historical (they describe
its removal by #1938).

### MV2 - The committed report is reproducible

- Goal: regenerating the report changes nothing but the timestamp in the generated part.
- Initial state: clean working tree.
- Status: `DONE`

#### Steps Performed

1. Regenerate to a scratch file (stable Rust toolchain):

   ```sh
   cargo run -q -p workspace-coupling -- /tmp/test-report-2446.md
   ```

2. Diff the generated part (from the title to `## Observations`) of both files:

   ```sh
   diff <(sed -n '/^# Workspace Coupling Report/,/^## Observations/p' $R) \
        <(sed -n '/^# Workspace Coupling Report/,/^## Observations/p' /tmp/test-report-2446.md)
   ```

#### Observed Result

```text
tool exit=0
3c3
< Generated: 2026-10-06 12:34 UTC
---
> Generated: 2026-10-06 16:17 UTC
```

#### Conclusion

Met. Only the `Generated:` line differs; the frontmatter and the hand-written Observations
section are outside the compared range by design.

### MV3 - The dependency diagram matches the manifests

- Goal: every edge in the Mermaid diagram is a direct normal `torrust*` dependency, and every such
  dependency is drawn.
- Initial state: `docs/media/packages/dependencies-workspace-packages.md` committed.
- Status: `DONE`

#### Steps Performed

1. Generate the expected edge list from `cargo metadata --no-deps`: for each `torrust*`
   package, every dependency with `kind == null` whose name starts with `torrust`, mapped to
   the diagram's node IDs. Count the edges:

   ```sh
   cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys;p=json.load(sys.stdin)["packages"];print(sum(1 for x in p if x["name"].startswith("torrust") for d in x["dependencies"] if d["kind"] is None and d["name"].startswith("torrust")))'
   ```

2. Extract the diagram's edges, excluding `server-lib --> net-prim` (an edge between two
   external crates, drawn for context), and diff them with the expected list:

   ```sh
   D=docs/media/packages/dependencies-workspace-packages.md
   grep -oE "^    [a-z0-9-]+ --> [a-z0-9-]+$" $D | sed 's/^ *//' | grep -v "^server-lib --> " | sort
   ```

#### Observed Result

```text
153
153
identical
```

#### Conclusion

Met. The diagram has exactly the 153 normal `torrust*` dependency edges and no other edge
between packages.

### MV4 - The README audit matches the audited package set

- Goal: `readme-audit.md` has one row for each of the 25 audited packages, with the right crate
  name and README line count.
- Initial state: `readme-audit.md` committed.
- Status: `DONE`

#### Steps Performed

1. Run this check script from the repository root:

   ```python
   import re, os
   A = "docs/issues/open/1669-overhaul-packages/readme-audit.md"
   seen = []; section = None; bad = 0
   for line in open(A):
       if line.startswith("## "): section = line.strip()
       m = re.match(r"^\| (\(repository root\)|`[^`]+`) +\| `([^`]+)` +\| (\d+) +\| (\w+)", line)
       if not m: continue
       d, crate, n, rating = m.groups()
       if d == "(repository root)": cdir = "."
       elif "console" in section: cdir = "console/tracker-client"
       else: cdir = "packages/" + d.strip("`")
       path = cdir + "/README.md"
       real = sum(1 for _ in open(path)) if os.path.exists(path) else None
       name = re.search(r'^name = "([^"]+)"', open(cdir + "/Cargo.toml").read(), re.M).group(1)
       if real != int(n) or name != crate: bad += 1; print("MISMATCH", d, crate, n, real, name)
       seen.append(cdir)
   pk = sorted("packages/" + x for x in os.listdir("packages") if os.path.isdir("packages/" + x))
   print("audited rows:", len(seen), "mismatches:", bad)
   print("not audited:", [p for p in pk + ["console/tracker-client", "."] if p not in seen])
   ```

#### Observed Result

```text
audited rows: 25 mismatches: 0
not audited: []
```

#### Conclusion

Met. 25 rows, one per audited package; no removed package and no `contrib/dev-tools/` member.

## Failures and Follow-up

None. The first MV3 comparison used a regular expression without digits and silently skipped
the `e2e-tools --> tracker` edge on both sides (152 = 152); the corrected expression above
matches all 153 edges.
