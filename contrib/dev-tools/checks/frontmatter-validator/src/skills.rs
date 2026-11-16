//! Repository-wide rules for where agent skills live.
//!
//! Claude Code discovers a project skill only at `.claude/skills/<name>/SKILL.md`: a skill anywhere
//! else, or one folder deeper, is never listed to the model. The skills used to live under
//! `.github/skills/`; each old `SKILL.md` path keeps a redirect stub, inventoried in
//! `.github/skills/README.md` (issue #2866). These decisions are pure: they take
//! repository-relative paths with `/` separators and file texts, and never touch the filesystem or
//! git.

use std::collections::{BTreeMap, BTreeSet};

use serde_yaml::Value;

use crate::{Diagnostic, DiagnosticCategory, DocumentOwnership, extract_with_ownership};

/// The only folder Claude Code discovers project skills in.
pub const SKILLS_DIRECTORY: &str = ".claude/skills/";

/// The folder the skills moved out of; it holds only redirect stubs and their inventory.
pub const REDIRECT_DIRECTORY: &str = ".github/skills/";

/// The inventory of the redirect stubs.
pub const STUB_INVENTORY: &str = ".github/skills/README.md";

/// The index that lists every skill by area.
pub const SKILLS_INDEX: &str = ".claude/skills/README.md";

const SKILL_FILE: &str = "SKILL.md";

/// The current name an inventory row gives a skill that was removed.
const REMOVED: &str = "removed";

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
        || path == SKILLS_INDEX
        || (path.starts_with(REDIRECT_DIRECTORY) && is_skill_file(path))
        || skill_name(path).is_some()
}

