#!/usr/bin/env bash
# Integration tests for the tracked-file UTF-8 check.
#
# Sensor: ../check-utf8-files.sh
# Run manually or through CI:
#   bash contrib/dev-tools/checks/tests/test-check-utf8-files.sh

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEST_DIRECTORY=$(mktemp -d "${TMPDIR:-/tmp}/test-check-utf8-files.XXXXXX")
trap 'rm -rf "${TEST_DIRECTORY}"' EXIT

mkdir -p "${TEST_DIRECTORY}/contrib/dev-tools/checks"
cp "${PROJECT_ROOT}/contrib/dev-tools/checks/check-utf8-files.sh" "${TEST_DIRECTORY}/contrib/dev-tools/checks/"
cd "${TEST_DIRECTORY}"
git init -q
check_script="${TEST_DIRECTORY}/contrib/dev-tools/checks/check-utf8-files.sh"
output="${TEST_DIRECTORY}/output.txt"

expect_pass() {
    bash "${check_script}" >"${output}" 2>&1
    grep -Fq 'UTF-8 check passed:' "${output}"
    printf 'PASS: %s\n' "$1"
}

expect_failure() {
    local status=0
    bash "${check_script}" >"${output}" 2>&1 || status=$?
    [[ ${status} -eq 1 ]]
    grep -Fq "'text.txt': invalid UTF-8 at byte offset 3" "${output}"
    grep -Fq 'Fix the encoding, or mark the path binary in .gitattributes' "${output}"
    printf 'PASS: %s\n' "$1"
}

printf 'plain text\n' >text.txt
git add text.txt
expect_pass 'clean tree'

printf 'abc\351\n' >text.txt
git add text.txt
expect_failure 'Latin-1 reports path and zero-based byte offset'

printf 'text.txt binary\n' >.gitattributes
git add .gitattributes
expect_pass 'explicit binary declaration'

printf 'text.txt -text\n' >.gitattributes
git add .gitattributes
expect_failure '-text alone is still checked'

printf 'text.txt binary\n' >.gitattributes
expect_failure 'unstaged binary declaration cannot hide staged error'

printf 'plain text\n' >text.txt
git add text.txt
printf '\377' >untracked.txt
printf '\377' >ignored.txt
printf 'ignored.txt\n' >.gitignore
git add .gitignore
expect_pass 'untracked and ignored invalid files excluded'

touch empty.txt
ln -s untracked.txt link
ln -s missing.txt dangling
git add empty.txt link dangling
expect_pass 'empty file and symlinks including dangling link'
grep -Fq '4 files checked, 2 skipped.' "${output}"

printf '\377' >text.txt
expect_pass 'unstaged invalid content excluded'
git add text.txt
printf 'valid working copy\n' >text.txt
status=0
bash "${check_script}" >"${output}" 2>&1 || status=$?
[[ ${status} -eq 1 ]]
grep -Fq "'text.txt': invalid UTF-8 at byte offset 0" "${output}"
printf 'PASS: staged invalid content cannot be hidden by working copy\n'
git add text.txt

# A gitlink's commit object need not exist.
git update-index --add --cacheinfo "160000,1111111111111111111111111111111111111111,submodule"
expect_pass 'submodule entry skipped'
grep -Fq '4 files checked, 3 skipped.' "${output}"

odd_name=$'space tab\tnewline\n.txt'
printf '\377' >"${odd_name}"
git add "${odd_name}"
status=0
bash "${check_script}" >"${output}" 2>&1 || status=$?
[[ ${status} -eq 1 ]]
grep -Fq "'space tab\\tnewline\\n.txt': invalid UTF-8 at byte offset 0" "${output}"
printf 'PASS: NUL-safe path handling and single-line diagnostics\n'
git rm -q -f -- "${odd_name}"

cd contrib/dev-tools
expect_pass 'invocation from a checkout subdirectory'
printf 'All UTF-8 check tests passed.\n'
