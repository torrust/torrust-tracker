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
COPILOT_PROMPT="${PROJECT_ROOT}/.github/prompts/process-copilot-suggestions.prompt.md"
ORCHESTRATION="${PROJECT_ROOT}/docs/agents/orchestration.md"
COPILOT_REDIRECT="${PROJECT_ROOT}/.github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md"
FEEDBACK_REDIRECT="${PROJECT_ROOT}/.github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md"
FETCH_THREADS="${PROJECT_ROOT}/.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md"
RESOLVE_THREADS="${PROJECT_ROOT}/.github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md"
PROCESS_PR_REVIEW="${PROJECT_ROOT}/.github/skills/dev/pr-reviews/process-pr-review/SKILL.md"

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

require_yaml_frontmatter() {
    local file_path=$1

    if [[ $(sed -n '1p' "${file_path}") != '---' ]] || ! sed -n '2,/^---$/p' "${file_path}" | grep -Fx -q -- '---'; then
        printf 'Expected %s to begin with closed YAML frontmatter.\n' "${file_path}" >&2
        return 1
    fi
}

require_yaml_related_artifact() {
    local file_path=$1
    local expected_artifact=$2

    if ! python3 - "${file_path}" "${expected_artifact}" <<'PY'
import sys

import yaml

file_path, expected_artifact = sys.argv[1:]
with open(file_path, encoding="utf-8") as source_file:
    frontmatter = source_file.read().split("---", 2)[1]

related_artifacts = yaml.safe_load(frontmatter)["metadata"]["semantic-links"]["related-artifacts"]
sys.exit(expected_artifact not in related_artifacts)
PY
    then
        printf 'Expected %s YAML frontmatter to contain related artifact: %s\n' "${file_path}" "${expected_artifact}" >&2
        return 1
    fi
}

require_tools_declaration_with_edit() {
    local file_path=$1

    if ! awk '/^tools:/ { declaration = declaration $0 } declaration && /edit/ { found = 1 } /^---$/ && NR > 1 { exit } END { exit !found }' "${file_path}"; then
        printf 'Expected %s tools declaration to include edit.\n' "${file_path}" >&2
        return 1
    fi
}

it_should_define_the_reusable_report_template_contract() {
    require_yaml_frontmatter "${TEMPLATE}"
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
    require_tools_declaration_with_edit "${COMPLEXITY_AUDITOR}"
    require_tools_declaration_with_edit "${TASK_REVIEWER}"
    require_tools_declaration_with_edit "${PR_REVIEWER}"
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

it_should_keep_pr_review_tracking_separate_from_independent_reviews() {
    local compatibility_redirect
    local helper_skill

    require_absent_text "${COPILOT_HANDLER}" 'agent-review-reports.md'
    require_text "${COPILOT_HANDLER}" 'docs/pr-reviews/pr-<PR_NUMBER>-review.md'
    require_text "${COPILOT_HANDLER}" 'process-pr-review skill'
    require_wrapped_text "${COPILOT_HANDLER}" 'Do not maintain a parallel Copilot-only audit procedure.'
    require_wrapped_text "${COPILOT_HANDLER}" 'commit-subject citation'
    require_absent_text "${COPILOT_HANDLER}" 'commit SHA'
    require_absent_text "${COPILOT_HANDLER}" 'branch SHA'
    require_text "${COPILOT_PROMPT}" 'process PR review skill'
    require_text "${COPILOT_PROMPT}" 'canonical skill exclusively defines audit fields'
    require_text "${COPILOT_PROMPT}" 'docs/pr-reviews/pr-<PR_NUMBER>-review.md'
    require_wrapped_text "${COPILOT_PROMPT}" 'commit-subject citation'
    require_absent_text "${COPILOT_PROMPT}" 'commit SHA'
    require_absent_text "${COPILOT_PROMPT}" 'branch SHA'
    require_text "${ORCHESTRATION}" 'process_pr_review[process-pr-review]'
    require_text "${ORCHESTRATION}" 'pr_review_audit[Pull-request review audit]'
    require_absent_text "${ORCHESTRATION}" 'Copilot suggestions tracker'

    for compatibility_redirect in "${COPILOT_REDIRECT}" "${FEEDBACK_REDIRECT}"; do
        require_yaml_frontmatter "${compatibility_redirect}"
        require_text "${compatibility_redirect}" 'This compatibility entry point is retained for one release.'
        require_text "${compatibility_redirect}" 'process-pr-review'
        require_text "${compatibility_redirect}" 'docs/pr-reviews/pr-<PR_NUMBER>-review.md'
        require_absent_text "${compatibility_redirect}" 'docs/copilot-pr-reviews/'
        require_absent_text "${compatibility_redirect}" 'docs/pr-review-feedback/'
    done

    for helper_skill in "${FETCH_THREADS}" "${RESOLVE_THREADS}"; do
        require_text "${helper_skill}" 'component skill within the **process-pr-review** workflow'
        require_text "${helper_skill}" 'See **process-pr-review** for the full end-to-end process.'
    done

    require_yaml_frontmatter "${PROCESS_PR_REVIEW}"
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" 'docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" 'docs/templates/PR-REVIEW-TEMPLATE.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" '.github/PULL_REQUEST_TEMPLATE/review-findings.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" '.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" '.github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md'
}

it_should_define_the_reusable_report_template_contract
it_should_grant_each_independent_reviewer_edit_access
it_should_define_the_shared_create_append_or_skip_policy
it_should_keep_commit_authority_with_the_caller_and_committer
it_should_keep_pr_review_tracking_separate_from_independent_reviews

printf 'All agent review report contract tests passed.\n'
