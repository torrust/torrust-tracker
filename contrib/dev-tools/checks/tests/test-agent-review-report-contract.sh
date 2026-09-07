#!/usr/bin/env bash
# Structural contract tests for the independent agent-review report workflow.
#
# Run manually after modifying the template or reviewer profiles:
#   bash contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
# This test verifies tracked documentation/profile contracts, not autonomous agent behavior.

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEMPLATE="${PROJECT_ROOT}/docs/templates/AGENT-REVIEW-REPORTS.md"
COMPLEXITY_AUDITOR="${PROJECT_ROOT}/.github/agents/complexity-auditor.agent.md"
TASK_REVIEWER="${PROJECT_ROOT}/.github/agents/task-reviewer.agent.md"
PR_REVIEWER="${PROJECT_ROOT}/.github/agents/pr-reviewer.agent.md"
COPILOT_HANDLER="${PROJECT_ROOT}/.github/agents/copilot-suggestions-handler.agent.md"

require_text() {
    local file_path=$1
    local expected_text=$2

    if ! grep -F -q -- "${expected_text}" "${file_path}"; then
        printf 'Expected %s to contain: %s\n' "${file_path}" "${expected_text}" >&2
        return 1
    fi
}

require_wrapped_text() {
    local file_path=$1
    local expected_text=$2

    if ! tr '\n' ' ' <"${file_path}" | grep -F -q -- "${expected_text}"; then
        printf 'Expected %s to contain wrapped text: %s\n' "${file_path}" "${expected_text}" >&2
        return 1
    fi
}

require_absent_text() {
    local file_path=$1
    local unexpected_text=$2

    if grep -F -q -- "${unexpected_text}" "${file_path}"; then
        printf 'Expected %s not to contain: %s\n' "${file_path}" "${unexpected_text}" >&2
        return 1
    fi
}

it_should_define_the_reusable_report_template_contract() {
    require_text "${TEMPLATE}" '---'
    require_text "${TEMPLATE}" 'Append one completed independent-review entry at a time.'
    require_text "${TEMPLATE}" '### {YYYY-MM-DD HH:MM UTC} - {Reviewer}'
    require_text "${TEMPLATE}" '- Invocation scope:'
    require_text "${TEMPLATE}" '- Inputs:'
    require_text "${TEMPLATE}" '- Evidence:'
    require_text "${TEMPLATE}" '- Findings:'
    require_text "${TEMPLATE}" '- Verdict:'
    require_text "${TEMPLATE}" '- Follow-up actions:'
}

it_should_grant_each_independent_reviewer_edit_access() {
    require_text "${COMPLEXITY_AUDITOR}" 'tools: [execute, read, search, edit]'
    require_text "${TASK_REVIEWER}" 'tools: [execute, read, search, edit, todo, agent]'
    require_text "${PR_REVIEWER}" 'tools: [execute, read, search, edit, todo, agent]'
}

it_should_define_the_shared_create_append_or_skip_policy() {
    local reviewer

    for reviewer in "${COMPLEXITY_AUDITOR}" "${TASK_REVIEWER}" "${PR_REVIEWER}"; do
        # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
        require_wrapped_text "${reviewer}" 'When the caller supplies an existing folder-style issue specification path whose primary file is `ISSUE.md` or `EPIC.md`, persist this independent review before returning the caller-facing verdict.'
        # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
        require_wrapped_text "${reviewer}" 'create `agent-review-reports.md` from `docs/templates/AGENT-REVIEW-REPORTS.md` when absent; otherwise append one complete entry after the final existing report entry.'
        require_wrapped_text "${reviewer}" 'Issue-local report skipped: no folder-style issue specification was supplied.'
    done
}

it_should_keep_commit_authority_with_the_caller_and_committer() {
    local reviewer

    require_absent_text "${COMPLEXITY_AUDITOR}" 'git commit'
    require_text "${COMPLEXITY_AUDITOR}" 'Do not alter the reviewed implementation or invoke Committer.'

    for reviewer in "${TASK_REVIEWER}" "${PR_REVIEWER}"; do
        require_wrapped_text "${reviewer}" 'Do not invoke Committer or self-commit a report. When the report changes a branch or pull-request worktree, the caller requests Committer to include it in the coherent reviewed change set or create a focused documentation commit.'
    done
}

it_should_keep_copilot_tracking_separate_from_independent_reviews() {
    require_absent_text "${COPILOT_HANDLER}" 'agent-review-reports.md'
    require_text "${COPILOT_HANDLER}" 'docs/copilot-pr-reviews/pr-<PR_NUMBER>-copilot-suggestions.md'
}

it_should_define_the_reusable_report_template_contract
it_should_grant_each_independent_reviewer_edit_access
it_should_define_the_shared_create_append_or_skip_policy
it_should_keep_commit_authority_with_the_caller_and_committer
it_should_keep_copilot_tracking_separate_from_independent_reviews

printf 'All agent review report contract tests passed.\n'
