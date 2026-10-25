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

cd "${TEST_DIRECTORY}"
printf 'abc\351\n' >text.txt
git add text.txt
printf '* binary\n' >.git/info/attributes
expect_failure 'local info/attributes cannot exempt invalid staged text'
rm .git/info/attributes

printf 'plain text\n' >text.txt
printf '\211PNG\r\n' >asset.png
printf '*.png binary\n' >.gitattributes
git add text.txt asset.png .gitattributes
printf '*.png -binary\n' >.git/info/attributes
expect_pass 'local info/attributes cannot revoke staged binary declaration'
rm .git/info/attributes

printf '*.png -binary\n' >global-attributes
printf '[core]\nattributesFile = "%s/global-attributes"\n' "${TEST_DIRECTORY}" >global-config
GIT_CONFIG_GLOBAL="${TEST_DIRECTORY}/global-config" expect_pass 'global override cannot revoke staged binary declaration'
printf 'abc\351\n' >text.txt
git add text.txt
printf '* binary\n' >global-attributes
GIT_CONFIG_GLOBAL="${TEST_DIRECTORY}/global-config" expect_failure 'global attributes cannot exempt invalid staged text'
GIT_CONFIG_NOSYSTEM=0 GIT_CONFIG_SYSTEM="${TEST_DIRECTORY}/global-config" \
    expect_failure 'system configuration cannot inject an attributes file'
GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.attributesFile GIT_CONFIG_VALUE_0="${TEST_DIRECTORY}/global-attributes" \
    expect_failure 'injected configuration cannot exempt invalid staged text'

# The system attributes file has no portable path override. It lives outside this
# fixture; the sensor explicitly sets GIT_ATTR_NOSYSTEM=1 for attribute lookup.
printf 'plain text\n' >text.txt
mkdir -p nested
printf '*.bin binary\n' >nested/.gitattributes
printf '\377' >nested/asset.bin
printf '[attr]image binary\n*.png image\n' >.gitattributes
git add text.txt .gitattributes nested
expect_pass 'indexed custom macros and nested declarations are preserved'

# Construct a fixture commit directly as an object; no signing command or
# signing configuration is involved. Track the sensor for the other checkouts.
git add contrib/dev-tools/checks/check-utf8-files.sh
tree=$(git write-tree)
commit=$(printf 'tree %s\nauthor fixture <fixture@example.invalid> 1 +0000\ncommitter fixture <fixture@example.invalid> 1 +0000\n\nfixture\n' "${tree}" | git hash-object -t commit -w --stdin)
git update-ref HEAD "${commit}"
git worktree add --quiet --detach "${TEST_DIRECTORY}/linked" HEAD
printf '* binary\n' >.git/info/attributes
printf 'abc\351\n' >linked/text.txt
git -C linked add text.txt
check_script="${TEST_DIRECTORY}/linked/contrib/dev-tools/checks/check-utf8-files.sh"
expect_failure 'linked worktree ignores common info/attributes and uses its own index'
printf 'plain text\n' >linked/text.txt
git -C linked add text.txt
expect_pass 'linked worktree preserves indexed binary declarations'
rm .git/info/attributes

# Local clones exercise an alternate object store without any network access.
git clone --quiet --shared --no-checkout "${TEST_DIRECTORY}" "${TEST_DIRECTORY}/shared"
git -C shared reset --quiet --hard HEAD
check_script="${TEST_DIRECTORY}/shared/contrib/dev-tools/checks/check-utf8-files.sh"
expect_pass 'alternate object store preserves indexed declarations'

git -c protocol.file.allow=always submodule add --quiet --force "${TEST_DIRECTORY}" module
printf '* binary\n' >.git/modules/module/info/attributes
printf 'abc\351\n' >module/text.txt
git -C module add text.txt
check_script="${TEST_DIRECTORY}/module/contrib/dev-tools/checks/check-utf8-files.sh"
expect_failure 'submodule checkout ignores its private info/attributes'
printf 'plain text\n' >module/text.txt
git -C module add text.txt
expect_pass 'submodule checkout preserves indexed declarations'

# Observe the actual attribute process: the compiled-in system attributes path
# cannot be replaced portably by a fixture, but its disable switch is testable.
mkdir -p "${TEST_DIRECTORY}/bin"
cat >"${TEST_DIRECTORY}/bin/git" <<'EOF'
#!/usr/bin/env bash
for argument in "$@"; do
    if [[ "${argument}" == check-attr ]]; then
        [[ "${GIT_ATTR_NOSYSTEM:-}" == 1 ]] || exit 97
        printf 'check-attr\n' >>"${UTF8_QUERY_LOG}"
    fi
done
exec "${UTF8_REAL_GIT}" "$@"
EOF
chmod +x "${TEST_DIRECTORY}/bin/git"
real_git=$(command -v git)
PATH="${TEST_DIRECTORY}/bin:${PATH}" UTF8_REAL_GIT="${real_git}" \
    UTF8_QUERY_LOG="${TEST_DIRECTORY}/attribute-queries" expect_pass 'system attributes disabled for the batched query'
[[ $(wc -l <"${TEST_DIRECTORY}/attribute-queries") -eq 1 ]]
printf 'PASS: exactly one batched attribute query\n'

status=0
bash "${check_script}" --unknown >"${output}" 2>&1 || status=$?
[[ ${status} -eq 2 ]]
printf 'PASS: usage error remains exit 2\n'
printf 'invalid index' >bad-index
status=0
GIT_INDEX_FILE="${TEST_DIRECTORY}/bad-index" bash "${check_script}" >"${output}" 2>&1 || status=$?
[[ ${status} -eq 2 ]]
printf 'PASS: Git error remains exit 2\n'
printf 'All UTF-8 check tests passed.\n'
