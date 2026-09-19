//! Validates the repository's independent agent-review documentation contract.

use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;

const REQUIRED_TEXT: &[(&str, &str)] = &[
    (
        "docs/templates/AGENT-REVIEW-REPORTS.md",
        "Append one completed independent-review entry at a time.",
    ),
    (
        "docs/templates/AGENT-REVIEW-REPORTS.md",
        "### {YYYY-MM-DD HH:MM UTC} - {Reviewer}",
    ),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Invocation scope:"),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Inputs:"),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Evidence:"),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Findings:"),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Verdict:"),
    ("docs/templates/AGENT-REVIEW-REPORTS.md", "- Follow-up actions:"),
    (
        ".github/agents/copilot-suggestions-handler.agent.md",
        "docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md",
    ),
    (
        ".github/prompts/process-copilot-suggestions.prompt.md",
        "docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md",
    ),
    ("docs/agents/orchestration.md", "pr_review_audit[Pull-request review audit]"),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md",
    ),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "| Author class |"),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "| Category |"),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |",
    ),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "## Finding Details"),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "### <FINDING_ID> - <SUMMARY>"),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "- Current-tree verification: <COMMAND_OR_INSPECTION_AND_RESULT>",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>",
    ),
    (
        "docs/skills/semantic-skill-link-convention.md",
        "`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`",
    ),
    (
        "docs/skills/semantic-skill-link-convention.md",
        "| `review-finding`    | `pr-<number>-<id>`",
    ),
    ("docs/skills/semantic-skill-link-convention.md", "`review-finding:pr-2230-f1`"),
    (
        ".github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md",
        "component skill within the **process-pr-review** workflow",
    ),
    (
        ".github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md",
        "See **process-pr-review** for the full end-to-end process.",
    ),
    (
        ".github/agents/copilot-suggestions-handler.agent.md",
        "process-pr-review skill",
    ),
    (
        ".github/prompts/process-copilot-suggestions.prompt.md",
        "canonical skill exclusively defines audit fields",
    ),
    (
        ".github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md",
        "This compatibility entry point is retained for one release.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md",
        "This compatibility entry point is retained for one release.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "docs/templates/REVIEW-FINDINGS.md",
    ),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "| Review finding reference |"),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "<REVIEW_FINDING_REFERENCE>"),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "Never change a reference after assigning it.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Classify `github-copilot[bot]` and",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "`copilot-pull-request-reviewer` as `Copilot`, human accounts as `Human`, and",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "every other bot or unavailable author as `Unknown`.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "**Categorize for future analysis.** For every new audit row, assign exactly one",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "primary category: `link-integrity`, `formatting`, `metadata`, `testing`,",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "`correctness`, `documentation`, `maintainability`, `security`, or `other`.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "author class, finding ID, review finding reference, severity, category, summary,",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "- Author class: `Copilot`, `Human`, `Unknown`",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "`link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "`documentation`, `maintainability`, `security`, `other`",
    ),
    ("docs/templates/PR-REVIEW-TEMPLATE.md", "exactly one primary"),
    (
        "docs/skills/semantic-skill-link-convention.md",
        "It is immutable once assigned",
    ),
];

