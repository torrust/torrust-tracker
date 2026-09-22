//! Canonical strict frontmatter profile types and structural validation.

use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use crate::{Diagnostic, DiagnosticCategory, Frontmatter, SemanticLinks};

/// A recognized frontmatter profile.
#[derive(Debug, Eq, PartialEq)]
pub enum Profile {
    /// A strict schema-version one issue profile.
    Issue(Issue),
    /// A strict schema-version one EPIC profile.
    Epic(Epic),
    /// A legacy, unknown, or otherwise permissive frontmatter profile.
    Permissive,
}

/// The canonical strict issue frontmatter model.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Issue {
    /// The strict contract version.
    pub schema_version: u8,
    /// The recognized issue document class.
    pub doc_type: IssueDocumentType,
    /// The issue classification.
    pub issue_type: IssueType,
    /// The current lifecycle state.
    pub status: IssueStatus,
    /// The issue urgency.
    pub priority: Priority,
    /// The optional positive parent EPIC issue number.
    pub epic: Option<u64>,
    /// The optional positive GitHub issue number.
    pub github_issue: Option<u64>,
    /// The repository-relative specification path.
    pub spec_path: String,
    /// The development branch name.
    pub branch: String,
    /// The optional positive related pull request number.
    pub related_pr: Option<u64>,
    /// The required UTC-minute update timestamp.
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

/// The canonical strict EPIC frontmatter model.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Epic {
    /// The strict contract version.
    pub schema_version: u8,
    /// The recognized EPIC document class.
    pub doc_type: EpicDocumentType,
    /// The current lifecycle state.
    pub status: IssueStatus,
    /// The optional positive parent EPIC issue number.
    pub epic: Option<u64>,
    /// The optional positive GitHub issue number.
    pub github_issue: Option<u64>,
    /// The repository-relative specification path.
    pub spec_path: String,
    /// The optional owner of the EPIC.
    pub epic_owner: Option<String>,
    /// The required UTC-minute update timestamp.
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

/// The strict semantic-link envelope with frozen v1 reference value types.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct StrictSemanticLinks {
    /// Optional validated repository skill names.
    pub skill_links: Option<Vec<SkillName>>,
    /// Optional validated repository paths or typed references.
    pub related_artifacts: Option<Vec<RelatedArtifact>>,
}

/// A validated repository skill name.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct SkillName(String);

/// A validated v1 related-artifact reference.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct RelatedArtifact(String);

impl TryFrom<String> for SkillName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_skill_name(&value) {
            Ok(Self(value))
        } else {
            Err(format!("`{value}` must match [a-z0-9]+(-[a-z0-9]+)*."))
        }
    }
}

impl TryFrom<String> for RelatedArtifact {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if is_related_artifact(&value) {
            Ok(Self(value))
        } else {
            Err(format!("`{value}` is not an approved v1 related-artifact reference."))
        }
    }
}

/// The fixed document type for strict issue records.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum IssueDocumentType {
    /// The issue document class.
    #[serde(rename = "issue")]
    Issue,
}

/// The fixed document type for strict EPIC records.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum EpicDocumentType {
    /// The EPIC document class.
    #[serde(rename = "epic")]
    Epic,
}

/// The allowed issue classifications.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IssueType {
    /// A routine unit of planned work.
    Task,
    /// A behavior defect.
    Bug,
    /// A new user-visible capability.
    Feature,
    /// An improvement to an existing capability.
    Enhancement,
}

/// The shared issue and EPIC lifecycle values.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum IssueStatus {
    /// Work is being drafted.
    Draft,
    /// Work is ready to start.
    Planned,
    /// Work is underway.
    InProgress,
    /// Work cannot proceed.
    Blocked,
    /// Work awaits review.
    InReview,
    /// Work is complete.
    Done,
}

/// The allowed issue urgency levels.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum Priority {
    /// Highest urgency.
    #[serde(rename = "p0")]
    P0,
    /// High urgency.
    #[serde(rename = "p1")]
    P1,
    /// Normal urgency.
    #[serde(rename = "p2")]
    P2,
    /// Lowest urgency.
    #[serde(rename = "p3")]
    P3,
}

