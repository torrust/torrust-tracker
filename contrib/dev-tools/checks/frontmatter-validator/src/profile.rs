//! Canonical strict frontmatter profile types and structural validation.

use schemars::JsonSchema;
use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use crate::{Diagnostic, DiagnosticCategory, DocumentOwnership, Frontmatter, SemanticLinks};

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

#[derive(Debug, Eq, PartialEq)]
enum StrictProfileKind {
    Issue,
    Epic,
}

impl StrictProfileKind {
    fn from_doc_type(doc_type: &str) -> Option<Self> {
        match doc_type {
            "issue" => Some(Self::Issue),
            "epic" => Some(Self::Epic),
            _ => None,
        }
    }

    const fn definition(&self) -> &'static StrictProfileDefinition {
        match self {
            Self::Issue => &ISSUE_PROFILE,
            Self::Epic => &EPIC_PROFILE,
        }
    }
}

/// The strict frontmatter document profiles represented by the v1 JSON Schema.
#[derive(JsonSchema)]
#[schemars(title = "Torrust Tracker Frontmatter V1")]
#[serde(untagged)]
pub enum FrontmatterV1 {
    /// The strict issue profile.
    Issue(Issue),
    /// The strict EPIC profile.
    Epic(Epic),
}

/// Generates the JSON Schema Draft 2020-12 projection of the strict v1 profiles.
#[must_use]
pub fn v1_schema() -> schemars::Schema {
    schemars::schema_for!(FrontmatterV1)
}

/// The exact bytes of the tracked `docs/schemas/frontmatter-v1.schema.json` artifact:
/// pretty-printed JSON with a single trailing newline.
///
/// # Panics
///
/// Panics if the schema cannot be serialized, which cannot happen: a `Schema` is plain JSON data.
#[must_use]
pub fn v1_schema_json() -> String {
    let schema = serde_json::to_string_pretty(&v1_schema()).expect("schemars::Schema serializes as JSON");
    format!("{schema}\n")
}

/// The canonical strict issue frontmatter model.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(extend("patternProperties" = { "^x-": {} }))]
pub struct Issue {
    /// The strict contract version.
    #[schemars(range(min = 1, max = 1))]
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
    #[schemars(range(min = 1))]
    pub epic: Option<u64>,
    /// The optional positive GitHub issue number.
    #[schemars(range(min = 1))]
    pub github_issue: Option<u64>,
    /// The repository-relative specification path.
    #[schemars(regex(pattern = r"^(?!.*(?:^|/)\.{1,2}(?:/|$))[^\s/:#]+(?:/[^\s/:#]+)*$"))]
    pub spec_path: String,
    /// The development branch name.
    #[schemars(length(min = 1))]
    pub branch: String,
    /// The optional positive related pull request number.
    #[schemars(range(min = 1))]
    pub related_pr: Option<u64>,
    /// The required UTC-minute update timestamp.
    #[schemars(regex(pattern = r"^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}$"))]
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

impl Issue {
    fn validate_invariants(&self, yaml: &str) -> Result<(), Diagnostic> {
        validate_optional_positive_integer("epic", self.epic)?;
        validate_optional_positive_integer("github-issue", self.github_issue)?;
        validate_optional_positive_integer("related-pr", self.related_pr)?;
        validate_repository_relative_path("spec-path", &self.spec_path)?;
        validate_non_empty_string("branch", &self.branch)?;
        validate_utc_minute_string(&self.last_updated_utc, yaml)
    }
}

/// The canonical strict EPIC frontmatter model.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(extend("patternProperties" = { "^x-": {} }))]
pub struct Epic {
    /// The strict contract version.
    #[schemars(range(min = 1, max = 1))]
    pub schema_version: u8,
    /// The recognized EPIC document class.
    pub doc_type: EpicDocumentType,
    /// The current lifecycle state.
    pub status: IssueStatus,
    /// The optional positive parent EPIC issue number.
    #[schemars(range(min = 1))]
    pub epic: Option<u64>,
    /// The optional positive GitHub issue number.
    #[schemars(range(min = 1))]
    pub github_issue: Option<u64>,
    /// The repository-relative specification path.
    #[schemars(regex(pattern = r"^(?!.*(?:^|/)\.{1,2}(?:/|$))[^\s/:#]+(?:/[^\s/:#]+)*$"))]
    pub spec_path: String,
    /// The optional owner of the EPIC.
    #[schemars(length(min = 1))]
    pub epic_owner: Option<String>,
    /// The required UTC-minute update timestamp.
    #[schemars(regex(pattern = r"^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}$"))]
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

impl Epic {
    fn validate_invariants(&self, yaml: &str) -> Result<(), Diagnostic> {
        validate_optional_positive_integer("epic", self.epic)?;
        validate_optional_positive_integer("github-issue", self.github_issue)?;
        validate_repository_relative_path("spec-path", &self.spec_path)?;
        if let Some(owner) = &self.epic_owner {
            validate_non_empty_string("epic-owner", owner)?;
        }
        validate_utc_minute_string(&self.last_updated_utc, yaml)
    }
}

/// The strict semantic-link envelope with frozen v1 reference value types.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct StrictSemanticLinks {
    /// Optional validated repository skill names.
    pub skill_links: Option<Vec<SkillName>>,
    /// Optional validated repository paths or typed references.
    pub related_artifacts: Option<Vec<RelatedArtifact>>,
}

/// A validated repository skill name.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(try_from = "String")]
#[schemars(extend("pattern" = r"^[a-z0-9]+(?:-[a-z0-9]+)*$"))]
pub struct SkillName(String);

/// A validated v1 related-artifact reference.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(try_from = "String")]
#[schemars(extend("pattern" = r"^(?:(?!.*(?:^|/)\.{1,2}(?:/|$))[^\s/:#]+(?:/[^\s/:#]+)*|issue #[1-9][0-9]*|review-finding:pr-[1-9][0-9]*-[a-z0-9]+(?:-[a-z0-9]+)*)$"))]
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
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
pub enum IssueDocumentType {
    /// The issue document class.
    #[serde(rename = "issue")]
    Issue,
}

/// The fixed document type for strict EPIC records.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
pub enum EpicDocumentType {
    /// The EPIC document class.
    #[serde(rename = "epic")]
    Epic,
}

/// The allowed issue classifications.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
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
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
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
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
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
    if frontmatter.ownership == DocumentOwnership::External {
        return Ok(Profile::Permissive);
    }