const WRAPPED_TEXT: &[(&str, &str)] = &[
    (
        ".github/agents/complexity-auditor.agent.md",
        "When the caller supplies an existing folder-style issue specification path whose primary file is `ISSUE.md` or `EPIC.md`, persist this independent review before returning the caller-facing verdict.",
    ),
    (
        ".github/agents/task-reviewer.agent.md",
        "create `agent-review-reports.md` from `docs/templates/AGENT-REVIEW-REPORTS.md` when absent; otherwise append one complete entry after the final existing report entry.",
    ),
    (
        ".github/agents/pr-reviewer.agent.md",
        "Issue-local report skipped: no folder-style issue specification was supplied.",
    ),
    (
        ".github/agents/task-reviewer.agent.md",
        "Do not invoke Committer or self-commit a report. When the report changes a branch or pull-request worktree, the caller requests Committer to include it in the coherent reviewed change set or create a focused documentation commit.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "For every new audit row, record the source author's derived `Author class` and exactly one primary `Category`.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Record each finding as one compact tracking row (finding ID, review finding reference, author class, severity, category, relationship, disposition, thread state) plus one matching detail entry carrying the remaining narrative and source-metadata fields, as laid out in the audit template.",
    ),
    (
        "docs/skills/semantic-skill-link-convention.md",
        "GitHub thread, comment, review, and URL identifiers remain source metadata",
    ),
    (
        ".github/agents/complexity-auditor.agent.md",
        "Do not alter the reviewed implementation or invoke Committer.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "Record each finding in two coordinated places: one compact tracking row below, and one matching entry in `## Finding Details` for the narrative fields.",
    ),
    (
        "docs/skills/semantic-skill-link-convention.md",
        "Use the reference in Markdown prose or as a `semantic-links.related-artifacts` value",
    ),
    (
        ".github/agents/task-reviewer.agent.md",
        "When the caller supplies an existing folder-style issue specification path whose primary file is `ISSUE.md` or `EPIC.md`, persist this independent review before returning the caller-facing verdict.",
    ),
    (
        ".github/agents/pr-reviewer.agent.md",
        "When the caller supplies an existing folder-style issue specification path whose primary file is `ISSUE.md` or `EPIC.md`, persist this independent review before returning the caller-facing verdict.",
    ),
    (
        ".github/agents/complexity-auditor.agent.md",
        "create `agent-review-reports.md` from `docs/templates/AGENT-REVIEW-REPORTS.md` when absent; otherwise append one complete entry after the final existing report entry.",
    ),
    (
        ".github/agents/pr-reviewer.agent.md",
        "create `agent-review-reports.md` from `docs/templates/AGENT-REVIEW-REPORTS.md` when absent; otherwise append one complete entry after the final existing report entry.",
    ),
    (
        ".github/agents/complexity-auditor.agent.md",
        "Issue-local report skipped: no folder-style issue specification was supplied.",
    ),
    (
        ".github/agents/task-reviewer.agent.md",
        "Issue-local report skipped: no folder-style issue specification was supplied.",
    ),
    (
        ".github/agents/pr-reviewer.agent.md",
        "Do not invoke Committer or self-commit a report. When the report changes a branch or pull-request worktree, the caller requests Committer to include it in the coherent reviewed change set or create a focused documentation commit.",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Categorize the concern rather than its proposed fix;",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Historical records remain valid without the analysis fields.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "Record each finding in two coordinated places: one compact tracking row below, and one matching entry in `## Finding Details` for the narrative fields.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with the finding ID lowercased.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "GitHub identifiers remain source metadata, not the canonical finding reference.",
    ),
    (
        "docs/templates/PR-REVIEW-TEMPLATE.md",
        "This convention applies to new audits only. Historical audit records remain unchanged",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Never change the reference after assigning it. Historical audit records remain unchanged",
    ),
    (
        ".github/agents/copilot-suggestions-handler.agent.md",
        "Do not maintain a parallel Copilot-only audit procedure.",
    ),
    (
        ".github/agents/copilot-suggestions-handler.agent.md",
        "commit-subject citation",
    ),
    (
        ".github/prompts/process-copilot-suggestions.prompt.md",
        "commit-subject citation",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Record each finding as one compact tracking row (finding ID, review finding reference, author class, severity, category, relationship, disposition, thread state) plus one matching detail entry carrying the remaining narrative and source-metadata fields, as laid out in the audit template.",
    ),
];

const EDIT_CAPABLE_REVIEWERS: &[&str] = &[
    ".github/agents/complexity-auditor.agent.md",
    ".github/agents/task-reviewer.agent.md",
    ".github/agents/pr-reviewer.agent.md",
];

const FRONTMATTER_FILES: &[&str] = &[
    "docs/templates/AGENT-REVIEW-REPORTS.md",
    ".github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md",
    ".github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md",
    ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
];

const PROCESS_PR_REVIEW_ARTIFACTS: &[&str] = &[
    "docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md",
    "docs/templates/PR-REVIEW-TEMPLATE.md",
    "docs/templates/REVIEW-FINDINGS.md",
    ".github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md",
    ".github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md",
];

