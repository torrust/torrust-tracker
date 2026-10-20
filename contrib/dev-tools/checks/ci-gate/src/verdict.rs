//! The merge verdict: whether the results of the job groups let a pull request merge.
//!
//! The verdict is a pure function of three inputs (issue #2525, task T3):
//!
//! - the [`Classification`] that change detection produced, which says which groups are needed;
//! - the [`GroupResult`] of each group that reported one;
//! - an optional maintainer override, identified by the login of the actor who applied it.
//!
//! The verdict passes when every needed group succeeded, or when the override applies. Otherwise it
//! fails and names each needed group that failed, was cancelled, was skipped, or never reported. A
//! group that is not needed never affects the verdict, whatever its result.
//!
//! The inputs are provider-neutral. Where the results and the override come from is decided by the
//! workflow that runs the tool, not here: this module never looks at the CI provider.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::{Deserialize, Serialize};

use crate::change_detection::Classification;
use crate::job_group::JobGroup;

/// How a job group ended, as the workflow reports it.
///
/// The values are the ones GitHub Actions gives a job in `needs.<job>.result`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GroupResult {
    Success,
    Failure,
    Cancelled,
    Skipped,
}

/// Why a needed group does not count as a success.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Problem {
    Failure,
    Cancelled,
    /// The group was needed but did not run.
    Skipped,
    /// The group was needed but reported no result at all.
    Missing,
}

impl Problem {
    const fn name(self) -> &'static str {
        match self {
            Self::Failure => "failure",
            Self::Cancelled => "cancelled",
            Self::Skipped => "skipped",
            Self::Missing => "missing",
        }
    }
}

/// A needed group that does not count as a success.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct GroupProblem {
    pub group: JobGroup,
    pub problem: Problem,
}

/// Whether the pull request may merge.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Pass,
    Fail,
}

/// The outcome of the verdict, as the tool prints it.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerdictResult {
    pub verdict: Verdict,
    /// Every needed group that does not count as a success, in [`JobGroup::ALL`] order, whether or
    /// not an override applied.
    pub problems: Vec<GroupProblem>,
    /// The login of the actor whose override made the verdict pass. It is set only when there were
    /// problems to override.
    pub overridden_by: Option<String>,
    /// A human-readable account of the verdict that names each group involved.
    pub explanation: String,
}

/// The JSON the tool reads on stdin.
///
/// `classification` is the output of the `classify` command, unchanged. `results` holds the
/// result of each group that reported one. Unknown fields and unknown group names are errors, so
/// that a typo cannot silently turn a result into a missing one.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerdictInput {
    pub classification: Classification,
    pub results: BTreeMap<JobGroup, GroupResult>,
}

