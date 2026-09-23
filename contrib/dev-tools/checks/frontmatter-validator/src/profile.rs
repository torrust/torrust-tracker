//! Canonical strict frontmatter profile types and structural validation.

use schemars::JsonSchema;
use serde::Deserialize;
use serde_yaml::{Mapping, Value};

use crate::syntax::{
    RELATED_ARTIFACT_PATTERN, REPOSITORY_RELATIVE_PATH_PATTERN, SKILL_NAME_PATTERN, UTC_MINUTE_PATTERN, has_utc_minute_layout,
    is_related_artifact, is_repository_relative_path, is_skill_name, is_valid_utc_minute_calendar,
};
use crate::{Diagnostic, DiagnosticCategory, DocumentOwnership, Frontmatter};

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
        StrictProfileKind::Issue => validate_issue(frontmatter, doc_type.definition()).map(Profile::Issue),
        StrictProfileKind::Epic => validate_epic(frontmatter, doc_type.definition()).map(Profile::Epic),
    }
}

fn validate_issue(frontmatter: &Frontmatter, definition: &StrictProfileDefinition) -> Result<Issue, Diagnostic> {
    validate_strict_profile(frontmatter, definition, Issue::validate_invariants)
}

fn validate_epic(frontmatter: &Frontmatter, definition: &StrictProfileDefinition) -> Result<Epic, Diagnostic> {
    validate_strict_profile(frontmatter, definition, Epic::validate_invariants)
}