/// Checks the skill layout of a whole repository snapshot: `files` are every tracked path, and
/// `text` returns the text of a path for which [`reads_text`] is true. Findings are sorted by path.
pub fn layout_findings<'a>(
    files: impl IntoIterator<Item = &'a str>,
    text: impl Fn(&str) -> Option<String>,
) -> Vec<LayoutFinding> {
    let files: BTreeSet<&str> = files.into_iter().collect();
    let text = |path: &str| files.contains(path).then(|| text(path)).flatten();
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
    findings.extend(
        inventory
            .iter()
            .filter(|(path, current)| current.as_str() != REMOVED && files.contains(path.as_str()))
            .filter_map(|(path, current)| {
                invalid_redirect_stub(path, &text(path).unwrap_or_default(), current, &files)
                    .map(|diagnostic| finding(path, diagnostic))
            }),
    );
    findings.extend(files.iter().filter_map(|path| {
        let folder = skill_name(path)?;
        skill_name_mismatch(folder, &text(path).unwrap_or_default()).map(|diagnostic| finding(path, diagnostic))
    }));
    let skills: BTreeSet<&str> = files.iter().filter_map(|path| skill_name(path)).collect();
    findings.extend(
        index_mismatches(&skills, text(SKILLS_INDEX).as_deref())
            .into_iter()
            .map(|diagnostic| finding(SKILLS_INDEX, diagnostic)),
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

/// The index lists every skill folder exactly once, as a `<name>/SKILL.md` link in a table row,
/// and no other.
fn index_mismatches(skills: &BTreeSet<&str>, index: Option<&str>) -> Vec<Diagnostic> {
    let Some(index) = index else {
        return (!skills.is_empty())
            .then(|| index_mismatch(format!("`{SKILLS_INDEX}` is missing; it must list every skill.")))
            .into_iter()
            .collect();
    };
    let mut listed: BTreeMap<&str, usize> = BTreeMap::new();
    let rows: String = index
        .lines()
        .filter(|line| line.starts_with('|'))
        .collect::<Vec<_>>()
        .join("\n");
    for name in link_targets(&rows).into_iter().filter_map(|target| {
        target
            .strip_suffix(&format!("/{SKILL_FILE}"))
            .filter(|name| !name.contains('/'))
    }) {
        *listed.entry(name).or_default() += 1;
    }

    let unlisted = skills
        .iter()
        .filter(|skill| !listed.contains_key(**skill))
        .map(|skill| format!("The index does not list the skill `{skill}`."));
    let repeated = listed
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(name, _)| format!("The index lists `{name}` more than once."));
    let missing = listed
        .keys()
        .filter(|name| !skills.contains(**name))
        .map(|name| format!("The index lists `{name}`, which is no skill under `{SKILLS_DIRECTORY}`."));
    unlisted.chain(repeated).chain(missing).map(index_mismatch).collect()
}

fn index_mismatch(message: String) -> Diagnostic {
    Diagnostic::new(DiagnosticCategory::SkillIndexMismatch, message)
}

/// A skill's `name` must equal its folder name, which is how Claude Code lists it.
fn skill_name_mismatch(folder: &str, markdown: &str) -> Option<Diagnostic> {
    let name = extract_with_ownership(markdown, DocumentOwnership::External)
        .ok()
        .flatten()
        .and_then(|frontmatter| frontmatter.values.get("name").and_then(Value::as_str).map(str::to_owned));
    (name.as_deref() != Some(folder)).then(|| {
        Diagnostic::for_field(
            DiagnosticCategory::SkillNameMismatch,
            "name",
            format!("The skill's `name` must be its folder name, `{folder}`."),
        )
    })
}

/// A stub of a skill that was not removed has no frontmatter and one link, to the skill's current
/// `SKILL.md`, which exists.
fn invalid_redirect_stub(path: &str, text: &str, current: &str, files: &BTreeSet<&str>) -> Option<Diagnostic> {
    let expected = format!("{SKILLS_DIRECTORY}{current}/{SKILL_FILE}");
    let problem = if text.starts_with("---") {
        Some("has frontmatter, so a tool could read it as a skill".to_owned())
    } else {
        match link_targets(text).as_slice() {
            [target]
                if resolve(parent(path), target).as_deref() == Some(expected.as_str()) && files.contains(expected.as_str()) =>
            {
                None
            }
            [_] => Some(format!("must link to `{expected}`, an existing skill")),
            links => Some(format!("must hold exactly one link, not {}", links.len())),
        }
    };
    problem.map(|problem| {
        Diagnostic::new(
            DiagnosticCategory::InvalidRedirectStub,
            format!("This redirect stub {problem}."),
        )
    })
}

/// The targets of the inline Markdown links in `text`, without a title.
fn link_targets(text: &str) -> Vec<&str> {
    text.match_indices("](")
        .filter_map(|(start, _)| {
            let rest = &text[start + 2..];
            rest[..rest.find(')')?].split_whitespace().next()
        })
        .collect()
}

/// The repository path a relative link target names from `directory`, without its fragment.
fn resolve(directory: &str, target: &str) -> Option<String> {
    let mut parts: Vec<&str> = directory.split('/').filter(|part| !part.is_empty()).collect();
    for part in target.split('#').next()?.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    Some(parts.join("/"))
}

fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(directory, _)| directory)
}