const FORBIDDEN_TEXT: &[(&str, &str)] = &[
    (
        ".github/agents/copilot-suggestions-handler.agent.md",
        "agent-review-reports.md",
    ),
    (".github/agents/copilot-suggestions-handler.agent.md", "commit SHA"),
    (".github/agents/copilot-suggestions-handler.agent.md", "branch SHA"),
    (".github/prompts/process-copilot-suggestions.prompt.md", "commit SHA"),
    (".github/prompts/process-copilot-suggestions.prompt.md", "branch SHA"),
    ("docs/agents/orchestration.md", "Copilot suggestions tracker"),
    (
        ".github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md",
        "docs/copilot-pr-reviews/",
    ),
    (
        ".github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md",
        "docs/pr-review-feedback/",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md",
        "docs/copilot-pr-reviews/",
    ),
    (
        ".github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md",
        "docs/pr-review-feedback/",
    ),
    (".github/agents/complexity-auditor.agent.md", "git commit"),
    ("docs/templates/REVIEW-FINDINGS.md", "Category"),
];

fn main() -> ExitCode {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("workspace root");
    let mut failures = Vec::new();

    for &(path, text) in REQUIRED_TEXT {
        if !contains(workspace_root, path, text, &mut failures) {
            failures.push(format!("expected `{path}` to contain `{text}`"));
        }
    }

    for &(path, text) in WRAPPED_TEXT {
        if !contains_normalized(workspace_root, path, text, &mut failures) {
            failures.push(format!("expected `{path}` to contain wrapped text `{text}`"));
        }
    }

    for path in EDIT_CAPABLE_REVIEWERS {
        if !has_edit_tool(workspace_root, path, &mut failures) {
            failures.push(format!("expected `{path}` to declare the edit tool"));
        }
    }

    for path in FRONTMATTER_FILES {
        if !has_frontmatter(workspace_root, path, &mut failures) {
            failures.push(format!("expected `{path}` to begin with closed YAML frontmatter"));
        }
    }

    for artifact in PROCESS_PR_REVIEW_ARTIFACTS {
        if !has_related_artifact(
            workspace_root,
            ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
            artifact,
            &mut failures,
        ) {
            failures.push(format!(
                "expected process-pr-review frontmatter to contain related artifact `{artifact}`"
            ));
        }
    }

    for &(path, text) in FORBIDDEN_TEXT {
        if contains(workspace_root, path, text, &mut failures) {
            failures.push(format!("expected `{path}` not to contain `{text}`"));
        }
    }

    verify_persistence_policy(workspace_root, &mut failures);
    verify_committer_authority(workspace_root, &mut failures);
    verify_review_routing(workspace_root, &mut failures);
    verify_audit_schema(workspace_root, &mut failures);
    verify_finding_details(workspace_root, &mut failures);
    verify_portable_references(workspace_root, &mut failures);

    if failures.is_empty() {
        match writeln!(io::stdout().lock(), "Agent review report contract check passed.") {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        }
    } else {
        let mut stderr = io::stderr().lock();
        for failure in failures {
            if writeln!(stderr, "{failure}").is_err() {
                return ExitCode::FAILURE;
            }
        }
        ExitCode::FAILURE
    }
}

fn contains(workspace_root: &Path, relative_path: &str, expected_text: &str, failures: &mut Vec<String>) -> bool {
    let path = workspace_root.join(relative_path);
    match fs::read_to_string(&path) {
        Ok(contents) => contents.contains(expected_text),
        Err(error) => {
            failures.push(format!("could not read `{}`: {error}", display_path(&path)));
            false
        }
    }
}

fn contains_normalized(workspace_root: &Path, relative_path: &str, expected_text: &str, failures: &mut Vec<String>) -> bool {
    let path = workspace_root.join(relative_path);
    match fs::read_to_string(&path) {
        Ok(contents) => normalize_whitespace(&contents).contains(&normalize_whitespace(expected_text)),
        Err(error) => {
            failures.push(format!("could not read `{}`: {error}", display_path(&path)));
            false
        }
    }
}

