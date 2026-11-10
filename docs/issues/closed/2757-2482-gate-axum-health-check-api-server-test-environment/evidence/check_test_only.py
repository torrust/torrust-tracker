#!/usr/bin/env python3
"""List what could still make a workspace package carry test-only code in a normal build.

Run from the repository root. Prints, from `cargo metadata`, every normal dependency on a
test-ish crate, every package with a `testing` feature (and whether `testing` is in its `default`
chain), every normal edge that enables `testing` on a workspace dependency, and, from the sources,
every `pub mod testing`/`environment`-style declaration with the line above it.
"""
import json
import pathlib
import re
import subprocess

meta = json.loads(subprocess.run(["cargo", "metadata", "--format-version", "1", "--no-deps"], capture_output=True, text=True, check=True).stdout)
names = {p["name"] for p in meta["packages"]}
TEST_ONLY_CRATES = {"torrust-tracker-test-helpers", "mockall", "mockito", "rstest", "testcontainers", "tempfile", "wiremock"}
for p in meta["packages"]:
    for d in p["dependencies"]:
        if d["kind"] is None and d["name"] in TEST_ONLY_CRATES:
            print("normal test-ish dependency:", p["name"], "->", d["name"], "(optional)" if d["optional"] else "")
    if "testing" in p["features"]:
        print("testing feature:", p["name"], p["features"]["testing"], "IN DEFAULT" if "testing" in p["features"].get("default", []) else "not in default")
    for d in p["dependencies"]:
        if d["kind"] is None and "testing" in d["features"] and d["name"] in names:
            print("normal edge enabling testing on a workspace dependency:", p["name"], "->", d["name"])
pattern = re.compile(r"^\s*pub mod (testing|test_helpers|test_support|environment|mocks?|fixtures)\b")
for path in sorted(pathlib.Path(".").glob("packages/*/src/**/*.rs")) + sorted(pathlib.Path("src").glob("**/*.rs")):
    lines = path.read_text().splitlines()
    for i, line in enumerate(lines):
        if pattern.match(line):
            print(f"{path}:{i + 1}: {line.strip()}   <- previous line: {lines[i - 1].strip() if i else ''}")