    let Some(doc_type) = strict_document_type(&frontmatter.values)? else {
        return Ok(Profile::Permissive);
    };

    match doc_type {
        StrictProfileKind::Issue => validate_issue(
            &frontmatter.values,
            &frontmatter.yaml,
            frontmatter.semantic_links.as_ref(),
            doc_type.definition(),
        )
        .map(Profile::Issue),
        StrictProfileKind::Epic => validate_epic(
            &frontmatter.values,
            &frontmatter.yaml,
            frontmatter.semantic_links.as_ref(),
            doc_type.definition(),
        )
        .map(Profile::Epic),
    }
}

fn strict_document_type(values: &Mapping) -> Result<Option<StrictProfileKind>, Diagnostic> {
    let Some(schema_version) = values.get(field_key("schema-version")) else {
        return Ok(None);
    };
    let profile_kind = values
        .get(field_key("doc-type"))
        .and_then(Value::as_str)
        .and_then(StrictProfileKind::from_doc_type);
    let Some(schema_version) = schema_version.as_i64() else {
        if profile_kind.is_some() {
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

    Ok(StrictProfileKind::from_doc_type(doc_type))
}

fn validate_issue(
    values: &Mapping,
    yaml: &str,
    semantic_links: Option<&SemanticLinks>,
    definition: &StrictProfileDefinition,
) -> Result<Issue, Diagnostic> {
    validate_strict_profile(values, yaml, semantic_links, definition, Issue::validate_invariants)
}

fn validate_epic(
    values: &Mapping,
    yaml: &str,
    semantic_links: Option<&SemanticLinks>,
    definition: &StrictProfileDefinition,
) -> Result<Epic, Diagnostic> {
    validate_strict_profile(values, yaml, semantic_links, definition, Epic::validate_invariants)
}

fn validate_strict_profile<T>(
    values: &Mapping,
    yaml: &str,
    semantic_links: Option<&SemanticLinks>,
    definition: &StrictProfileDefinition,
    validate_invariants: impl FnOnce(&T, &str) -> Result<(), Diagnostic>,
) -> Result<T, Diagnostic>
where
    T: for<'de> Deserialize<'de>,
{
    definition.validate_structure(values)?;
    validate_reference_syntax(semantic_links)?;
    let profile = deserialize_strict(values)?;
    validate_invariants(&profile, yaml)?;

    Ok(profile)
}

fn validate_reference_syntax(semantic_links: Option<&SemanticLinks>) -> Result<(), Diagnostic> {
    let Some(semantic_links) = semantic_links else {
        return Err(Diagnostic {
            category: DiagnosticCategory::MissingRequiredField,
            message: String::from("Strict profiles require a `semantic-links` mapping."),
        });
    };
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
        && !value.contains('\\')
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
        && value
            .split('-')
            .all(|segment| !segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
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
    if !has_utc_minute_layout(value) || !is_valid_utc_minute_calendar(value) || !has_double_quoted_timestamp(yaml) {
        return Err(invalid_field_value(String::from(
            "`last-updated-utc` must be a double-quoted YAML string in YYYY-MM-DD HH:MM UTC-minute format.",
        )));
    }

    Ok(())
}

fn has_utc_minute_layout(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 16
        && bytes.get(4) == Some(&b'-')
        && bytes.get(7) == Some(&b'-')
        && bytes.get(10) == Some(&b' ')
        && bytes.get(13) == Some(&b':')
        && bytes
            .iter()
            .enumerate()
            .filter(|(index, _)| !matches!(index, 4 | 7 | 10 | 13))
            .all(|(_, byte)| byte.is_ascii_digit())
}

fn has_double_quoted_timestamp(yaml: &str) -> bool {
    yaml.lines()
        .find_map(|line| line.strip_prefix("last-updated-utc:").map(str::trim))
        .map(strip_yaml_comment)
        .is_some_and(|value| value.starts_with('"') && value.ends_with('"'))
}

fn strip_yaml_comment(value: &str) -> &str {
    value
        .char_indices()
        .find_map(|(index, character)| {
            (character == '#' && value[..index].chars().last().is_some_and(char::is_whitespace)).then_some(index)
        })
        .map_or(value, |index| value[..index].trim_end())
}

fn is_valid_utc_minute_calendar(value: &str) -> bool {
    debug_assert!(has_utc_minute_layout(value));
    let year = value[0..4].parse::<u16>().expect("UTC-minute layout contains ASCII digits");
    let month = value[5..7].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let day = value[8..10].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let hour = value[11..13].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
    let minute = value[14..16].parse::<u8>().expect("UTC-minute layout contains ASCII digits");
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

struct StrictProfileDefinition {
    fields: &'static [&'static str],
    allowed_values: &'static [(&'static str, &'static [&'static str])],
}