fn is_skill_file(path: &str) -> bool {
    path == SKILL_FILE || path.ends_with(&format!("/{SKILL_FILE}"))
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
    (is_skill_file(path) && skill_name(path).is_none()).then(|| {
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

    use std::collections::BTreeMap;

    use rstest::rstest;

    use super::{SKILLS_INDEX, STUB_INVENTORY, layout_findings, misplaced_skill};
    use crate::DiagnosticCategory;

    const SKILL: &str = ".claude/skills/create-issue/SKILL.md";
    const STUB: &str = ".github/skills/dev/planning/create-issue/SKILL.md";

    /// The text of a skill whose frontmatter `name` is `name`.
    fn skill(name: &str) -> String {
        format!("---\nname: {name}\ndescription: Example.\n---\n# Example\n")
    }

    /// The text of a stub at `STUB` that links to the skill `name`.
    fn stub(name: &str) -> String {
        format!("# Moved: `create-issue`\n\nThis skill moved to [`{name}`](../../../../../.claude/skills/{name}/SKILL.md).\n")
    }

    /// A skills index with one row per listed name.
    fn index(names: &[&str]) -> String {
        let rows = names
            .iter()
            .map(|name| format!("| [`{name}`]({name}/SKILL.md) | Example. |\n"))
            .collect::<Vec<_>>()
            .concat();
        format!("# Agent Skills\n\n| Skill | Purpose |\n| --- | --- |\n{rows}")
    }

    /// A stub inventory with one row per `(old path, current name)`.
    fn inventory(rows: &[(&str, &str)]) -> String {
        let rows = rows
            .iter()
            .map(|(path, current)| format!("| `{path}` | `create-issue` | `{current}` |\n"))
            .collect::<Vec<_>>()
            .concat();
        format!("# Moved\n\n| Old path | Original name | Current name |\n| --- | --- | --- |\n{rows}")
    }

    /// The tracked files and the texts the layout rules read.
    struct Snapshot {
        texts: BTreeMap<String, String>,
        files: Vec<String>,
    }

    impl Snapshot {
        /// One moved skill, its stub, the inventory that lists the stub, and the index that lists
        /// the skill.
        fn accepted() -> Self {
            Self {
                texts: BTreeMap::new(),
                files: Vec::new(),
            }
            .with(SKILL, &skill("create-issue"))
            .with(STUB, &stub("create-issue"))
            .with(STUB_INVENTORY, &inventory(&[(STUB, "create-issue")]))
            .with(SKILLS_INDEX, &index(&["create-issue"]))
        }

        fn with(mut self, path: &str, text: &str) -> Self {
            self.files.push(path.to_owned());
            self.texts.insert(path.to_owned(), text.to_owned());
            self
        }

        fn without(mut self, path: &str) -> Self {
            self.files.retain(|file| file != path);
            self
        }

        /// The findings as `(path, category)` pairs.
        fn findings(&self) -> Vec<(String, DiagnosticCategory)> {
            layout_findings(self.files.iter().map(String::as_str), |path| self.texts.get(path).cloned())
                .into_iter()
                .map(|finding| (finding.path, finding.diagnostic.category))
                .collect()
        }
    }

    fn only(path: &str, category: DiagnosticCategory) -> Vec<(String, DiagnosticCategory)> {
        vec![(path.to_owned(), category)]
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
        assert_eq!(Snapshot::accepted().findings(), []);
    }

    #[test]
    fn it_should_report_an_inventoried_path_whose_redirect_stub_was_deleted() {
        let snapshot = Snapshot::accepted().without(STUB);

        assert_eq!(snapshot.findings(), only(STUB, DiagnosticCategory::MissingRedirectStub));
    }

    #[test]
    fn it_should_report_a_skill_under_the_old_folder_that_the_inventory_does_not_list() {
        let new_skill = ".github/skills/dev/planning/new-skill/SKILL.md";

        let snapshot = Snapshot::accepted().with(new_skill, "");

        assert_eq!(snapshot.findings(), only(new_skill, DiagnosticCategory::MisplacedSkill));
    }

    #[test]
    fn it_should_report_a_file_under_the_old_folder_that_is_no_redirect_stub() {
        let script = ".github/skills/dev/planning/create-issue/scripts/check.sh";

        let snapshot = Snapshot::accepted().with(script, "");

        assert_eq!(snapshot.findings(), only(script, DiagnosticCategory::StrayRedirectFile));
    }

    #[rstest]
    #[case::frontmatter(&format!("---\nname: create-issue\n---\n{}", stub("create-issue")))]
    #[case::no_link("# Moved: `create-issue`\n")]
    #[case::two_links(&format!("{}See [the index](../../../../../.claude/skills/README.md).\n", stub("create-issue")))]
    #[case::link_to_another_skill(&stub("create-pr"))]
    fn it_should_report_a_redirect_stub_that_does_not_lead_to_its_skill(#[case] text: &str) {
        let snapshot = Snapshot::accepted().with(STUB, text);

        assert_eq!(snapshot.findings(), only(STUB, DiagnosticCategory::InvalidRedirectStub));
    }

    #[test]
    fn it_should_report_the_redirect_stub_and_the_index_row_of_a_deleted_skill() {
        let snapshot = Snapshot::accepted().without(SKILL);

        assert_eq!(
            snapshot.findings(),
            [
                (SKILLS_INDEX.to_owned(), DiagnosticCategory::SkillIndexMismatch),
                (STUB.to_owned(), DiagnosticCategory::InvalidRedirectStub),
            ]
        );
    }

    #[test]
    fn it_should_accept_a_redirect_stub_that_links_to_the_new_name_of_a_renamed_skill() {
        let renamed = ".claude/skills/open-issue/SKILL.md";

        let snapshot = Snapshot::accepted()
            .without(SKILL)
            .with(renamed, &skill("open-issue"))
            .with(STUB, &stub("open-issue"))
            .with(STUB_INVENTORY, &inventory(&[(STUB, "open-issue")]))
            .with(SKILLS_INDEX, &index(&["open-issue"]));

        assert_eq!(snapshot.findings(), []);
    }

    #[test]
    fn it_should_accept_a_redirect_stub_of_a_removed_skill_without_a_link() {
        let snapshot = Snapshot::accepted()
            .without(SKILL)
            .with(STUB, "# Removed: `create-issue`\n\nThis skill was removed.\n")
            .with(STUB_INVENTORY, &inventory(&[(STUB, "removed")]))
            .with(SKILLS_INDEX, &index(&[]));

        assert_eq!(snapshot.findings(), []);
    }

    #[rstest]
    #[case::another_name(&skill("open-issue"))]
    #[case::no_name("---\ndescription: Example.\n---\n")]
    #[case::no_frontmatter("# Example\n")]
    fn it_should_report_a_skill_whose_name_is_not_its_folder_name(#[case] text: &str) {
        let snapshot = Snapshot::accepted().with(SKILL, text);

        assert_eq!(snapshot.findings(), only(SKILL, DiagnosticCategory::SkillNameMismatch));
    }

    #[rstest]
    #[case::unlisted_skill(&[".claude/skills/open-pr/SKILL.md"], &["create-issue"])]
    #[case::listed_twice(&[], &["create-issue", "create-issue"])]
    #[case::listed_but_missing(&[], &["create-issue", "open-pr"])]
    fn it_should_report_an_index_that_does_not_list_every_skill_exactly_once(
        #[case] new_skills: &[&str],
        #[case] listed: &[&str],
    ) {
        let snapshot = new_skills
            .iter()
            .fold(Snapshot::accepted(), |snapshot, path| snapshot.with(path, &skill("open-pr")))
            .with(SKILLS_INDEX, &index(listed));

        assert_eq!(
            snapshot.findings(),
            only(SKILLS_INDEX, DiagnosticCategory::SkillIndexMismatch)
        );
    }

    #[test]
    fn it_should_report_a_missing_index() {
        let snapshot = Snapshot::accepted().without(SKILLS_INDEX);

        assert_eq!(
            snapshot.findings(),
            only(SKILLS_INDEX, DiagnosticCategory::SkillIndexMismatch)
        );
    }

    #[test]
    fn it_should_not_require_an_index_when_there_is_no_skill() {
        let findings = layout_findings([".github/skills/README.md"], |_| Some(String::new()));

        assert_eq!(findings, []);
    }

    #[test]
    fn it_should_count_only_the_index_table_rows() {
        let prose = format!("See [`create-issue`](create-issue/SKILL.md).\n\n{}", index(&["create-issue"]));

        let snapshot = Snapshot::accepted().with(SKILLS_INDEX, &prose);

        assert_eq!(snapshot.findings(), []);
    }
}
