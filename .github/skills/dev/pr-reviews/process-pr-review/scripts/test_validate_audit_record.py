#!/usr/bin/env python3
"""Deterministic tests for the commit subjects that validate-audit-record.py accepts.

`git` and `gh` are faked at the process boundary: no repository and no network are needed.
Run: python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts
"""

import contextlib
import importlib.util
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
def fake_git_and_gh(git_subjects: list[str], pull_commits: dict[str, list[str]]):
    """Fake `git log <base>..HEAD` and `gh api <endpoint> --paginate`, keyed by commits endpoint."""

    def run(argv, **kwargs):
        if argv[:2] == ["git", "log"]:
            return subprocess.CompletedProcess(argv, 0, stdout="\n".join(git_subjects), stderr="")
        endpoint = argv[2]
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


if __name__ == "__main__":
    unittest.main()