/// Validates a parsed frontmatter block against the strict v1 profile boundary.
///
/// Legacy and unknown document classes are deliberately permissive.
///
/// # Errors
///
/// Returns a diagnostic when a strict v1 issue or EPIC record has an unknown field, omits a
/// required field, uses the wrong scalar type, or violates an allowed-value or field invariant.
pub fn validate(frontmatter: &Frontmatter) -> Result<Profile, Diagnostic> {
    let Some(doc_type) = strict_document_type(&frontmatter.values)? else {
        return Ok(Profile::Permissive);
    };

    match doc_type {
        "issue" => {
            validate_issue(&frontmatter.values, &frontmatter.yaml, frontmatter.semantic_links.as_ref()).map(Profile::Issue)
        }
        "epic" => validate_epic(&frontmatter.values, &frontmatter.yaml, frontmatter.semantic_links.as_ref()).map(Profile::Epic),
        _ => Ok(Profile::Permissive),
    }
}

fn strict_document_type(values: &Mapping) -> Result<Option<&str>, Diagnostic> {
    let Some(schema_version) = values.get(field_key("schema-version")) else {
        return Ok(None);
    };
    let doc_type = values.get(field_key("doc-type")).and_then(Value::as_str);
    let is_strict_profile_candidate = matches!(doc_type, Some("issue" | "epic"));
    let Some(schema_version) = schema_version.as_i64() else {
        if is_strict_profile_candidate {
            return Err(Diagnostic {
                category: DiagnosticCategory::WrongScalarType,
                message: String::from("`schema-version` must be a YAML integer."),
            });
        }

        return Ok(None);
    };
    if schema_version != 1 {
        return Ok(None);
    }

    let Some(doc_type) = values.get(field_key("doc-type")) else {
        return Err(Diagnostic {
            category: DiagnosticCategory::MissingRequiredField,
            message: String::from("Strict frontmatter requires `doc-type`."),
        });
    };
    let Some(doc_type) = doc_type.as_str() else {
        return Err(Diagnostic {
            category: DiagnosticCategory::WrongScalarType,
            message: String::from("`doc-type` must be a YAML string."),
        });
    };

    Ok(Some(doc_type))
}

fn validate_issue(values: &Mapping, yaml: &str, semantic_links: Option<&SemanticLinks>) -> Result<Issue, Diagnostic> {
    validate_known_fields(values, ISSUE_FIELDS)?;
    validate_required_fields(values, ISSUE_FIELDS)?;
    validate_allowed_string(values, "issue-type", &["task", "bug", "feature", "enhancement"])?;
    validate_allowed_string(
        values,
        "status",
        &["draft", "planned", "in-progress", "blocked", "in-review", "done"],
    )?;
    validate_allowed_string(values, "priority", &["p0", "p1", "p2", "p3"])?;
    validate_reference_syntax(semantic_links)?;
    let issue = deserialize_strict(values)?;
    validate_issue_invariants(&issue, yaml)?;

    Ok(issue)
}

fn validate_epic(values: &Mapping, yaml: &str, semantic_links: Option<&SemanticLinks>) -> Result<Epic, Diagnostic> {
    validate_known_fields(values, EPIC_FIELDS)?;
    validate_required_fields(values, EPIC_FIELDS)?;
    validate_allowed_string(
        values,
        "status",
        &["draft", "planned", "in-progress", "blocked", "in-review", "done"],
    )?;
    validate_reference_syntax(semantic_links)?;
    let epic = deserialize_strict(values)?;
    validate_epic_invariants(&epic, yaml)?;

    Ok(epic)
}