impl StrictProfileDefinition {
    fn validate_structure(&self, values: &Mapping) -> Result<(), Diagnostic> {
        validate_known_fields(values, self.fields)?;
        validate_required_fields(values, self.fields)?;
        for (field, allowed_values) in self.allowed_values {
            validate_allowed_string(values, field, allowed_values)?;
        }

        Ok(())
    }
}

const STATUS_VALUES: &[&str] = &["draft", "planned", "in-progress", "blocked", "in-review", "done"];

const ISSUE_PROFILE: StrictProfileDefinition = StrictProfileDefinition {
    fields: ISSUE_FIELDS,
    allowed_values: &[
        ("issue-type", &["task", "bug", "feature", "enhancement"]),
        ("status", STATUS_VALUES),
        ("priority", &["p0", "p1", "p2", "p3"]),
    ],
};

const EPIC_PROFILE: StrictProfileDefinition = StrictProfileDefinition {
    fields: EPIC_FIELDS,
    allowed_values: &[("status", STATUS_VALUES)],
};

#[cfg(test)]
mod tests {
    // Owns strict-profile recognition, schema projection, invariants, and reference syntax decisions.
    use serde_json::Value as JsonValue;

    use super::*;
    use crate::{DiagnosticCategory, DocumentOwnership, extract, extract_with_ownership};