fn verify_persistence_policy(workspace_root: &Path, failures: &mut Vec<String>) {
    for reviewer in EDIT_CAPABLE_REVIEWERS {
        require_wrapped(
            workspace_root,
            reviewer,
            "When the caller supplies an existing folder-style issue specification path whose primary file is `ISSUE.md` or `EPIC.md`, persist this independent review before returning the caller-facing verdict.",
            failures,
        );
        require_wrapped(
            workspace_root,
            reviewer,
            "create `agent-review-reports.md` from `docs/templates/AGENT-REVIEW-REPORTS.md` when absent; otherwise append one complete entry after the final existing report entry.",
            failures,
        );
        require_wrapped(
            workspace_root,
            reviewer,
            "Issue-local report skipped: no folder-style issue specification was supplied.",
            failures,
        );
    }
}

fn verify_committer_authority(workspace_root: &Path, failures: &mut Vec<String>) {
    let complexity_auditor = ".github/agents/complexity-auditor.agent.md";
    forbid(workspace_root, complexity_auditor, "git commit", failures);
    require(
        workspace_root,
        complexity_auditor,
        "Do not alter the reviewed implementation or invoke Committer.",
        failures,
    );

    for reviewer in [".github/agents/task-reviewer.agent.md", ".github/agents/pr-reviewer.agent.md"] {
        require_wrapped(
            workspace_root,
            reviewer,
            "Do not invoke Committer or self-commit a report. When the report changes a branch or pull-request worktree, the caller requests Committer to include it in the coherent reviewed change set or create a focused documentation commit.",
            failures,
        );
    }
}

fn verify_review_routing(workspace_root: &Path, failures: &mut Vec<String>) {
    let handler = ".github/agents/copilot-suggestions-handler.agent.md";
    let prompt = ".github/prompts/process-copilot-suggestions.prompt.md";
    let audit_path = "docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md";

    require(workspace_root, handler, audit_path, failures);
    require_wrapped(
        workspace_root,
        handler,
        "Do not maintain a parallel Copilot-only audit procedure.",
        failures,
    );
    require_wrapped(workspace_root, handler, "commit-subject citation", failures);
    forbid(workspace_root, handler, "agent-review-reports.md", failures);
    forbid(workspace_root, handler, "commit SHA", failures);
    forbid(workspace_root, handler, "branch SHA", failures);
    require(workspace_root, prompt, "process PR review skill", failures);
    require(
        workspace_root,
        prompt,
        "canonical skill exclusively defines audit fields",
        failures,
    );
    require(workspace_root, prompt, audit_path, failures);
    require_wrapped(workspace_root, prompt, "commit-subject citation", failures);
    forbid(workspace_root, prompt, "commit SHA", failures);
    forbid(workspace_root, prompt, "branch SHA", failures);
    require(
        workspace_root,
        "docs/agents/orchestration.md",
        "process_pr_review[process-pr-review]",
        failures,
    );

    for helper in [
        ".github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md",
        ".github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md",
    ] {
        require(
            workspace_root,
            helper,
            "component skill within the **process-pr-review** workflow",
            failures,
        );
        require(
            workspace_root,
            helper,
            "See **process-pr-review** for the full end-to-end process.",
            failures,
        );
    }

    for redirect in [
        ".github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md",
        ".github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md",
    ] {
        require_frontmatter(workspace_root, redirect, failures);
        require(
            workspace_root,
            redirect,
            "This compatibility entry point is retained for one release.",
            failures,
        );
        require(workspace_root, redirect, "process-pr-review", failures);
        require(workspace_root, redirect, audit_path, failures);
        forbid(workspace_root, redirect, "docs/copilot-pr-reviews/", failures);
        forbid(workspace_root, redirect, "docs/pr-review-feedback/", failures);
    }
}

