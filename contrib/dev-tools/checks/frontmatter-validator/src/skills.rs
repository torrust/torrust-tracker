//! Repository-wide rules for where agent skills live.
//!
//! Claude Code discovers a project skill only at `.claude/skills/<name>/SKILL.md`: a skill anywhere
//! else, or one folder deeper, is never listed to the model. The skills used to live under
//! `.github/skills/`; each old `SKILL.md` path keeps a redirect stub, inventoried in
//! `.github/skills/README.md` (issue #2866). These decisions are pure: they take
//! repository-relative paths with `/` separators and file texts, and never touch the filesystem or
//! git.

use std::collections::{BTreeMap, BTreeSet};

use crate::{Diagnostic, DiagnosticCategory};

/// The only folder Claude Code discovers project skills in.
pub const SKILLS_DIRECTORY: &str = ".claude/skills/";

/// The folder the skills moved out of; it holds only redirect stubs and their inventory.
pub const REDIRECT_DIRECTORY: &str = ".github/skills/";

/// The inventory of the redirect stubs.
pub const STUB_INVENTORY: &str = ".github/skills/README.md";

const SKILL_FILE: &str = "SKILL.md";

/// A finding of the repository-wide skill rules and the path it concerns.
#[derive(Debug, Eq, PartialEq)]
pub struct LayoutFinding {
    /// The repository-relative path the finding is reported for.
    pub path: String,
    /// What is wrong there.
    pub diagnostic: Diagnostic,
}

/// Whether the layout rules read the text of this path; the caller supplies only those texts.
#[must_use]
pub fn reads_text(path: &str) -> bool {
    path == STUB_INVENTORY
}

/// Checks the skill layout of a whole repository snapshot: `files` are every tracked path, and
/// `text` returns the text of a path for which [`reads_text`] is true. Findings are sorted by path.
pub fn layout_findings<'a>(
    files: impl IntoIterator<Item = &'a str>,
    text: impl Fn(&str) -> Option<String>,
) -> Vec<LayoutFinding> {
    let files: BTreeSet<&str> = files.into_iter().collect();
    let inventory = text(STUB_INVENTORY)
        .map(|inventory| stub_inventory(&inventory))
        .unwrap_or_default();

    let mut findings: Vec<LayoutFinding> = files
        .iter()
        .filter(|path| !inventory.contains_key(**path))
        .filter_map(|path| {
            misplaced_skill(path)
                .or_else(|| stray_redirect_file(path))
                .map(|diagnostic| finding(path, diagnostic))
        })
        .collect();
    findings.extend(
        inventory
            .keys()
            .filter(|path| !files.contains(path.as_str()))
            .map(|path| finding(path, missing_redirect_stub())),
    );
    findings.sort_by(|left, right| left.path.cmp(&right.path));
    findings
}

fn finding(path: &str, diagnostic: Diagnostic) -> LayoutFinding {
    LayoutFinding {
        path: path.to_owned(),
        diagnostic,
    }
}

/// The inventoried old `SKILL.md` paths, each with its row's current skill name or `removed`.
fn stub_inventory(markdown: &str) -> BTreeMap<String, String> {
    markdown
        .lines()
        .filter_map(|line| {
            let cells: Vec<&str> = line
                .strip_prefix('|')?
                .strip_suffix('|')?
                .split('|')
                .map(|cell| cell.trim().trim_matches('`'))
                .collect();
            match cells.as_slice() {
                [old_path, _original, current] if old_path.starts_with(REDIRECT_DIRECTORY) && old_path.ends_with(SKILL_FILE) => {
                    Some(((*old_path).to_owned(), (*current).to_owned()))
                }
                _ => None,
            }
        })
        .collect()
}

fn stray_redirect_file(path: &str) -> Option<Diagnostic> {
    (path.starts_with(REDIRECT_DIRECTORY) && path != STUB_INVENTORY).then(|| {
        Diagnostic::new(
            DiagnosticCategory::StrayRedirectFile,
            format!("`{REDIRECT_DIRECTORY}` holds only the inventoried redirect stubs and `{STUB_INVENTORY}`."),
        )
    })
}

fn missing_redirect_stub() -> Diagnostic {
    Diagnostic::new(
        DiagnosticCategory::MissingRedirectStub,
        format!("`{STUB_INVENTORY}` inventories this path, but it has no redirect stub."),
    )
}

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

    use super::{STUB_INVENTORY, layout_findings, misplaced_skill};
    use crate::DiagnosticCategory;

    const STUB: &str = ".github/skills/dev/planning/create-issue/SKILL.md";

    /// A stub inventory with one row per old `SKILL.md` path, whose skill keeps its folder name.
    fn inventory(old_paths: &[&str]) -> String {
        let rows = old_paths
            .iter()
            .map(|path| {
                let name = path.trim_end_matches("/SKILL.md").rsplit('/').next().unwrap();
                format!("| `{path}` | `{name}` | `{name}` |\n")
            })
            .collect::<Vec<_>>()
            .concat();
        format!("# Moved\n\n| Old path | Original name | Current name |\n| --- | --- | --- |\n{rows}")
    }

    /// The layout findings of `files`, whose stub inventory is `inventory`, as (path, category).
    fn layout(files: &[&str], inventory: &str) -> Vec<(String, DiagnosticCategory)> {
        let inventory = inventory.to_owned();
        layout_findings(files.iter().copied(), |path| {
            (path == STUB_INVENTORY).then(|| inventory.clone())
        })
        .into_iter()
        .map(|finding| (finding.path, finding.diagnostic.category))
        .collect()
    }

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

    #[test]
    fn it_should_accept_an_inventoried_redirect_stub_at_an_old_skill_path() {
        let findings = layout(&[STUB, STUB_INVENTORY], &inventory(&[STUB]));

        assert_eq!(findings, []);
    }

    #[test]
    fn it_should_report_an_inventoried_path_whose_redirect_stub_was_deleted() {
        let findings = layout(&[STUB_INVENTORY], &inventory(&[STUB]));

        assert_eq!(findings, [(STUB.to_owned(), DiagnosticCategory::MissingRedirectStub)]);
    }

    #[test]
    fn it_should_report_a_skill_under_the_old_folder_that_the_inventory_does_not_list() {
        let new_skill = ".github/skills/dev/planning/new-skill/SKILL.md";

        let findings = layout(&[new_skill, STUB, STUB_INVENTORY], &inventory(&[STUB]));

        assert_eq!(findings, [(new_skill.to_owned(), DiagnosticCategory::MisplacedSkill)]);
    }

    #[test]
    fn it_should_report_a_file_under_the_old_folder_that_is_no_redirect_stub() {
        let script = ".github/skills/dev/planning/create-issue/scripts/check.sh";

        let findings = layout(&[STUB, script, STUB_INVENTORY], &inventory(&[STUB]));

        assert_eq!(findings, [(script.to_owned(), DiagnosticCategory::StrayRedirectFile)]);
    }
}
