# Issue #2606 Test-Design Review

Prose-first Arrange-Act-Assert reviews for each test-producing increment, as required by the
`write-unit-test` skill. The tests live in
`.github/skills/dev/pr-reviews/process-pr-review/scripts/test_validate_audit_record.py` and run with:

```sh
python3 -m unittest discover -s .github/skills/dev/pr-reviews/process-pr-review/scripts
```

Python 3.14.4. `git` and `gh` are faked at the process boundary (`subprocess.run`), so no repository
state or network is involved.

## Interface Under Test

`valid_subjects(base, pulls)` returns the set of commit subjects a `FIXED` resolution reference may
cite. `pulls` is a list of `(owner, repo, number)` tuples whose commit lists are fetched with
`gh api repos/<owner>/<repo>/pulls/<n>/commits --paginate`; the result also holds the subjects of
`git log <base>..HEAD`. The tuple needs no named type: three positional fields are unpacked in one place.

## T2 - A Subject Only in the Audited Pull Request's Commits Is Accepted

Test added: `ValidSubjectsTest.test_it_should_accept_a_subject_found_only_in_the_audited_pull_requests_commits`

Prose-first comparison:

- Arrange: the local range `<base>..HEAD` is empty, as it is once the fix is merged and the base is
  current (`torrust/develop`); the audited pull request's commit list holds one fix subject.
- Act: ask for the valid subjects of the audited pull request with that base.
- Assert: the fix subject is among them.

The causal initial state is one difference: the subject is present only in the pull request's commit
list. It is visible in the test as `git_subjects=[]` next to `pull_commits=audited_commits`. The
fixture `fake_git_and_gh` owns only the incidental mechanics (the `CompletedProcess` objects and the
JSON shape of the commits endpoint). The subject literal is written in the test, not derived from
production code, and the Act is a single visible call. No prose was retained as comments.

Red run (before the production change), against the unmodified script:

```text
ERROR: test_it_should_accept_a_subject_found_only_in_the_audited_pull_requests_commits
AttributeError: module 'validate_audit_record' has no attribute 'valid_subjects'. Did you mean: 'branch_subjects'?
Ran 1 test in 0.000s
FAILED (errors=1)
```

The red is an error for the missing interface: the current code has no function that takes a pull
request. The behavioral red on the real audits (4 failures) is in `manual-verification-evidence.md`
section V1.