fn validate_reference_syntax(semantic_links: Option<&SemanticLinks>) -> Result<(), Diagnostic> {
    let semantic_links = semantic_links.expect("strict profiles require `semantic-links`");
    for skill_link in semantic_links.skill_links.as_deref().unwrap_or_default() {
        if !is_skill_name(skill_link) {
            return Err(invalid_reference_syntax(format!(
                "`skill-links` entry `{skill_link}` must match [a-z0-9]+(-[a-z0-9]+)*."
            )));
        }
    }
    for artifact in semantic_links.related_artifacts.as_deref().unwrap_or_default() {
        if !is_related_artifact(artifact) {
            return Err(invalid_reference_syntax(format!(
                "`related-artifacts` entry `{artifact}` is not an approved v1 reference."
            )));
        }
    }

    Ok(())
}

fn is_skill_name(value: &str) -> bool {
    is_lowercase_identifier(value)
}

fn is_related_artifact(value: &str) -> bool {
    is_repository_relative_path(value) || is_issue_reference(value) || is_review_finding(value)
}

fn is_repository_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains(':')
        && !value.contains('#')
        && !value.chars().any(char::is_whitespace)
        && !value.split('/').any(|component| matches!(component, "" | "." | ".."))
}

fn is_issue_reference(value: &str) -> bool {
    value.strip_prefix("issue #").is_some_and(is_positive_integer)
}

fn is_review_finding(value: &str) -> bool {
    let Some(value) = value.strip_prefix("review-finding:pr-") else {
        return false;
    };
    let Some((pull_request, identifier)) = value.split_once('-') else {
        return false;
    };

    is_positive_integer(pull_request) && is_lowercase_identifier(identifier)
}

fn is_positive_integer(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()) && value != "0"
}

