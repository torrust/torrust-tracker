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

Green run (T3, after adding the audited pull request's commits): `Ran 1 test ... OK`. On the real
audits with `--base torrust/develop`, PR #2599 now passes and PR #2584 fails only WAIT-001, whose
fix merged in PR #2596 (T4 and T5 cover that case).

## T4 - A Subject Only in a Follow-up Pull Request's Commits Is Accepted

Test added: `ValidSubjectsTest.test_it_should_accept_a_subject_found_only_in_a_follow_up_pull_requests_commits`

Interface added by this increment: `pulls_to_check(audit_text, audited)` returns the audited pull
request followed by every pull request named in a `Follow-up PR URL` field of the audit text.

Prose-first comparison:

- Arrange: the audited pull request has no commit with the fix; the audit records PR #2596 in a
  `Follow-up PR URL` field; PR #2596's commit list holds the fix subject; the local range is empty.
- Act: collect the pull requests to check from the audit text, then ask for their valid subjects.
- Assert: the fix subject is among them.

The causal difference is where the subject lives: only the follow-up's commit list, reachable only
through the audit's `Follow-up PR URL` line. Both appear inline (`audit`, `commits`), so the reader
sees why the subject is expected. The two-step Act (`pulls_to_check` then `valid_subjects`) mirrors
`main`; it is shown rather than hidden in a helper. The expected subject is a literal.

Red run (before the production change):

```text
ERROR: test_it_should_accept_a_subject_found_only_in_a_follow_up_pull_requests_commits
AttributeError: module 'validate_audit_record' has no attribute 'pulls_to_check'
Ran 2 tests in 0.001s
FAILED (errors=1)
```

As for T2, the red is an error for the missing interface; the behavioral red on the real audit
(WAIT-001 fails with `--base torrust/develop`) is recorded in the T3 green run above.

Green run (T5, after adding follow-up pull requests): `Ran 2 tests ... OK`. On the real PR #2584
audit with `--base torrust/develop`: `{"status": "ok", "rows": 4, "log_entries": 6, "failures": 0}`.

## T6 - Rejection, Local Range, `gh` Failure, and Follow-up URL Parsing (held for maintainer review)

Tests added to `ValidSubjectsTest`:

- `test_it_should_reject_a_subject_found_in_no_source` (guards existing behavior; AC3)
- `test_it_should_accept_a_subject_found_only_in_the_local_range_of_unpushed_commits` (guards existing behavior)
- `test_it_should_exit_with_a_usage_error_naming_the_endpoint_when_listing_commits_fails` (red; AC4)

Tests added to the new `PullsToCheckTest` (beyond the spec's case list; they pin the follow-up
parsing decision recorded in Design and Ownership Review, and guard the T5 behavior):

- `test_it_should_fetch_a_follow_up_from_the_repository_named_in_its_url`
- `test_it_should_ignore_empty_follow_ups_and_fetch_a_repeated_follow_up_once`

Fixture change: `fake_git_and_gh` takes `failing_endpoints`, the one incidental mechanic needed to
make a `gh api` call exit non-zero.

Prose-first comparison:

- **No source.** Arrange: the local range and the audited pull request each hold a commit, neither
  with the cited subject. Act: ask for the valid subjects. Assert: the subject is not among them.
  The cited subject is a literal that appears only in the assertion, so its absence is the causal
  state.
- **Local range.** Arrange: the pull request's commit list is empty and one subject is only in
  `git log <base>..HEAD` (an unpushed fix). Act: ask for the valid subjects. Assert: the subject is
  among them.
- **`gh` failure.** Arrange: the commits endpoint of the audited pull request fails. Act: ask for
  the valid subjects, capturing stderr. Assert: the process exits with code 2 and stderr names the
  failing endpoint. The endpoint is computed by the test's own `commits_endpoint`, independent of
  production code.
- **Follow-up parsing.** Arrange: the audit text holds the `Follow-up PR URL` lines. Act:
  `pulls_to_check`. Assert: the exact list of pull requests. The URL literals and expected tuples
  are inline.

Red run (T6 tests against the T5 production code; the first run of the new tests):

```text
ERROR: test_it_should_exit_with_a_usage_error_naming_the_endpoint_when_listing_commits_fails
subprocess.CalledProcessError: Command '['gh', 'api', 'repos/torrust/torrust-tracker/pulls/2584/commits', '--paginate']' returned non-zero exit status 1.
Ran 7 tests in 0.002s
FAILED (errors=1)
```

The six other tests pass because they guard behavior that already exists; only the `gh` failure test
is red, as the Implementation Plan expects.