    #[test]
    fn it_should_encode_the_schema_artifact_as_deterministic_newline_terminated_json() {
        // Arrange: the canonical strict profile model is unchanged between encodings.

        // Act: encode the artifact bytes twice.
        let first = v1_schema_json();
        let second = v1_schema_json();

        // Assert: both encodings are identical, parse as the same schema, and end with one newline.
        assert_eq!(first, second);
        assert_eq!(
            serde_json::from_str::<JsonValue>(&first).unwrap(),
            serde_json::to_value(v1_schema()).unwrap()
        );
        assert!(first.ends_with('\n') && !first.ends_with("\n\n"));
    }

    #[test]
    fn it_should_generate_a_draft_2020_12_schema_for_the_strict_v1_profiles() {
        // Arrange: the canonical strict issue and EPIC models define the v1 contract.

        // Act: generate their JSON Schema projection.
        let schema = serde_json::to_value(v1_schema()).unwrap();

        // Assert: the root declares Draft 2020-12 and exposes exactly the two strict profiles.
        assert_eq!(
            schema["$schema"],
            JsonValue::String(String::from("https://json-schema.org/draft/2020-12/schema"))
        );
        assert_eq!(schema["anyOf"].as_array().map(Vec::len), Some(2));
        assert!(schema["$defs"].get("Issue").is_some());
        assert!(schema["$defs"].get("Epic").is_some());
        assert_eq!(schema["$defs"]["Issue"]["properties"]["schema-version"]["minimum"], 1);
        assert!(schema["$defs"]["Issue"]["patternProperties"].get("^x-").is_some());
        assert!(schema["$defs"]["Epic"]["patternProperties"].get("^x-").is_some());
        assert_eq!(schema["$defs"]["SkillName"]["pattern"], "^[a-z0-9]+(?:-[a-z0-9]+)*$");
        assert_eq!(
            schema["$defs"]["RelatedArtifact"]["pattern"],
            "^(?:(?!.*(?:^|/)\\.{1,2}(?:/|$))[^\\s/:#]+(?:/[^\\s/:#]+)*|issue #[1-9][0-9]*|review-finding:pr-[1-9][0-9]*-[a-z0-9]+(?:-[a-z0-9]+)*)$"
        );
    }
    #[test]
    fn it_should_keep_an_external_v1_looking_document_permissive() {
        // Arrange: an external schema happens to use strict-profile field names.
        let markdown = "---\nname: external-record\ndescription: An externally governed record.\nschema-version: 1\ndoc-type: issue\nmetadata:\n  semantic-links:\n    skill-links:\n      - write-markdown-docs\n---\n# External\n";
        let frontmatter = extract_with_ownership(markdown, DocumentOwnership::External)
            .unwrap()
            .unwrap();

        // Act: validate the extracted document profile.
        let profile = validate(&frontmatter).unwrap();

        // Assert: ownership prevents strict repository-profile interpretation.
        assert!(matches!(profile, Profile::Permissive));
    }

    #[test]
    fn it_should_keep_an_unknown_v1_document_type_permissive() {
        // Arrange: a repository-owned version-one document uses an unrecognized document class.
        let markdown = "---\nschema-version: 1\ndoc-type: note\n---\n# Note\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify the parsed frontmatter profile.
        let profile = validate(&frontmatter).unwrap();

        // Assert: unrecognized document classes remain outside the strict v1 profiles.
        assert_eq!(profile, Profile::Permissive);
    }

    #[test]
    fn it_should_classify_the_accepted_issue_fixture_as_a_strict_issue_profile() {
        // Arrange: the predecessor's accepted issue fixture declares schema version one.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/accepted/issue.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict issue contract.
        assert!(matches!(profile, Profile::Issue(_)));
    }

    #[test]
    fn it_should_classify_the_accepted_epic_fixture_as_a_strict_epic_profile() {
        // Arrange: the predecessor's accepted EPIC fixture declares schema version one.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/accepted/epic.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict EPIC contract.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_reject_the_wrong_scalar_fixture() {
        // Arrange: the predecessor fixture quotes a positive-integer issue identifier.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-wrong-scalar.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic distinguishes the exact scalar-type violation.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_the_unknown_field_fixture() {
        // Arrange: the predecessor fixture introduces an unprefixed strict-profile field.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-unknown-field.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the unsupported field.
        assert_eq!(error.category, DiagnosticCategory::UnknownField);
    }

