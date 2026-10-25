#!/usr/bin/env bash
# Check that every tracked regular file is UTF-8 unless explicitly marked binary.
#
# Tests: tests/test-check-utf8-files.sh (also run by CI)
#   bash contrib/dev-tools/checks/tests/test-check-utf8-files.sh
#
# Usage:
#   ./contrib/dev-tools/checks/check-utf8-files.sh
#
# Requires Git and Python 3 (standard library only), as used by the review and
# merge scripts. CI's Ubuntu runner provides both; contributors need them on PATH.
# Checks index contents and index attributes, so unstaged edits cannot hide a
# staged encoding error. Symlinks and submodules are skipped, never followed.
# Offsets are zero-based bytes. Exit: 0 success, 1 invalid UTF-8, 2 usage/Git error.
# No options are required. Run from any directory within the checkout.

set -uo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
PROJECT_ROOT=$(cd -- "${SCRIPT_DIR}/../../.." && pwd)

if [[ $# -ne 0 ]]; then
    printf 'Usage: %s (no arguments)\n' "${BASH_SOURCE[0]}" >&2
    exit 2
fi

exec python3 - "${PROJECT_ROOT}" <<'PY'
import io
import subprocess
import sys


def git(*args, data=None):
    return subprocess.run(
        ["git", "-C", sys.argv[1], *args], input=data,
        stdout=subprocess.PIPE, check=True,
    ).stdout


def check():
    entries = []
    for record in git("ls-files", "-z", "--stage").split(b"\0"):
        if not record:
            continue
        metadata, path = record.split(b"\t", 1)
        mode, oid, stage = metadata.split()
        if stage != b"0":
            raise ValueError("resolve index conflicts before checking UTF-8")
        entries.append((mode, oid, path))

    paths = b"".join(path + b"\0" for _, _, path in entries)
    fields = git("check-attr", "--cached", "-z", "binary", "--stdin", data=paths).split(b"\0")
    attributes = dict(zip(fields[0:-1:3], fields[2:-1:3]))
    regular = [
        (oid, path) for mode, oid, path in entries
        if mode not in (b"120000", b"160000") and attributes[path] != b"set"
    ]
    # Batch object reads avoid a Git process per file and check precisely the index.
    objects = io.BytesIO(git("cat-file", "--batch", data=b"".join(oid + b"\n" for oid, _ in regular)))
    failed = False
    for oid, path in regular:
        header = objects.readline().split()
        if len(header) != 3 or header[:2] != [oid, b"blob"]:
            raise ValueError("cannot read an indexed blob")
        size = int(header[2])
        content = objects.read(size)
        if len(content) != size or objects.read(1) != b"\n":
            raise ValueError("incomplete indexed blob")
        try:
            content.decode("utf-8")
        except UnicodeDecodeError as error:
            # repr keeps tabs, newlines and undecodable filenames on a single line.
            name = repr(path.decode("utf-8", errors="backslashreplace"))
            print(f"{name}: invalid UTF-8 at byte offset {error.start}", file=sys.stderr)
            failed = True
    if failed:
        print("Fix the encoding, or mark the path binary in .gitattributes; stage the changes.", file=sys.stderr)
        return 1
    print(f"UTF-8 check passed: {len(regular)} files checked, {len(entries) - len(regular)} skipped.")
    return 0


try:
    sys.exit(check())
except (OSError, ValueError, subprocess.CalledProcessError) as error:
    print(f"Error: {error}", file=sys.stderr)
    sys.exit(2)
PY