fn validate_strict_profile<T>(
    frontmatter: &Frontmatter,
    definition: &StrictProfileDefinition,
    validate_invariants: impl FnOnce(&T, &Frontmatter) -> Result<(), Diagnostic>,
) -> Result<T, Diagnostic>
where
    T: for<'de> Deserialize<'de>,
{
    definition.validate_structure(&frontmatter.values)?;
    validate_reference_syntax(&frontmatter.values)?;
    let profile = deserialize_strict(&frontmatter.values)?;
    validate_invariants(&profile, frontmatter)?;

    Ok(profile)
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

fn strict_document_type(values: &Mapping) -> Result<Option<StrictProfileKind>, Diagnostic> {
    let Some(schema_version) = values.get("schema-version") else {
        return Ok(None);
    };
    let profile_kind = values
        .get("doc-type")
        .and_then(Value::as_str)
        .and_then(StrictProfileKind::from_doc_type);
    let Some(schema_version) = schema_version.as_i64() else {
        if profile_kind.is_some() {
            return Err(Diagnostic::new(
                DiagnosticCategory::WrongScalarType,
                "`schema-version` must be a YAML integer.",
            ));
        }

        return Ok(None);
    };
    if schema_version != 1 {
        return Ok(None);
    }

    let Some(doc_type) = values.get("doc-type") else {
        return Err(Diagnostic::new(
            DiagnosticCategory::MissingRequiredField,
            "Strict frontmatter requires `doc-type`.",
        ));
    };
    let Some(doc_type) = doc_type.as_str() else {
        return Err(Diagnostic::new(
            DiagnosticCategory::WrongScalarType,
            "`doc-type` must be a YAML string.",
        ));
    };

    Ok(StrictProfileKind::from_doc_type(doc_type))
}

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

fn validate_known_fields(values: &Mapping, allowed_fields: &[&str]) -> Result<(), Diagnostic> {
    for key in values.keys() {
        let Some(field) = key.as_str() else {
            return Err(Diagnostic::new(
                DiagnosticCategory::UnknownField,
                "Strict frontmatter field names must be strings.",
            ));
        };
        if !allowed_fields.contains(&field) && !field.starts_with("x-") {
            return Err(Diagnostic::new(
                DiagnosticCategory::UnknownField,
                format!("`{field}` is not allowed by this strict frontmatter profile."),
            ));
        }
    }

    Ok(())
}

fn validate_required_fields(values: &Mapping, required_fields: &[&str]) -> Result<(), Diagnostic> {
    for field in required_fields {
        if !values.contains_key(*field) {
            return Err(Diagnostic::new(
                DiagnosticCategory::MissingRequiredField,
                format!("Strict frontmatter requires `{field}`."),
            ));
        }
    }

    Ok(())
}

fn validate_allowed_string(values: &Mapping, field: &str, allowed_values: &[&str]) -> Result<(), Diagnostic> {
    let value = values.get(field).expect("required fields were checked first");
    let Some(value) = value.as_str() else {
        return Err(Diagnostic::new(
            DiagnosticCategory::WrongScalarType,
            format!("`{field}` must be a YAML string."),
        ));
    };
    if !allowed_values.contains(&value) {
        return Err(Diagnostic::new(
            DiagnosticCategory::InvalidAllowedValue,
            format!("`{field}` has an unsupported value `{value}`."),
        ));
    }

    Ok(())
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
    #[schemars(regex(pattern = REPOSITORY_RELATIVE_PATH_PATTERN))]
    pub spec_path: String,
    /// The development branch name.
    #[schemars(length(min = 1))]
    pub branch: String,
    /// The optional positive related pull request number.
    #[schemars(range(min = 1))]
    pub related_pr: Option<u64>,
    /// The required UTC-minute update timestamp.
    #[schemars(regex(pattern = UTC_MINUTE_PATTERN))]
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

impl Issue {
    fn validate_invariants(&self, frontmatter: &Frontmatter) -> Result<(), Diagnostic> {
        validate_optional_positive_integer("epic", self.epic)?;
        validate_optional_positive_integer("github-issue", self.github_issue)?;
        validate_optional_positive_integer("related-pr", self.related_pr)?;
        validate_repository_relative_path("spec-path", &self.spec_path)?;
        validate_non_empty_string("branch", &self.branch)?;
        validate_utc_minute_string(
            &self.last_updated_utc,
            frontmatter.has_double_quoted_scalar("last-updated-utc"),
        )
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
    #[schemars(regex(pattern = REPOSITORY_RELATIVE_PATH_PATTERN))]
    pub spec_path: String,
    /// The optional owner of the EPIC.
    #[schemars(length(min = 1))]
    pub epic_owner: Option<String>,
    /// The required UTC-minute update timestamp.
    #[schemars(regex(pattern = UTC_MINUTE_PATTERN))]
    pub last_updated_utc: String,
    /// The required strict semantic-link envelope.
    pub semantic_links: StrictSemanticLinks,
}

impl Epic {
    fn validate_invariants(&self, frontmatter: &Frontmatter) -> Result<(), Diagnostic> {
        validate_optional_positive_integer("epic", self.epic)?;
        validate_optional_positive_integer("github-issue", self.github_issue)?;
        validate_repository_relative_path("spec-path", &self.spec_path)?;
        if let Some(owner) = &self.epic_owner {
            validate_non_empty_string("epic-owner", owner)?;
        }
        validate_utc_minute_string(
            &self.last_updated_utc,
            frontmatter.has_double_quoted_scalar("last-updated-utc"),
        )
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
#[schemars(extend("pattern" = SKILL_NAME_PATTERN))]
pub struct SkillName(String);

/// A validated v1 related-artifact reference.
#[derive(Debug, Deserialize, Eq, JsonSchema, PartialEq)]
#[serde(try_from = "String")]
#[schemars(extend("pattern" = RELATED_ARTIFACT_PATTERN))]
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

/// Reference syntax is reported before any other scalar-type failure in the profile.
fn validate_reference_syntax(values: &Mapping) -> Result<(), Diagnostic> {
    let semantic_links = values.get("semantic-links").expect("required fields were checked first");
    serde_yaml::from_value::<StrictSemanticLinks>(semantic_links.clone())
        .map(drop)
        .map_err(|error| {
            Diagnostic::new(
                DiagnosticCategory::InvalidReferenceSyntax,
                format!("`semantic-links` contains an invalid v1 reference: {error}"),
            )
        })
}

fn deserialize_strict<T>(values: &Mapping) -> Result<T, Diagnostic>
where
    T: for<'de> Deserialize<'de>,
{
    let mut values = values.clone();
    values.retain(|key, _| !key.as_str().is_some_and(|field| field.starts_with("x-")));
    serde_yaml::from_value(Value::Mapping(values))
        .map_err(|error| Diagnostic::new(DiagnosticCategory::WrongScalarType, error.to_string()))
}

fn validate_optional_positive_integer(field: &str, value: Option<u64>) -> Result<(), Diagnostic> {
    if value == Some(0) {
        return Err(Diagnostic::new(
            DiagnosticCategory::InvalidFieldValue,
            format!("`{field}` must be a positive integer or null."),
        ));
    }

    Ok(())
}

fn validate_non_empty_string(field: &str, value: &str) -> Result<(), Diagnostic> {
    if value.is_empty() {
        return Err(Diagnostic::new(
            DiagnosticCategory::InvalidFieldValue,
            format!("`{field}` must not be empty."),
        ));
    }

    Ok(())
}

fn validate_repository_relative_path(field: &str, value: &str) -> Result<(), Diagnostic> {
    if !is_repository_relative_path(value) {
        return Err(Diagnostic::new(
            DiagnosticCategory::InvalidFieldValue,
            format!("`{field}` must be a repository-relative path."),
        ));
    }

    Ok(())
}

fn validate_utc_minute_string(value: &str, double_quoted: bool) -> Result<(), Diagnostic> {
    if !has_utc_minute_layout(value) || !is_valid_utc_minute_calendar(value) || !double_quoted {
        return Err(Diagnostic::new(
            DiagnosticCategory::InvalidFieldValue,
            "`last-updated-utc` must be a double-quoted YAML string in YYYY-MM-DD HH:MM UTC-minute format.",
        ));
    }

    Ok(())
}

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
        assert_eq!(
            schema["$defs"]["Issue"]["properties"]["spec-path"]["pattern"],
            REPOSITORY_RELATIVE_PATH_PATTERN
        );
        assert_eq!(
            schema["$defs"]["Epic"]["properties"]["spec-path"]["pattern"],
            REPOSITORY_RELATIVE_PATH_PATTERN
        );
        assert_eq!(
            schema["$defs"]["Issue"]["properties"]["last-updated-utc"]["pattern"],
            UTC_MINUTE_PATTERN
        );
        assert_eq!(
            schema["$defs"]["Epic"]["properties"]["last-updated-utc"]["pattern"],
            UTC_MINUTE_PATTERN
        );
        assert_eq!(schema["$defs"]["SkillName"]["pattern"], SKILL_NAME_PATTERN);
        assert_eq!(schema["$defs"]["RelatedArtifact"]["pattern"], RELATED_ARTIFACT_PATTERN);
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
        let markdown = include_str!("../fixtures/accepted/issue.md");
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict issue contract.
        assert!(matches!(profile, Profile::Issue(_)));
    }

    #[test]
    fn it_should_classify_the_accepted_epic_fixture_as_a_strict_epic_profile() {
        // Arrange: the predecessor's accepted EPIC fixture declares schema version one.
        let markdown = include_str!("../fixtures/accepted/epic.md");
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: classify and structurally validate the parsed frontmatter.
        let profile = validate(&frontmatter).unwrap();

        // Assert: the canonical model recognizes the strict EPIC contract.
        assert!(matches!(profile, Profile::Epic(_)));
    }

    #[test]
    fn it_should_reject_the_wrong_scalar_fixture() {
        // Arrange: the predecessor fixture quotes a positive-integer issue identifier.
        let markdown = include_str!("../fixtures/rejected/issue-wrong-scalar.md");
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic distinguishes the exact scalar-type violation.
        assert_eq!(error.category, DiagnosticCategory::WrongScalarType);
    }

    #[test]
    fn it_should_reject_the_unknown_field_fixture() {
        // Arrange: the predecessor fixture introduces an unprefixed strict-profile field.
        let markdown = include_str!("../fixtures/rejected/issue-unknown-field.md");
        let frontmatter = extract(markdown).unwrap().unwrap();

        // Act: structurally validate the parsed strict issue frontmatter.
        let error = validate(&frontmatter).unwrap_err();

        // Assert: the diagnostic identifies the unsupported field.
        assert_eq!(error.category, DiagnosticCategory::UnknownField);
    }

    #[test]
    fn it_should_reject_the_invalid_status_fixture() {
        // Arrange: the predecessor fixture uses a status outside the issue lifecycle enum.
        let markdown = include_str!("../fixtures/rejected/issue-invalid-status.md");
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
        let markdown = include_str!("../fixtures/rejected/issue-invalid-reference.md");
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