    #[test]
    fn it_should_reject_the_invalid_status_fixture() {
        // Arrange: the predecessor fixture uses a status outside the issue lifecycle enum.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-invalid-status.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the disallowed lifecycle value.
        assert_eq!(error.category, DiagnosticCategory::InvalidAllowedValue);
    }

    #[test]
    fn it_should_prioritize_unknown_fields_before_reference_syntax_for_strict_profiles() {
        // Arrange: each strict profile has an unknown field and an invalid semantic-link value.
        let issue = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - Invalid\nfuture-field: value\n---\n# Issue\n";
        let epic = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - Invalid\nfuture-field: value\n---\n# EPIC\n";

        // Act: validate both strict profiles.
        let issue_error = validate(&extract(issue).unwrap().unwrap()).unwrap_err();
        let epic_error = validate(&extract(epic).unwrap().unwrap()).unwrap_err();

        // Assert: profile-specific contracts retain common structural diagnostic precedence.
        assert_eq!(issue_error.category, DiagnosticCategory::UnknownField);
        assert_eq!(epic_error.category, DiagnosticCategory::UnknownField);
    }

    #[test]
    fn it_should_keep_a_legacy_issue_record_permissive() {
        // Arrange: an issue record has no v1 schema version and an otherwise incomplete shape.
        let markdown = "---\ndoc-type: issue\nstatus: planned\n---\n# Legacy issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the legacy record.
        let profile = validate(&frontmatter).unwrap();

        // Assert: v1 strict requirements do not rewrite historical compatibility behavior.
        assert_eq!(profile, Profile::Permissive);
    }

    #[test]
    fn it_should_reject_a_non_positive_strict_issue_identifier() {
        // Arrange: a v1 issue record supplies zero for a positive-only GitHub issue identifier.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 0\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:05\"\nsemantic-links: {}\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the positive-integer invariant.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_an_invalid_strict_timestamp_shape() {
        // Arrange: a v1 EPIC record has a timestamp outside the UTC-minute format.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026/09/21\"\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the required UTC-minute string format.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_an_unquoted_strict_timestamp() {
        // Arrange: a v1 EPIC record uses a valid-looking but plain YAML timestamp scalar.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: 2026-09-21 20:35\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the strict contract requires a double-quoted timestamp string.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_accept_a_quoted_strict_timestamp_with_a_trailing_comment() {
        // Arrange: a v1 EPIC record annotates its quoted timestamp with a YAML comment.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-09-21 20:35\" # updated\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let profile = validate(&frontmatter).unwrap();

        // Assert: comments do not change the quoted scalar's v1 validity.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_accept_a_quoted_strict_timestamp_with_a_tab_separated_comment() {
        // Arrange: a v1 EPIC record separates a quoted timestamp and YAML comment with a tab.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-09-21 20:35\"\t# updated\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let profile = validate(&frontmatter).unwrap();

        // Assert: any YAML whitespace before a trailing comment remains valid.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_reject_an_impossible_strict_timestamp() {
        // Arrange: a v1 EPIC record has syntactically shaped but impossible calendar and clock values.
        let markdown = "---\nschema-version: 1\ndoc-type: epic\nstatus: planned\nepic: null\ngithub-issue: 2264\nspec-path: docs/issues/open/example/EPIC.md\nepic-owner: null\nlast-updated-utc: \"2026-99-99 99:99\"\nsemantic-links: {}\n---\n# EPIC\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict EPIC profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic rejects out-of-range date and time components.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_distinguish_utc_minute_layout_from_calendar_boundaries() {
        // Arrange: each row changes one UTC-minute layout or calendar boundary.
        let cases = [
            ("2024-02-29 23:59", true, true),
            ("2025-02-29 23:59", true, false),
            ("2026-04-31 12:00", true, false),
            ("2026-13-01 12:00", true, false),
            ("2026-01-01 24:00", true, false),
            ("2026-01-01 12:60", true, false),
            ("2026-01-01 12:0", false, false),
            ("2026-01-01 12:é0", false, false),
        ];

        for (value, expected_layout, expected_calendar) in cases {
            // Act: evaluate layout first, then calendar validity only for a safe fixed layout.
            let actual_layout = has_utc_minute_layout(value);
            let actual_calendar = if actual_layout {
                is_valid_utc_minute_calendar(value)
            } else {
                false
            };

            // Assert: each independent boundary has the documented result without panics.
            assert_eq!(actual_layout, expected_layout, "unexpected layout result for `{value}`");
            assert_eq!(actual_calendar, expected_calendar, "unexpected calendar result for `{value}`");
        }
    }