fn verify_audit_schema(workspace_root: &Path, failures: &mut Vec<String>) {
    let template = "docs/templates/PR-REVIEW-TEMPLATE.md";
    let workflow = ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md";

    for text in [
        "| Author class |",
        "| Category |",
        "- Author class: `Copilot`, `Human`, `Unknown`",
        "`link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,",
        "`documentation`, `maintainability`, `security`, `other`",
        "exactly one primary",
    ] {
        require(workspace_root, template, text, failures);
    }
    require_wrapped(
        workspace_root,
        template,
        "For every new audit row, record the source author's derived `Author class` and exactly one primary `Category`.",
        failures,
    );
    require_wrapped(
        workspace_root,
        template,
        "Historical records predate this schema and remain unchanged.",
        failures,
    );
    for text in [
        "Classify `github-copilot[bot]` and",
        "`copilot-pull-request-reviewer` as `Copilot`, human accounts as `Human`, and",
        "every other bot or unavailable author as `Unknown`.",
        "**Categorize for future analysis.** For every new audit row, assign exactly one",
        "primary category: `link-integrity`, `formatting`, `metadata`, `testing`,",
        "`correctness`, `documentation`, `maintainability`, `security`, or `other`.",
        "author class, finding ID, review finding reference, severity, category, summary,",
        "listed category fits. Do not backfill or reinterpret historical audit records.",
    ] {
        require(workspace_root, workflow, text, failures);
    }
    require_wrapped(
        workspace_root,
        workflow,
        "Categorize the concern rather than its proposed fix;",
        failures,
    );
    require_wrapped(
        workspace_root,
        workflow,
        "Historical records remain valid without the analysis fields.",
        failures,
    );
    forbid(workspace_root, "docs/templates/REVIEW-FINDINGS.md", "Category", failures);
}

fn verify_finding_details(workspace_root: &Path, failures: &mut Vec<String>) {
    let template = "docs/templates/PR-REVIEW-TEMPLATE.md";
    require(
        workspace_root,
        template,
        "| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |",
        failures,
    );
    require(workspace_root, template, "## Finding Details", failures);
    require(workspace_root, template, "### <FINDING_ID> - <SUMMARY>", failures);
    require(
        workspace_root,
        template,
        "- Current-tree verification: <COMMAND_OR_INSPECTION_AND_RESULT>",
        failures,
    );
    require(
        workspace_root,
        template,
        "- Resolution reference: <UNIQUE_COMMIT_SUBJECT_OR_REPLY_URL>",
        failures,
    );
    require_wrapped(
        workspace_root,
        template,
        "Record each finding in two coordinated places: one compact tracking row below, and one matching entry in `## Finding Details` for the narrative fields.",
        failures,
    );
    require_wrapped(
        workspace_root,
        ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md",
        "Record each finding as one compact tracking row (finding ID, review finding reference, author class, severity, category, relationship, disposition, thread state) plus one matching detail entry carrying the remaining narrative and source-metadata fields, as laid out in the audit template.",
        failures,
    );
}

fn verify_portable_references(workspace_root: &Path, failures: &mut Vec<String>) {
    let convention = "docs/skills/semantic-skill-link-convention.md";
    let template = "docs/templates/PR-REVIEW-TEMPLATE.md";
    let workflow = ".github/skills/dev/pr-reviews/process-pr-review/SKILL.md";
    for text in [
        "| `review-finding`    | `pr-<number>-<id>`",
        "`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`",
        "`review-finding:pr-2230-f1`",
        "It is immutable once assigned",
    ] {
        require(workspace_root, convention, text, failures);
    }
    require_wrapped(
        workspace_root,
        convention,
        "GitHub thread, comment, review, and URL identifiers remain source metadata",
        failures,
    );
    require_wrapped(
        workspace_root,
        convention,
        "Use the reference in Markdown prose or as a `semantic-links.related-artifacts` value",
        failures,
    );
    for text in [
        "| Review finding reference |",
        "<REVIEW_FINDING_REFERENCE>",
        "Assign each new row an immutable repository reference in the form",
        "Never change a reference after assigning it.",
    ] {
        require(workspace_root, template, text, failures);
    }
    require_wrapped(
        workspace_root,
        template,
        "`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with the finding ID lowercased.",
        failures,
    );
    require_wrapped(
        workspace_root,
        template,
        "GitHub identifiers remain source metadata, not the canonical finding reference.",
        failures,
    );
    require_wrapped(
        workspace_root,
        template,
        "This convention applies to new audits only. Historical audit records remain unchanged",
        failures,
    );
    require_wrapped(
        workspace_root,
        workflow,
        "source-review and source-order order. Assign the immutable repository reference",
        failures,
    );
    require(
        workspace_root,
        workflow,
        "`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, lowercasing the finding ID in",
        failures,
    );
    require_wrapped(
        workspace_root,
        workflow,
        "GitHub identifiers remain source metadata, not the canonical finding reference.",
        failures,
    );
    require_wrapped(
        workspace_root,
        workflow,
        "Never change the reference after assigning it. Historical audit records remain unchanged",
        failures,
    );
}