/// Decides whether a pull request may merge.
///
/// # Errors
///
/// Returns a message when `classification` does not list every job group exactly once. The verdict
/// refuses to guess which groups are needed, because a classification that omits a group would
/// otherwise let a pull request merge without it.
pub fn decide(
    classification: &Classification,
    results: &BTreeMap<JobGroup, GroupResult>,
    override_actor: Option<&str>,
) -> Result<VerdictResult, String> {
    check_every_group_is_listed_once(classification)?;

    let problems: Vec<GroupProblem> = classification
        .groups
        .iter()
        .filter(|decision| decision.needed)
        .filter_map(|decision| {
            problem_of(results.get(&decision.group)).map(|problem| GroupProblem {
                group: decision.group,
                problem,
            })
        })
        .collect();

    let overridden_by = override_actor.filter(|_| !problems.is_empty()).map(str::to_string);
    let verdict = if problems.is_empty() || overridden_by.is_some() {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    let explanation = explain(classification, &problems, overridden_by.as_deref());

    Ok(VerdictResult {
        verdict,
        problems,
        overridden_by,
        explanation,
    })
}

fn check_every_group_is_listed_once(classification: &Classification) -> Result<(), String> {
    for group in JobGroup::ALL {
        let listed = classification
            .groups
            .iter()
            .filter(|decision| decision.group == group)
            .count();
        if listed != 1 {
            return Err(format!(
                "the classification lists the group `{}` {listed} times; it must list every group exactly once",
                group.name()
            ));
        }
    }
    Ok(())
}

const fn problem_of(result: Option<&GroupResult>) -> Option<Problem> {
    match result {
        Some(GroupResult::Success) => None,
        Some(GroupResult::Failure) => Some(Problem::Failure),
        Some(GroupResult::Cancelled) => Some(Problem::Cancelled),
        Some(GroupResult::Skipped) => Some(Problem::Skipped),
        None => Some(Problem::Missing),
    }
}

fn explain(classification: &Classification, problems: &[GroupProblem], overridden_by: Option<&str>) -> String {
    if problems.is_empty() {
        let needed: Vec<&str> = classification
            .groups
            .iter()
            .filter(|decision| decision.needed)
            .map(|decision| decision.group.name())
            .collect();
        return format!("every needed group succeeded: {}", needed.join(", "));
    }

    let mut listed = String::new();
    for (index, problem) in problems.iter().enumerate() {
        if index > 0 {
            listed.push_str(", ");
        }
        write!(listed, "{} ({})", problem.group.name(), problem.problem.name()).expect("writing to a String cannot fail");
    }

    overridden_by.map_or_else(
        || format!("needed groups without a success: {listed}"),
        |actor| format!("overridden by `{actor}`; needed groups without a success: {listed}"),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::change_detection::GroupDecision;

    /// A classification that needs exactly `needed` and lists every other group as not needed.
    fn classification_needing(needed: &[JobGroup]) -> Classification {
        Classification {
            base_branch: "develop".to_string(),
            changed_path_count: 1,
            groups: JobGroup::ALL
                .iter()
                .map(|group| GroupDecision {
                    group: *group,
                    needed: needed.contains(group),
                    explanation: "test".to_string(),
                })
                .collect(),
        }
    }

    fn results(entries: &[(JobGroup, GroupResult)]) -> BTreeMap<JobGroup, GroupResult> {
        entries.iter().copied().collect()
    }

    #[test]
    fn it_should_pass_when_every_needed_group_succeeded() {
        // Arrange
        let classification = classification_needing(&[JobGroup::DocsLint, JobGroup::Testing]);
        let results = results(&[
            (JobGroup::DocsLint, GroupResult::Success),
            (JobGroup::Testing, GroupResult::Success),
        ]);

        // Act
        let verdict = decide(&classification, &results, None).expect("the classification is complete");

        // Assert
        assert_eq!(
            verdict,
            VerdictResult {
                verdict: Verdict::Pass,
                problems: vec![],
                overridden_by: None,
                explanation: "every needed group succeeded: docs-lint, testing".to_string(),
            }
        );
    }

    #[rstest]
    #[case::failed(Some(GroupResult::Failure), Problem::Failure)]
    #[case::cancelled(Some(GroupResult::Cancelled), Problem::Cancelled)]
    #[case::skipped_although_needed(Some(GroupResult::Skipped), Problem::Skipped)]
    #[case::reported_no_result(None, Problem::Missing)]
    fn it_should_fail_naming_the_group_when_a_needed_group_did_not_succeed(
        #[case] result: Option<GroupResult>,
        #[case] expected_problem: Problem,
    ) {
        // Arrange
        let classification = classification_needing(&[JobGroup::DocsLint, JobGroup::Testing]);
        let mut results = results(&[(JobGroup::DocsLint, GroupResult::Success)]);
        if let Some(result) = result {
            results.insert(JobGroup::Testing, result);
        }

        // Act
        let verdict = decide(&classification, &results, None).expect("the classification is complete");

        // Assert
        assert_eq!(
            verdict,
            VerdictResult {
                verdict: Verdict::Fail,
                problems: vec![GroupProblem {
                    group: JobGroup::Testing,
                    problem: expected_problem
                }],
                overridden_by: None,
                explanation: format!("needed groups without a success: testing ({})", expected_problem.name()),
            }
        );
    }

    #[test]
    fn it_should_name_every_group_that_did_not_succeed_in_group_order_when_several_did_not() {
        // Arrange
        let classification = classification_needing(&[
            JobGroup::DocsLint,
            JobGroup::Testing,
            JobGroup::Coverage,
            JobGroup::DbCompatibility,
        ]);
        let results = results(&[
            (JobGroup::DbCompatibility, GroupResult::Cancelled),
            (JobGroup::Testing, GroupResult::Failure),
            (JobGroup::DocsLint, GroupResult::Success),
        ]);

        // Act
        let verdict = decide(&classification, &results, None).expect("the classification is complete");

        // Assert
        assert_eq!(verdict.verdict, Verdict::Fail);
        assert_eq!(
            verdict.explanation,
            "needed groups without a success: testing (failure), coverage (missing), db-compatibility (cancelled)"
        );
    }

    #[rstest]
    #[case::no_result(None)]
    #[case::skipped(Some(GroupResult::Skipped))]
    #[case::failed_although_not_needed(Some(GroupResult::Failure))]
    #[case::cancelled_although_not_needed(Some(GroupResult::Cancelled))]
    fn it_should_ignore_a_group_that_is_not_needed_whatever_its_result(#[case] result: Option<GroupResult>) {
        // Arrange
        let classification = classification_needing(&[JobGroup::DocsLint]);
        let mut results = results(&[(JobGroup::DocsLint, GroupResult::Success)]);
        if let Some(result) = result {
            results.insert(JobGroup::SecurityScan, result);
        }

        // Act
        let verdict = decide(&classification, &results, None).expect("the classification is complete");

        // Assert
        assert_eq!(
            (verdict.verdict, verdict.problems),
            (Verdict::Pass, vec![]),
            "`security-scan` is not needed, so its result {result:?} must not matter"
        );
    }

    #[test]
    fn it_should_pass_and_name_the_actor_and_the_overridden_groups_when_an_override_applies() {
        // Arrange
        let classification = classification_needing(&[JobGroup::DocsLint, JobGroup::Testing, JobGroup::Coverage]);
        let results = results(&[
            (JobGroup::DocsLint, GroupResult::Success),
            (JobGroup::Testing, GroupResult::Failure),
        ]);

        // Act
        let verdict = decide(&classification, &results, Some("josecelano")).expect("the classification is complete");

        // Assert
        assert_eq!(
            verdict,
            VerdictResult {
                verdict: Verdict::Pass,
                problems: vec![
                    GroupProblem {
                        group: JobGroup::Testing,
                        problem: Problem::Failure
                    },
                    GroupProblem {
                        group: JobGroup::Coverage,
                        problem: Problem::Missing
                    },
                ],
                overridden_by: Some("josecelano".to_string()),
                explanation: "overridden by `josecelano`; needed groups without a success: testing (failure), coverage (missing)"
                    .to_string(),
            }
        );
    }

    #[test]
    fn it_should_not_mention_the_override_when_every_needed_group_succeeded() {
        // Arrange
        let classification = classification_needing(&[JobGroup::DocsLint]);
        let results = results(&[(JobGroup::DocsLint, GroupResult::Success)]);

        // Act
        let verdict = decide(&classification, &results, Some("josecelano")).expect("the classification is complete");

        // Assert
        assert_eq!(
            (verdict.verdict, verdict.overridden_by),
            (Verdict::Pass, None),
            "an override that overrides nothing must not be recorded as having been used"
        );
    }

    #[test]
    fn it_should_refuse_to_decide_when_the_classification_omits_a_group() {
        // Arrange
        let mut classification = classification_needing(&[JobGroup::DocsLint]);
        classification.groups.retain(|decision| decision.group != JobGroup::Testing);
        let results = results(&[(JobGroup::DocsLint, GroupResult::Success)]);

        // Act
        let verdict = decide(&classification, &results, Some("josecelano"));

        // Assert
        assert_eq!(
            verdict,
            Err("the classification lists the group `testing` 0 times; it must list every group exactly once".to_string()),
            "an omitted group could be a needed one, so even an override must not turn this into a pass"
        );
    }

    #[test]
    fn it_should_refuse_to_decide_when_the_classification_lists_a_group_twice() {
        // Arrange
        let mut classification = classification_needing(&[JobGroup::DocsLint]);
        classification.groups.push(GroupDecision {
            group: JobGroup::Container,
            needed: false,
            explanation: "test".to_string(),
        });
        let results = results(&[(JobGroup::DocsLint, GroupResult::Success)]);

        // Act
        let verdict = decide(&classification, &results, None);

        // Assert
        assert_eq!(
            verdict,
            Err("the classification lists the group `container` 2 times; it must list every group exactly once".to_string())
        );
    }

    #[test]
    fn it_should_name_each_group_as_the_json_does() {
        // Arrange
        let groups = JobGroup::ALL;

        // Act
        let json_names: Vec<String> = groups
            .iter()
            .map(|group| {
                serde_json::to_value(group)
                    .expect("a group serializes")
                    .as_str()
                    .expect("as a string")
                    .to_string()
            })
            .collect();

        // Assert
        let names: Vec<&str> = groups.iter().map(|group| group.name()).collect();
        assert_eq!(names, json_names);
    }

    #[test]
    fn it_should_read_the_classification_the_classify_command_prints() {
        // Arrange
        let classification = crate::change_detection::classify(&["README.md"], "develop");
        let input = serde_json::json!({
            "classification": classification,
            "results": { "docs-lint": "success" },
        });

        // Act
        let parsed: VerdictInput = serde_json::from_value(input).expect("the input is valid");
        let verdict = decide(&parsed.classification, &parsed.results, None).expect("the classification is complete");

        // Assert
        assert_eq!(
            (verdict.verdict, verdict.explanation.as_str()),
            (Verdict::Pass, "every needed group succeeded: docs-lint")
        );
    }

    // Each case changes one thing in an input that is otherwise valid, so it can only be rejected for
    // the reason it names. The base input is accepted: see the test above.
    #[rstest]
    #[case::unknown_top_level_field(serde_json::json!({ "result": {} }))]
    #[case::unknown_group_in_results(serde_json::json!({ "results": { "tests": "success" } }))]
    #[case::unknown_result_value(serde_json::json!({ "results": { "testing": "passed" } }))]
    #[case::results_that_are_not_an_object(serde_json::json!({ "results": ["success"] }))]
    fn it_should_reject_an_input_that_is_not_exactly_the_documented_shape(#[case] change: serde_json::Value) {
        // Arrange
        let mut input = serde_json::json!({
            "classification": crate::change_detection::classify(&["README.md"], "develop"),
            "results": { "docs-lint": "success" },
        });
        input
            .as_object_mut()
            .expect("the input is an object")
            .extend(change.as_object().expect("the change is an object").clone());

        // Act
        let parsed = serde_json::from_value::<VerdictInput>(input.clone());

        // Assert
        assert!(parsed.is_err(), "the input {input} must be rejected");
    }
}
