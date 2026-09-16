#!/usr/bin/env bash
# Structural contract tests for the independent agent-review report workflow.
#
# Run manually after modifying the template or reviewer profiles:
#   bash contrib/dev-tools/checks/tests/test-agent-review-report-contract.sh
# This test verifies tracked documentation/profile contracts, not autonomous agent behavior.

set -euo pipefail

PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
TEMPLATE="${PROJECT_ROOT}/docs/templates/AGENT-REVIEW-REPORTS.md"
PR_REVIEW_TEMPLATE="${PROJECT_ROOT}/docs/templates/PR-REVIEW-TEMPLATE.md"
REVIEW_FINDINGS_TEMPLATE="${PROJECT_ROOT}/docs/templates/REVIEW-FINDINGS.md"
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
SEMANTIC_LINK_CONVENTION="${PROJECT_ROOT}/docs/skills/semantic-skill-link-convention.md"

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

    # Parses the block-style related-artifacts list used across repository frontmatter.
    if ! awk -v artifact="${expected_artifact}" '
        NR == 1 && $0 != "---" { exit 1 }
        NR > 1 && $0 == "---" { exit !found }
        /^[[:space:]]*related-artifacts:[[:space:]]*$/ { in_list = 1; next }
        in_list && /^[[:space:]]*-[[:space:]]/ {
            value = $0
            sub(/^[[:space:]]*-[[:space:]]*/, "", value)
            sub(/[[:space:]]*$/, "", value)
            if (value == artifact) { found = 1 }
            next
        }
        in_list { in_list = 0 }
        END { exit !found }
    ' "${file_path}"; then
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
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" 'docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" 'docs/templates/PR-REVIEW-TEMPLATE.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" 'docs/templates/REVIEW-FINDINGS.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" '.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md'
    require_yaml_related_artifact "${PROCESS_PR_REVIEW}" '.github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md'
}

it_should_define_analysis_fields_for_new_pr_review_audits() {
    require_text "${PR_REVIEW_TEMPLATE}" '| Author class |'
    require_text "${PR_REVIEW_TEMPLATE}" '| Category |'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PR_REVIEW_TEMPLATE}" '- Author class: `Copilot`, `Human`, `Unknown`'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PR_REVIEW_TEMPLATE}" '`link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PR_REVIEW_TEMPLATE}" '`documentation`, `maintainability`, `security`, `other`'
    require_text "${PR_REVIEW_TEMPLATE}" 'exactly one primary'
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" $'For every new audit row, record the source author\'s derived `Author class` and exactly one primary `Category`.'
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" 'Historical records predate this schema and remain unchanged.'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" 'Classify `github-copilot[bot]` and'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" '`copilot-pull-request-reviewer` as `Copilot`, human accounts as `Human`, and'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" 'every other bot or unavailable author as `Unknown`.'
    require_text "${PROCESS_PR_REVIEW}" '**Categorize for future analysis.** For every new audit row, assign exactly one'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" 'primary category: `link-integrity`, `formatting`, `metadata`, `testing`,'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" '`correctness`, `documentation`, `maintainability`, `security`, or `other`.'
    require_text "${PROCESS_PR_REVIEW}" 'author class, finding ID, review finding reference, severity, category, summary,'
    require_wrapped_text "${PROCESS_PR_REVIEW}" 'Categorize the concern rather than its proposed fix;'
    require_text "${PROCESS_PR_REVIEW}" 'listed category fits. Do not backfill or reinterpret historical audit records.'
    require_wrapped_text "${PROCESS_PR_REVIEW}" 'Historical records remain valid without the analysis fields.'
    require_absent_text "${REVIEW_FINDINGS_TEMPLATE}" 'Category'
}

it_should_split_audit_tracking_from_finding_details() {
    require_text "${PR_REVIEW_TEMPLATE}" '| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |'
    require_text "${PR_REVIEW_TEMPLATE}" '## Finding Details'
    require_text "${PR_REVIEW_TEMPLATE}" '### <FINDING_ID> - <SUMMARY>'
    require_text "${PR_REVIEW_TEMPLATE}" '- Current-tree verification: <COMMAND_OR_INSPECTION_AND_RESULT>'
    require_text "${PR_REVIEW_TEMPLATE}" '- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" 'Record each finding in two coordinated places: one compact tracking row below, and one matching entry in `## Finding Details` for the narrative fields.'
    require_wrapped_text "${PROCESS_PR_REVIEW}" 'Record each finding as one compact tracking row (finding ID, review finding reference, author class, severity, category, relationship, disposition, thread state) plus one matching detail entry carrying the remaining narrative and source-metadata fields, as laid out in the audit template.'
}

it_should_define_portable_review_finding_references() {
    require_text "${SEMANTIC_LINK_CONVENTION}" "| \`review-finding\`    | \`pr-<number>-<id>\`"
    require_text "${SEMANTIC_LINK_CONVENTION}" "\`review-finding:pr-<PR_NUMBER>-<FINDING_ID>\`"
    require_text "${SEMANTIC_LINK_CONVENTION}" "\`review-finding:pr-2230-f1\`"
    require_text "${SEMANTIC_LINK_CONVENTION}" 'It is immutable once assigned'
    require_wrapped_text "${SEMANTIC_LINK_CONVENTION}" 'GitHub thread, comment, review, and URL identifiers remain source metadata'
    require_wrapped_text "${SEMANTIC_LINK_CONVENTION}" "Use the reference in Markdown prose or as a \`semantic-links.related-artifacts\` value"
    require_text "${PR_REVIEW_TEMPLATE}" '| Review finding reference |'
    require_text "${PR_REVIEW_TEMPLATE}" '<REVIEW_FINDING_REFERENCE>'
    require_text "${PR_REVIEW_TEMPLATE}" 'Assign each new row an immutable repository reference in the form'
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" "\`review-finding:pr-<PR_NUMBER>-<FINDING_ID>\`, with the finding ID lowercased."
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" 'GitHub identifiers remain source metadata, not the canonical finding reference.'
    require_text "${PR_REVIEW_TEMPLATE}" 'Never change a reference after assigning it.'
    require_wrapped_text "${PR_REVIEW_TEMPLATE}" 'This convention applies to new audits only. Historical audit records remain unchanged'
    require_text "${PROCESS_PR_REVIEW}" 'order. Assign the immutable repository reference'
    # shellcheck disable=SC2016 # Expected text includes literal Markdown code spans.
    require_text "${PROCESS_PR_REVIEW}" '`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, lowercasing the finding ID in'
    require_wrapped_text "${PROCESS_PR_REVIEW}" 'GitHub identifiers remain source metadata, not the canonical finding reference.'
    require_wrapped_text "${PROCESS_PR_REVIEW}" 'Never change the reference after assigning it. Historical audit records remain unchanged'
}

it_should_define_the_reusable_report_template_contract
it_should_grant_each_independent_reviewer_edit_access
it_should_define_the_shared_create_append_or_skip_policy
it_should_keep_commit_authority_with_the_caller_and_committer
it_should_keep_pr_review_tracking_separate_from_independent_reviews
it_should_define_analysis_fields_for_new_pr_review_audits
it_should_split_audit_tracking_from_finding_details
it_should_define_portable_review_finding_references

printf 'All agent review report contract tests passed.\n'
