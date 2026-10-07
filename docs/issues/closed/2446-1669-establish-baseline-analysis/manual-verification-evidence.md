---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2446-1669-establish-baseline-analysis/ISSUE.md
last-updated-utc: 2026-10-07 14:54
---

<!-- cspell:ignore cdir finditer isdir listdir startswith -->

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

- Date and time (UTC): MV4 at 2026-10-06 16:47; MV3 at 2026-10-07 08:32; MV1 and MV2 at
  2026-10-07 09:06 (rerun to record every command that prints an observed line)
- Artifact under test: branch `2446-1669-establish-baseline-analysis` from
  "docs(issues): [#2446] regenerate the coupling report after rebasing onto develop" onwards;
  the later commits change no generated report section, manifest or README
- Operating system / environment: Linux 7.0.0-34-generic; stable Rust toolchain
  (`rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478 2026-08-27)`)
- Prerequisites and setup performed: clean working tree (`git status --short` printed
  nothing). The scenarios were first run at 12:55 and 16:17 UTC on 2026-10-06 before the
  rebase, with the same results.

## Verification Processes

### MV1 - The new report covers the current workspace

- Goal: the 2026-10-06 coupling report lists exactly the workspace members from `cargo metadata`.
- Initial state: `workspace-coupling-report-2026-10-06.md` committed.
- Status: `DONE`

#### Steps Performed

Rerun on 2026-10-07 09:06 UTC (stable Rust toolchain, `rustc 1.99.0`). Every observed line
below is printed by one of these commands, in order:

```sh
R=docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
# Workspace member names, from cargo metadata
cargo metadata --no-deps --format-version 1 \
  | python3 -c 'import json,sys;[print(p["name"]) for p in json.load(sys.stdin)["packages"]]' \
  | sort > .tmp/mv1-metadata.txt
# Packages in the report: leaf list plus detail sections
{ sed -n '/^## Packages with no workspace dependencies/,/^## Package coupling/p' $R \
    | grep -oP '^- `\K[^`]+'; grep -oP '^### `\K[^`]+' $R; } | sort > .tmp/mv1-report.txt
wc -l < .tmp/mv1-metadata.txt                            # line 1
wc -l < .tmp/mv1-report.txt                              # line 2
diff .tmp/mv1-metadata.txt .tmp/mv1-report.txt && echo identical   # line 3
sed -n '/^## Observations/q;p' $R | grep -c "rest-api-core"        # line 4
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

Rerun on 2026-10-07 09:06 UTC (stable Rust toolchain, `rustc 1.99.0`), with `R` as in MV1.
Every observed line is printed by one of these commands:

```sh
cargo run -q -p workspace-coupling -- /tmp/test-report-2446.md > /dev/null 2>&1; echo "tool exit=$?"
diff <(sed -n '/^# Workspace Coupling Report/,/^## Observations/p' $R) \
     <(sed -n '/^# Workspace Coupling Report/,/^## Observations/p' /tmp/test-report-2446.md)
```

#### Observed Result

```text
tool exit=0
3c3
< Generated: 2026-10-06 16:35 UTC
---
> Generated: 2026-10-07 09:06 UTC
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

Rerun on 2026-10-07 08:32 UTC on the same head, after the layer-placement fix, with a single
self-contained script (stable Rust toolchain, `rustc 1.99.0`). It builds the expected edge set
from `cargo metadata --no-deps` (for each `torrust*` package, every dependency with
`kind == null` whose name starts with `torrust`, mapped to the diagram's node IDs), extracts the
diagram's edges, excluding `server-lib --> net-prim` (an edge between two external crates, drawn
for context), and compares the two sets:

```python
import json
import re
import subprocess

NODE = {
    "torrust-tracker": "tracker",
    "torrust-tracker-axum-http-server": "axum-http",
    "torrust-tracker-axum-rest-api-server": "axum-rest",
    "torrust-tracker-axum-health-check-api-server": "axum-health",
    "torrust-tracker-udp-server": "udp-srv",
    "torrust-tracker-axum-server": "axum-base",
    "torrust-tracker-core": "tracker-core",
    "torrust-tracker-http-core": "http-core",
    "torrust-tracker-udp-core": "udp-core",
    "torrust-tracker-rest-api-application": "rest-app",
    "torrust-tracker-rest-api-runtime-adapter": "rest-adapter",
    "torrust-tracker-http-protocol": "http-proto",
    "torrust-tracker-udp-protocol": "udp-proto",
    "torrust-tracker-rest-api-protocol": "rest-proto",
    "torrust-tracker-swarm-coordination-registry": "swarm",
    "torrust-tracker-configuration": "config",
    "torrust-tracker-primitives": "primitives",
    "torrust-tracker-events": "events",
    "torrust-tracker-client-lib": "client-lib",
    "torrust-tracker-client": "tracker-client",
    "torrust-tracker-rest-api-client": "rest-client",
    "torrust-tracker-test-helpers": "test-helpers",
    "torrust-tracker-torrent-repository-benchmarking": "torrent-bench",
    "torrust-tracker-persistence-benchmark": "persist-bench",
    "torrust-tracker-e2e-tools": "e2e-tools",
    "torrust-clock": "clock",
    "torrust-info-hash": "info-hash",
    "torrust-located-error": "located-err",
    "torrust-metrics": "metrics",
    "torrust-net-primitives": "net-prim",
    "torrust-peer-id": "peer-id",
    "torrust-bencode": "bencode",
    "torrust-server-lib": "server-lib",
}

metadata = json.loads(
    subprocess.run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        capture_output=True, check=True, text=True,
    ).stdout
)
expected = {
    f"{NODE[p['name']]} --> {NODE[d['name']]}"
    for p in metadata["packages"]
    if p["name"].startswith("torrust")
    for d in p["dependencies"]
    if d["kind"] is None and d["name"].startswith("torrust")
}

diagram = open("docs/media/packages/dependencies-workspace-packages.md").read()
drawn = {
    m.group(1)
    for m in re.finditer(r"^    ([a-z0-9-]+ --> [a-z0-9-]+)$", diagram, re.M)
    if not m.group(1).startswith("server-lib --> ")
}

print("expected edges:", len(expected))
print("drawn edges:", len(drawn))
print("missing from diagram:", sorted(expected - drawn))
print("extra in diagram:", sorted(drawn - expected))
```

#### Observed Result

```text
expected edges: 153
drawn edges: 153
missing from diagram: []
extra in diagram: []
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

After rebasing onto `develop` (76 new commits, none touching a manifest), the pre-rebase
report no longer reproduced exactly: a few import paths changed in six edges, with no
dependency edge added or removed. The report was regenerated ("docs(issues): [#2446]
regenerate the coupling report after rebasing onto develop") and MV1 to MV4 rerun above.
