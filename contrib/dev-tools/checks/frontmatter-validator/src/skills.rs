//! Repository-wide rules for where agent skills live.
//!
//! Claude Code discovers a project skill only at `.claude/skills/<name>/SKILL.md`: a skill anywhere
//! else, or one folder deeper, is never listed to the model. These decisions are pure: they take
//! repository-relative paths with `/` separators and never touch the filesystem or git.

use crate::{Diagnostic, DiagnosticCategory};

/// The only folder Claude Code discovers project skills in.
pub const SKILLS_DIRECTORY: &str = ".claude/skills/";

const SKILL_FILE: &str = "SKILL.md";

/// Reports a `SKILL.md` that Claude Code would not discover; any other path has no finding.
#[must_use]
pub fn misplaced_skill(path: &str) -> Option<Diagnostic> {
    let is_skill_file = path == SKILL_FILE || path.ends_with(&format!("/{SKILL_FILE}"));
    (is_skill_file && skill_name(path).is_none()).then(|| {
        Diagnostic::new(
            DiagnosticCategory::MisplacedSkill,
            format!("A skill must be at `{SKILLS_DIRECTORY}<name>/{SKILL_FILE}`, where Claude Code discovers it."),
        )
    })
}

/// The name of the skill a discoverable `SKILL.md` path belongs to: its folder name.
#[must_use]
pub fn skill_name(path: &str) -> Option<&str> {
    path.strip_prefix(SKILLS_DIRECTORY)?
        .strip_suffix(&format!("/{SKILL_FILE}"))
        .filter(|name| !name.is_empty() && !name.contains('/'))
}

#[cfg(test)]
mod tests {
    // Owns the placement rule: which `SKILL.md` paths Claude Code discovers. Which files the rule
    // runs over (the index) is owned by the binary and tested in `tests/cli.rs`.

    use rstest::rstest;

    use super::misplaced_skill;
    use crate::DiagnosticCategory;

    #[rstest]
    #[case::under_the_copilot_folder(".github/skills/dev/planning/create-issue/SKILL.md")]
    #[case::one_folder_too_deep(".claude/skills/planning/create-issue/SKILL.md")]
    #[case::directly_in_the_skills_folder(".claude/skills/SKILL.md")]
    #[case::at_the_repository_root("SKILL.md")]
    fn it_should_report_a_skill_that_claude_code_would_not_discover(#[case] path: &str) {
        let category = misplaced_skill(path).map(|diagnostic| diagnostic.category);

        assert_eq!(category, Some(DiagnosticCategory::MisplacedSkill));
    }

    #[rstest]
    #[case::flat_skill(".claude/skills/create-issue/SKILL.md")]
    #[case::other_file_in_a_skill(".claude/skills/create-issue/references/SKILL-notes.md")]
    #[case::skill_named_folder("docs/SKILL.md/notes.md")]
    #[case::not_a_skill("docs/AGENTS.md")]
    fn it_should_accept_a_path_that_is_a_discoverable_skill_or_no_skill(#[case] path: &str) {
        assert_eq!(misplaced_skill(path), None);
    }
}
