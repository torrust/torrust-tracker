#!/usr/bin/env python3
"""Deterministic tests for the commit subjects that validate-audit-record.py accepts.

`git` and `gh` are faked at the process boundary: no repository and no network are needed.
Run: python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts
"""

import contextlib
import importlib.util
import io
import json
import subprocess
import unittest
from pathlib import Path
from unittest import mock

SCRIPT = Path(__file__).resolve().parent / "validate-audit-record.py"
spec = importlib.util.spec_from_file_location("validate_audit_record", SCRIPT)
validate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validate)

UP_TO_DATE_BASE = "torrust/develop"
AUDITED_PULL = ("torrust", "torrust-tracker", 2584)


def commits_endpoint(pull: tuple[str, str, int]) -> str:
    owner, repo, number = pull
    return f"repos/{owner}/{repo}/pulls/{number}/commits"


@contextlib.contextmanager
def fake_git_and_gh(
    git_subjects: list[str],
    pull_commits: dict[str, list[str]],
    failing_endpoints: frozenset[str] = frozenset(),
    gh_installed: bool = True,
):
    """Fake `git log <base>..HEAD` and `gh api <endpoint> --paginate`, keyed by commits endpoint."""

    def run(argv, **kwargs):
        if argv[:2] == ["git", "log"]:
            return subprocess.CompletedProcess(argv, 0, stdout="\n".join(git_subjects), stderr="")
        if not gh_installed:
            raise FileNotFoundError(2, "No such file or directory", "gh")
        endpoint = argv[2]
        if endpoint in failing_endpoints:
            raise subprocess.CalledProcessError(1, argv, stderr="HTTP 502")
        commits = [{"commit": {"message": f"{subject}\n\nbody"}} for subject in pull_commits[endpoint]]
        return subprocess.CompletedProcess(argv, 0, stdout=json.dumps(commits), stderr="")

    with mock.patch.object(validate.subprocess, "run", side_effect=run):
        yield


class ValidSubjectsTest(unittest.TestCase):
    def test_it_should_accept_a_subject_found_only_in_the_audited_pull_requests_commits(self):
        fix_subject = "docs(issues): clarify manual runs in the spec"
        audited_commits = {commits_endpoint(AUDITED_PULL): [fix_subject]}

        with fake_git_and_gh(git_subjects=[], pull_commits=audited_commits):
            subjects = validate.valid_subjects(UP_TO_DATE_BASE, [AUDITED_PULL])

        self.assertIn(fix_subject, subjects)

    def test_it_should_accept_a_subject_found_only_in_a_follow_up_pull_requests_commits(self):
        fix_subject = "docs(pr-reviews): move the WAIT-001 closure entry"
        follow_up_pull = ("torrust", "torrust-tracker", 2596)
        audit = "- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2596>\n"
        commits = {
            commits_endpoint(AUDITED_PULL): [],
            commits_endpoint(follow_up_pull): [fix_subject],
        }

        with fake_git_and_gh(git_subjects=[], pull_commits=commits):
            subjects = validate.valid_subjects(UP_TO_DATE_BASE, validate.pulls_to_check(audit, AUDITED_PULL))

        self.assertIn(fix_subject, subjects)

    def test_it_should_accept_a_subject_found_only_in_the_local_range_of_unpushed_commits(self):
        unpushed_subject = "docs(pr-reviews): fix not pushed yet"
        audited_commits = {commits_endpoint(AUDITED_PULL): []}

        with fake_git_and_gh(git_subjects=[unpushed_subject], pull_commits=audited_commits):
            subjects = validate.valid_subjects(UP_TO_DATE_BASE, [AUDITED_PULL])

        self.assertIn(unpushed_subject, subjects)

    def test_it_should_exit_with_a_usage_error_naming_the_endpoint_when_listing_commits_fails(self):
        endpoint = commits_endpoint(AUDITED_PULL)

        with fake_git_and_gh(git_subjects=[], pull_commits={}, failing_endpoints=frozenset({endpoint})):
            with contextlib.redirect_stderr(io.StringIO()) as stderr, self.assertRaises(SystemExit) as raised:
                validate.valid_subjects(UP_TO_DATE_BASE, [AUDITED_PULL])

        self.assertEqual(2, raised.exception.code)
        self.assertIn(endpoint, stderr.getvalue())

    def test_it_should_exit_with_a_usage_error_when_gh_is_not_installed(self):
        with fake_git_and_gh(git_subjects=[], pull_commits={}, gh_installed=False):
            with contextlib.redirect_stderr(io.StringIO()) as stderr, self.assertRaises(SystemExit) as raised:
                validate.valid_subjects(UP_TO_DATE_BASE, [AUDITED_PULL])

        self.assertEqual(2, raised.exception.code)
        self.assertIn(commits_endpoint(AUDITED_PULL), stderr.getvalue())


class CheckRowTest(unittest.TestCase):
    KNOWN_SUBJECT = "fix(pr-reviews): accept fix subjects from the audited PR's commits"

    @staticmethod
    def detail_citing(resolution_reference: str) -> str:
        return (
            "- Source URL: https://github.com/torrust/torrust-tracker/pull/1#pullrequestreview-5\n"
            "- Source review ID: 5\n"
            f"- Resolution reference: {resolution_reference}\n"
        )

    def test_it_should_accept_a_resolution_reference_equal_to_a_known_subject(self):
        failures = validate.check_row("F1", "Minor", self.detail_citing(self.KNOWN_SUBJECT), {}, {self.KNOWN_SUBJECT})

        self.assertEqual([], failures)

    def test_it_should_reject_a_resolution_reference_that_only_nearly_matches_a_known_subject(self):
        near_misses = {
            "other case": self.KNOWN_SUBJECT.upper(),
            "prefix": self.KNOWN_SUBJECT[:-1],
            "suffix": self.KNOWN_SUBJECT + " again",
        }

        for name, reference in near_misses.items():
            with self.subTest(name):
                failures = validate.check_row("F1", "Minor", self.detail_citing(reference), {}, {self.KNOWN_SUBJECT})

                self.assertEqual(1, len(failures))
                self.assertIn("not a commit subject", failures[0])


class PullsToCheckTest(unittest.TestCase):
    def test_it_should_fetch_a_follow_up_from_the_repository_named_in_its_url(self):
        audit = "- Follow-up PR URL: <https://github.com/torrust/torrust-bittorrent/pull/12>\n"

        pulls = validate.pulls_to_check(audit, AUDITED_PULL)

        self.assertEqual([AUDITED_PULL, ("torrust", "torrust-bittorrent", 12)], pulls)

    def test_it_should_ignore_empty_follow_ups_and_fetch_a_repeated_follow_up_once(self):
        audit = (
            "- Follow-up PR URL: N/A\n"
            "- Follow-up PR URL: NA\n"
            "- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2596>\n"
            "- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2596>\n"
        )

        pulls = validate.pulls_to_check(audit, AUDITED_PULL)

        self.assertEqual([AUDITED_PULL, ("torrust", "torrust-tracker", 2596)], pulls)


if __name__ == "__main__":
    unittest.main()
