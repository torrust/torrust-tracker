#!/usr/bin/env python3
"""Compare the arrows of the workspace dependency diagram with `cargo metadata`.

Run from the repository root. Prints the arrows drawn but declared by no manifest (or only as
a test-only `optional (testing)` edge) and the declared edges that are not drawn.
"""
import json
import re
import subprocess

DIAGRAM = "docs/media/packages/dependencies-workspace-packages.md"
meta = json.loads(subprocess.run(["cargo", "metadata", "--format-version", "1", "--no-deps"], capture_output=True, text=True, check=True).stdout)
text = open(DIAGRAM).read()
nodes = {m.group(1): m.group(2) for m in re.finditer(r'^\s+([A-Za-z0-9_-]+)\["([^"<]+)', text, re.M)}
edges = set(re.findall(r"^\s+([A-Za-z0-9_-]+) --> ([A-Za-z0-9_-]+)\s*$", text, re.M))


def node_id(package_name):
    for node, label in nodes.items():
        if package_name in (label, "torrust-tracker-" + label, "torrust-" + label):
            return node
    return None


declared = set()
for package in meta["packages"]:
    source = node_id(package["name"])
    if not source:
        continue
    testing = {f[4:] for f in package["features"].get("testing", []) if f.startswith("dep:")}
    other = {(f[4:] if f.startswith("dep:") else f) for k, v in package["features"].items() if k != "testing" for f in v}
    for dep in package["dependencies"]:
        target = node_id(dep["name"])
        if dep["kind"] is not None or not target or target == source:
            continue
        if dep["optional"] and dep["name"] in testing and dep["name"] not in other:
            continue
        declared.add((source, target))

print(f"nodes {len(nodes)}, arrows drawn {len(edges)}, declared non-test-only edges {len(declared)}")
print("drawn but not declared:", sorted(edges - declared))
print("declared but not drawn:", sorted(declared - edges))