fn require(workspace_root: &Path, path: &str, text: &str, failures: &mut Vec<String>) {
    if !contains(workspace_root, path, text, failures) {
        failures.push(format!("expected `{path}` to contain `{text}`"));
    }
}

fn require_wrapped(workspace_root: &Path, path: &str, text: &str, failures: &mut Vec<String>) {
    if !contains_normalized(workspace_root, path, text, failures) {
        failures.push(format!("expected `{path}` to contain wrapped text `{text}`"));
    }
}

fn forbid(workspace_root: &Path, path: &str, text: &str, failures: &mut Vec<String>) {
    if contains(workspace_root, path, text, failures) {
        failures.push(format!("expected `{path}` not to contain `{text}`"));
    }
}

fn require_frontmatter(workspace_root: &Path, path: &str, failures: &mut Vec<String>) {
    if !has_frontmatter(workspace_root, path, failures) {
        failures.push(format!("expected `{path}` to begin with closed YAML frontmatter"));
    }
}

fn normalize_whitespace(value: &str) -> String {
    // Normalize wrapping without treating paragraph breaks as semantic content.
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn has_frontmatter(workspace_root: &Path, relative_path: &str, failures: &mut Vec<String>) -> bool {
    let path = workspace_root.join(relative_path);
    match fs::read_to_string(&path) {
        Ok(contents) => contents.starts_with("---\n") && contents[4..].contains("\n---\n"),
        Err(error) => {
            failures.push(format!("could not read `{}`: {error}", display_path(&path)));
            false
        }
    }
}

fn has_related_artifact(workspace_root: &Path, relative_path: &str, artifact: &str, failures: &mut Vec<String>) -> bool {
    let path = workspace_root.join(relative_path);
    match fs::read_to_string(&path) {
        Ok(contents) => contents
            .lines()
            .skip(1)
            .take_while(|line| *line != "---")
            .scan(false, |in_list, line| {
                if line.trim() == "related-artifacts:" {
                    *in_list = true;
                } else if *in_list && !line.starts_with(char::is_whitespace) {
                    *in_list = false;
                }
                Some(*in_list && line.trim_start().strip_prefix("- ") == Some(artifact))
            })
            .any(|matches| matches),
        Err(error) => {
            failures.push(format!("could not read `{}`: {error}", display_path(&path)));
            false
        }
    }
}

fn has_edit_tool(workspace_root: &Path, relative_path: &str, failures: &mut Vec<String>) -> bool {
    let path = workspace_root.join(relative_path);
    match fs::read_to_string(&path) {
        Ok(contents) => contents
            .lines()
            .skip(1)
            .take_while(|line| *line != "---")
            .find_map(|line| line.strip_prefix("tools:"))
            .is_some_and(|tools| {
                tools
                    .split(|character: char| !character.is_alphanumeric())
                    .any(|tool| tool == "edit")
            }),
        Err(error) => {
            failures.push(format!("could not read `{}`: {error}", display_path(&path)));
            false
        }
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::normalize_whitespace;

    #[test]
    fn normalize_whitespace_accepts_indented_wrapping() {
        assert_eq!(normalize_whitespace("one\n   two"), "one two");
    }

    #[test]
    fn normalize_whitespace_accepts_paragraph_breaks() {
        assert_eq!(normalize_whitespace("one\n\ntwo"), "one two");
    }
}