    #[test]
    fn it_should_reject_a_traversal_strict_specification_path() {
        // Arrange: a v1 issue record uses a parent-directory traversal as its specification path.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: ../outside/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links: {}\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic requires a repository-relative specification path.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_a_backslash_strict_specification_path() {
        // Arrange: a v1 issue uses a platform-specific separator in its specification path.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs\\issues\\open\\example\\ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links: {}\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: repository paths use forward-slash separators.
        assert_eq!(error.category, DiagnosticCategory::InvalidFieldValue);
    }

    #[test]
    fn it_should_reject_a_non_string_document_type_for_a_v1_record() {
        // Arrange: a v1 record supplies an integer instead of a document-type string.
        let markdown = "---\nschema-version: 1\ndoc-type: 2266\n---\n# Invalid record\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the strict-profile dispatcher identifies the scalar-type violation.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_a_non_integer_schema_version_for_a_strict_issue_candidate() {
        // Arrange: an issue-shaped v1 candidate quotes its schema-version integer.
        let markdown = "---\nschema-version: \"1\"\ndoc-type: issue\n---\n# Invalid issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and validate the frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the schema version must remain an unquoted YAML integer.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_the_invalid_reference_fixture() {
        // Arrange: the predecessor fixture uses an absolute URL as a related artifact.
        let markdown = include_str!(
            "../../../../../docs/issues/closed/2265-2264-inventory-markdown-frontmatter-contracts/frontmatter-fixtures/rejected/issue-invalid-reference.md"
        );
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the provisional reference syntax violation.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_accept_all_provisional_related_artifact_forms() {
        // Arrange: a strict issue uses a repository path, issue reference, and review finding.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - write-markdown-docs\n  related-artifacts:\n    - Cargo.toml\n    - issue #2264\n    - review-finding:pr-2230-f1\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let profile = validate(&frontmatter).unwrap();

        // Assert: every approved provisional reference form remains accepted.
        assert!(matches!(profile, Profile::Issue(_)));
    }

    #[test]
    fn it_should_reject_an_unapproved_tagged_related_artifact() {
        // Arrange: a strict issue uses a tagged artifact form outside the frozen v1 union.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  related-artifacts:\n    - adr:frontmatter\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic reserves typed forms for the approved v1 union.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_reject_a_backslash_related_artifact_path() {
        // Arrange: a strict issue uses a platform-specific separator in a related artifact path.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  related-artifacts:\n    - docs\\AGENTS.md\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: validate the strict issue profile.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: related artifact paths use forward-slash separators.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_report_a_missing_envelope_instead_of_panicking() {
        // Arrange: a strict issue's retained envelope has been lost after extraction.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links: {}\n---\n# Issue\n";
        let mut frontmatter = extract(markdown).unwrap().unwrap();
        frontmatter.semantic_links = None;

        // Act: validate the inconsistent strict frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: validation reports the missing envelope instead of panicking.
        assert_eq!(error.category, DiagnosticCategory::MissingRequiredField);
    }

    #[test]
    fn it_should_reject_an_invalid_skill_name() {
        // Arrange: a strict issue uses uppercase letters in a frozen skill-name reference.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 18:30\"\nsemantic-links:\n  skill-links:\n    - Write-Markdown-Docs\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the frozen skill-name syntax violation.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }

    #[test]
    fn it_should_reject_a_skill_name_with_consecutive_hyphens() {
        // Arrange: a strict issue uses an empty segment in a hyphen-separated skill name.
        let markdown = "---\nschema-version: 1\ndoc-type: issue\nissue-type: task\nstatus: planned\npriority: p1\nepic: null\ngithub-issue: 2266\nspec-path: docs/issues/open/example/ISSUE.md\nbranch: example\nrelated-pr: null\nlast-updated-utc: \"2026-09-21 21:10\"\nsemantic-links:\n  skill-links:\n    - write--markdown-docs\n---\n# Issue\n";
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: every identifier segment must contain lowercase letters or digits.
        assert_eq!(error.category, DiagnosticCategory::InvalidReferenceSyntax);
    }
}