fn is_lowercase_identifier(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn validate_issue_invariants(issue: &Issue, yaml: &str) -> Result<(), Diagnostic> {
    validate_optional_positive_integer("epic", issue.epic)?;
    validate_optional_positive_integer("github-issue", issue.github_issue)?;
    validate_optional_positive_integer("related-pr", issue.related_pr)?;
    validate_repository_relative_path("spec-path", &issue.spec_path)?;
    validate_non_empty_string("branch", &issue.branch)?;
    validate_utc_minute_string(&issue.last_updated_utc, yaml)
}

fn validate_epic_invariants(epic: &Epic, yaml: &str) -> Result<(), Diagnostic> {
    validate_optional_positive_integer("epic", epic.epic)?;
    validate_optional_positive_integer("github-issue", epic.github_issue)?;
    validate_repository_relative_path("spec-path", &epic.spec_path)?;
    if let Some(owner) = &epic.epic_owner {
        validate_non_empty_string("epic-owner", owner)?;
    }
    validate_utc_minute_string(&epic.last_updated_utc, yaml)
}

fn validate_optional_positive_integer(field: &str, value: Option<u64>) -> Result<(), Diagnostic> {
    if value == Some(0) {
        return Err(invalid_field_value(format!("`{field}` must be a positive integer or null.")));
    }

    Ok(())
}

fn validate_non_empty_string(field: &str, value: &str) -> Result<(), Diagnostic> {
    if value.is_empty() {
        return Err(invalid_field_value(format!("`{field}` must not be empty.")));
    }

    Ok(())
}

fn validate_repository_relative_path(field: &str, value: &str) -> Result<(), Diagnostic> {
    if !is_repository_relative_path(value) {
        return Err(invalid_field_value(format!("`{field}` must be a repository-relative path.")));
    }

    Ok(())
}

fn validate_utc_minute_string(value: &str, yaml: &str) -> Result<(), Diagnostic> {
    let bytes = value.as_bytes();
    let has_expected_separators = bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes.get(10) == Some(&b' ')
        && bytes.get(13) == Some(&b':');
    let has_ascii_digits = bytes
        .iter()
        .enumerate()
        .filter(|(index, _)| !matches!(index, 4 | 7 | 10 | 13))
        .all(|(_, byte)| byte.is_ascii_digit());
    if bytes.len() != 16
        || !has_expected_separators
        || !has_ascii_digits
        || !is_valid_utc_minute(value)
        || !has_double_quoted_timestamp(yaml)
    {
        return Err(invalid_field_value(String::from(
            "`last-updated-utc` must use the YYYY-MM-DD HH:MM UTC-minute format.",
        )));
    }

    Ok(())
}

fn has_double_quoted_timestamp(yaml: &str) -> bool {
    yaml.lines()
        .find_map(|line| line.strip_prefix("last-updated-utc:").map(str::trim))
        .is_some_and(|value| value.starts_with('"') && value.ends_with('"'))
}

fn is_valid_utc_minute(value: &str) -> bool {
    let year = value[0..4].parse::<u16>().expect("timestamp digits were checked first");
    let month = value[5..7].parse::<u8>().expect("timestamp digits were checked first");
    let day = value[8..10].parse::<u8>().expect("timestamp digits were checked first");
    let hour = value[11..13].parse::<u8>().expect("timestamp digits were checked first");
    let minute = value[14..16].parse::<u8>().expect("timestamp digits were checked first");
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return false,
    };

    (1..=days_in_month).contains(&day) && hour < 24 && minute < 60
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

const fn invalid_field_value(message: String) -> Diagnostic {
    Diagnostic {
        category: DiagnosticCategory::InvalidFieldValue,
        message,
    }
}

const fn invalid_reference_syntax(message: String) -> Diagnostic {
    Diagnostic {
        category: DiagnosticCategory::InvalidReferenceSyntax,
        message,
    }
}

fn validate_known_fields(values: &Mapping, allowed_fields: &[&str]) -> Result<(), Diagnostic> {
    for key in values.keys() {
        let Some(field) = key.as_str() else {
            return Err(Diagnostic {
                category: DiagnosticCategory::UnknownField,
                message: String::from("Strict frontmatter field names must be strings."),
            });
        };
        if !allowed_fields.contains(&field) && !field.starts_with("x-") {
            return Err(Diagnostic {
                category: DiagnosticCategory::UnknownField,
                message: format!("`{field}` is not allowed by this strict frontmatter profile."),
            });
        }
    }

    Ok(())
}

fn validate_required_fields(values: &Mapping, required_fields: &[&str]) -> Result<(), Diagnostic> {
    for field in required_fields {
        if !values.contains_key(field_key(field)) {
            return Err(Diagnostic {
                category: DiagnosticCategory::MissingRequiredField,
                message: format!("Strict frontmatter requires `{field}`."),
            });
        }
    }

    Ok(())
}

fn validate_allowed_string(values: &Mapping, field: &str, allowed_values: &[&str]) -> Result<(), Diagnostic> {
    let value = values.get(field_key(field)).expect("required fields were checked first");
    let Some(value) = value.as_str() else {
        return Err(Diagnostic {
            category: DiagnosticCategory::WrongScalarType,
            message: format!("`{field}` must be a YAML string."),
        });
    };
    if !allowed_values.contains(&value) {
        return Err(Diagnostic {
            category: DiagnosticCategory::InvalidAllowedValue,
            message: format!("`{field}` has an unsupported value `{value}`."),
        });
    }

    Ok(())
}

fn deserialize_strict<T>(values: &Mapping) -> Result<T, Diagnostic>
where
    T: for<'de> Deserialize<'de>,
{
    let mut values = values.clone();
    values.retain(|key, _| !key.as_str().is_some_and(|field| field.starts_with("x-")));
    serde_yaml::from_value(Value::Mapping(values)).map_err(|error| Diagnostic {
        category: DiagnosticCategory::WrongScalarType,
        message: error.to_string(),
    })
}

fn field_key(field: &str) -> Value {
    Value::String(String::from(field))
}

const ISSUE_FIELDS: &[&str] = &[
    "schema-version",
    "doc-type",
    "issue-type",
    "status",
    "priority",
    "epic",
    "github-issue",
    "spec-path",
    "branch",
    "related-pr",
    "last-updated-utc",
    "semantic-links",
];

const EPIC_FIELDS: &[&str] = &[
    "schema-version",
    "doc-type",
    "status",
    "epic",
    "github-issue",
    "spec-path",
    "epic-owner",
    "last-updated-utc",
    "semantic-links",
];
