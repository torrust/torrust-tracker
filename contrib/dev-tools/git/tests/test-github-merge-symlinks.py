#!/usr/bin/env python3

"""Deterministic tests for the merge tool's symbolic-link declaration handling.

Each test builds a throwaway Git repository together with the bare upstream it merges from. The
upstream holds the target branch and the pull request's refs the way a forge publishes them, and
a URL rewrite points the tool's fetch at it, so the real fetch and merge paths run with no
network and no GitHub API.

A driver script imports the tool, replaces the three functions that talk to the GitHub API with
fixture data, and calls main(). Every prompt after the symbolic-link check is answered with 'x',
so a run that passes the check stops at the signing prompt and exits 1, while a run the check
refuses exits 4. The two codes are what the tests distinguish, together with the output text.
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

TESTS_DIRECTORY = Path(__file__).resolve().parent
MERGE_TOOL = TESTS_DIRECTORY.parent / 'github-merge.py'

PULL_REQUEST = '2175'
TARGET_BRANCH = 'develop'
FIXTURE_HOST = 'https://fixture.invalid'
FIXTURE_REPOSITORY = 'torrust/torrust-tracker'
DECLARATION = '.symlinks.json'

EXIT_REFUSED = 4
EXIT_NOT_SIGNED = 1
EXIT_USAGE_ERROR = 2

DRIVER = '''#!/usr/bin/env python3

"""Run the vendored merge tool against a fixture repository.

Usage: driver.py <tool path> [tool argument...]

The GitHub API calls are replaced with fixture data so the run stays local and deterministic.
Everything else, including the symbolic-link check under test, is the tool's own code.
"""

import importlib.util
import sys

specification = importlib.util.spec_from_file_location('vendored_github_merge', sys.argv[1])
tool = importlib.util.module_from_spec(specification)
specification.loader.exec_module(tool)

tool.retrieve_pr_info = lambda repository, pull, token: {
    'title': 'Fixture pull request',
    'body': 'Fixture pull request body',
    'base': {'ref': 'develop'},
}
tool.retrieve_pr_comments = lambda repository, pull, token: []
tool.retrieve_pr_reviews = lambda repository, pull, token: []

sys.argv = sys.argv[1:]
tool.main()
'''


def declaration_document(*entries):
    """Build a declaration carrying one (path, target, reason) triple per entry."""
    return json.dumps({
        'namespace': 'com.torrust.repository.symlinks',
        'version': [1, 0, 0],
        'symlinks': [
            {'path': path, 'target': target, 'reason': reason}
            for path, target, reason in entries
        ],
    }, indent=2) + '\n'


REASON = 'Docker reads only .dockerignore, while Podman and Buildah prefer .containerignore.'


class MergeFixture:
    """A working repository and the bare upstream that publishes one pull request to it."""

    def __init__(self, root, driver):
        self.root = Path(root)
        self.driver = Path(driver)
        self.upstream = self.root.parent / 'upstream.git'
        self.git('init', '--quiet', f'--initial-branch={TARGET_BRANCH}')
        self.git('init', '--quiet', '--bare', str(self.upstream))
        self.git('config', 'user.name', 'Merge workflow test')
        self.git('config', 'user.email', 'merge-workflow-test@example.com')
        self.git('config', 'commit.gpgsign', 'false')
        self.git('config', 'user.signingkey', '0123456789ABCDEF')
        self.git('config', 'githubmerge.repository', FIXTURE_REPOSITORY)
        self.git('config', 'githubmerge.branch', TARGET_BRANCH)
        self.git('config', 'githubmerge.host', FIXTURE_HOST)
        # Send the tool's fetch of the upstream repository to the fixture's own bare upstream, so
        # the run exercises the real fetch path without reaching any network.
        self.git('config', f'url.{self.upstream}.insteadOf',
                 f'{FIXTURE_HOST}/{FIXTURE_REPOSITORY}.git')

    def environment(self):
        environment = dict(os.environ)
        environment['GIT_CONFIG_NOSYSTEM'] = '1'
        environment['GIT_CONFIG_GLOBAL'] = '/dev/null'
        environment['GIT_TERMINAL_PROMPT'] = '0'
        # The driver imports the tool from the repository, and a cache directory written next to
        # it would leave the working tree dirty, which the merge workflow itself refuses to run on.
        environment['PYTHONDONTWRITEBYTECODE'] = '1'
        # With no githubmerge.testcmd configured the tool opens an inspection shell before the
        # signing prompt. A shell that exits immediately keeps the run non-interactive.
        environment['SHELL'] = '/bin/true'
        return environment

    def git(self, *arguments):
        return subprocess.run(['git', *arguments], cwd=self.root, env=self.environment(),
                              check=True, capture_output=True, text=True).stdout

    def write(self, relative_path, content):
        path = self.root / relative_path
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding='utf-8')

    def link(self, relative_path, target):
        path = self.root / relative_path
        if path.is_symlink() or path.exists():
            path.unlink()
        path.symlink_to(target)

    def link_bytes(self, relative_path, target):
        """Create a link from literal bytes, which a tree accepts whether or not they decode."""
        path = os.path.join(os.fsencode(self.root), relative_path)
        if os.path.lexists(path):
            os.unlink(path)
        os.symlink(target, path)

    def remove(self, relative_path):
        (self.root / relative_path).unlink()

    def commit(self, message):
        self.git('add', '--all')
        self.git('-c', 'core.hooksPath=/dev/null', 'commit', '--quiet', '--allow-empty',
                 '-m', message)
        return self.git('rev-parse', 'HEAD').strip()

    def branch(self, name):
        self.git('checkout', '--quiet', '-b', name)

    def checkout(self, name):
        self.git('checkout', '--quiet', name)

    def publish_pull_request(self, head_commit):
        """Publish the target branch and the pull request's refs the way the forge exposes them."""
        self.git('push', '--quiet', str(self.upstream),
                 f'+{TARGET_BRANCH}:refs/heads/{TARGET_BRANCH}',
                 f'+{head_commit}:refs/pull/{PULL_REQUEST}/head',
                 f'+{head_commit}:refs/pull/{PULL_REQUEST}/merge')

    def merge(self, *arguments, answer='x\n'):
        return subprocess.run(
            [sys.executable, str(self.driver), str(MERGE_TOOL), *arguments,
             PULL_REQUEST, TARGET_BRANCH],
            cwd=self.root, env=self.environment(), input=answer,
            capture_output=True, text=True, timeout=300)


class SymlinkDeclarationTestCase(unittest.TestCase):
    """Base case building one fixture per test in its own temporary directory."""

    def setUp(self):
        self.temporary_directory = tempfile.TemporaryDirectory(
            prefix='test-github-merge-symlinks.')
        self.addCleanup(self.temporary_directory.cleanup)
        root = Path(self.temporary_directory.name)
        driver = root / 'driver.py'
        driver.write_text(DRIVER, encoding='utf-8')
        work = root / 'work'
        work.mkdir()
        self.fixture = MergeFixture(work, driver)
        self.fixture.write('README.md', 'fixture\n')
        self.base_commit = self.fixture.commit('Initial fixture')

    def open_pull_request(self):
        self.fixture.branch(f'pull-request-{PULL_REQUEST}')

    def close_pull_request(self, head_commit):
        self.fixture.checkout(TARGET_BRANCH)
        self.fixture.publish_pull_request(head_commit)

    def assertRefused(self, result, path, commit):
        """Assert the run refused a link, naming the commit that carries it."""
        self.assertEqual(result.returncode, EXIT_REFUSED, result.stdout + result.stderr)
        self.assertIn(f"ERROR: File '{path}' was a symlink in commit {commit}", result.stdout)

    def assertReachedSigning(self, result):
        """Assert the run passed the check and stopped where the maintainer would sign."""
        self.assertEqual(result.returncode, EXIT_NOT_SIGNED, result.stdout + result.stderr)
        self.assertNotIn('was a symlink', result.stdout)
        self.assertIn('Not signing off on merge', result.stderr)


class DeclaredLinksTest(SymlinkDeclarationTestCase):

    def it_should_accept_a_declared_symbolic_link(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        head_commit = self.fixture.commit('Add a declared symbolic link')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn("Accepted symlink: '.dockerignore' -> '.containerignore': " + REASON,
                      result.stdout)

    def it_should_accept_a_declared_link_the_base_branch_already_carries(self):
        # The blocked sibling repository's case: the link and its declaration are already on the
        # target branch, and an unrelated pull request must still be mergeable.

        # Arrange
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        self.fixture.commit('Carry a declared symbolic link on the target branch')
        self.open_pull_request()
        self.fixture.write('feature.md', 'unrelated change\n')
        head_commit = self.fixture.commit('Change something unrelated')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn("Accepted symlink: '.dockerignore' -> '.containerignore'", result.stdout)
        self.assertNotIn('is stale', result.stdout)

    def it_should_refuse_an_undeclared_symbolic_link(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        head_commit = self.fixture.commit('Add an undeclared symbolic link')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_refuse_a_link_declared_with_a_different_target(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.gitignore', REASON)))
        head_commit = self.fixture.commit('Declare a symbolic link with the wrong target')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_refuse_a_link_whose_target_is_not_valid_utf8(self):
        # A tree stores a link target as bytes, and nothing requires them to decode. Matching a
        # decoded rendering would admit this link: b'\xff\xfe' rendered with a replacing decoder
        # is exactly the two replacement characters the declaration names below, and every other
        # undecodable target of the same length would render the same way. Matching bytes refuses
        # it, because no text can encode to a sequence that is not valid UTF-8.

        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link_bytes(b'.dockerignore', b'\xff\xfe')
        replaced = chr(0xfffd) * 2  # what a replacing decoder makes of the two bytes above
        self.fixture.write(DECLARATION, declaration_document(
            ('.dockerignore', replaced, REASON)))
        head_commit = self.fixture.commit('Declare a target that is not valid UTF-8')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_accept_a_declared_target_outside_ascii(self):
        # The other side of the same rule: a target that does decode still matches, byte for byte.

        # Arrange
        target = '.containerignore' + chr(0xe9)  # a target ending outside ASCII
        self.open_pull_request()
        self.fixture.write(target, 'target\n')
        self.fixture.link('.dockerignore', target)
        self.fixture.write(DECLARATION, declaration_document(('.dockerignore', target, REASON)))
        head_commit = self.fixture.commit('Declare a symbolic link with a target outside ASCII')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn(f"Accepted symlink: '.dockerignore' -> '{target}': " + REASON, result.stdout)

    def it_should_accept_a_declared_path_outside_ascii(self):
        # A path is tree bytes as well. Listing a tree without '-z' renders a path outside ASCII
        # as a quoted escape sequence under the default 'core.quotePath', so a declaration naming
        # the path itself would never match it.

        # Arrange
        path = 'dockerignore' + chr(0xe9) + '.link'  # a path outside ASCII
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link_bytes(path.encode('utf-8'), b'.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document((path, '.containerignore', REASON)))
        head_commit = self.fixture.commit('Declare a symbolic link at a path outside ASCII')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn(f"Accepted symlink: '{path}' -> '.containerignore': " + REASON, result.stdout)

    def it_should_refuse_a_declared_target_that_escapes_the_repository(self):
        # Arrange
        self.open_pull_request()
        self.fixture.link('.dockerignore', '../outside-the-repository')
        self.fixture.write(DECLARATION, declaration_document(
            ('.dockerignore', '../outside-the-repository', REASON)))
        head_commit = self.fixture.commit('Declare an escaping symbolic-link target')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)

    def it_should_refuse_a_declared_absolute_target(self):
        # Arrange
        self.open_pull_request()
        self.fixture.link('.dockerignore', '/etc/hostname')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '/etc/hostname', REASON)))
        head_commit = self.fixture.commit('Declare an absolute symbolic-link target')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)


class DeclarationSourceTest(SymlinkDeclarationTestCase):

    def it_should_exempt_nothing_without_the_declaration_argument(self):
        # The declaration reaches the merged result, and is still not read: an exemption is only
        # ever granted by the invocation that names a declaration path.

        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        head_commit = self.fixture.commit('Add a declared symbolic link')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge()

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)
        self.assertNotIn('is stale', result.stdout)

    def it_should_exempt_nothing_when_the_merged_result_carries_no_declaration(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        head_commit = self.fixture.commit('Add a symbolic link with no declaration anywhere')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_ignore_a_declaration_that_only_exists_in_the_working_directory(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        head_commit = self.fixture.commit('Add an undeclared symbolic link')
        self.close_pull_request(head_commit)
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_ignore_a_declaration_the_merged_result_does_not_carry(self):
        # The declaration is committed on the target branch, and the pull request deletes it
        # while adding the link, so the merged result has the link and no declaration.

        # Arrange
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        self.fixture.commit('Declare a symbolic link on the target branch')
        self.open_pull_request()
        self.fixture.remove(DECLARATION)
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        head_commit = self.fixture.commit('Add the link and drop the declaration')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertNotIn('Accepted symlink', result.stdout)

    def it_should_ignore_a_declaration_that_cannot_be_read(self):
        # A broken declaration can never widen what is accepted; it refuses and says why.

        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION, 'this is not JSON\n')
        head_commit = self.fixture.commit('Commit a declaration that is not a declaration')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', head_commit)
        self.assertIn(f"WARNING: Ignoring the symlink declaration '{DECLARATION}'", result.stdout)


class CheckedRangeTest(SymlinkDeclarationTestCase):

    def it_should_refuse_a_link_only_an_intermediate_commit_carries(self):
        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        intermediate_commit = self.fixture.commit('Add a symbolic link')
        self.fixture.remove('.dockerignore')
        head_commit = self.fixture.commit('Remove the symbolic link again')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', intermediate_commit)
        self.assertNotIn(head_commit, result.stdout)

    def it_should_refuse_a_link_whose_target_changes_within_the_range(self):
        # The declaration matches the target the tip carries, and the earlier commit points the
        # same path somewhere else, so that commit refuses while the tip is admitted.

        # Arrange
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.write('.gitignore', 'other target\n')
        self.fixture.link('.dockerignore', '.gitignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        intermediate_commit = self.fixture.commit('Point the link at one target')
        self.fixture.link('.dockerignore', '.containerignore')
        head_commit = self.fixture.commit('Point the link at the declared target')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', intermediate_commit)
        self.assertNotIn(f"was a symlink in commit {head_commit}", result.stdout)

    def it_should_not_walk_history_the_merge_does_not_introduce(self):
        # An undeclared link that existed once on the target branch and was removed before the
        # pull request branched is not part of what the merge introduces.

        # Arrange
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        historical_commit = self.fixture.commit('Carry a symbolic link once')
        self.fixture.remove('.dockerignore')
        self.fixture.commit('Remove the symbolic link from the target branch')
        self.open_pull_request()
        self.fixture.write('feature.md', 'unrelated change\n')
        head_commit = self.fixture.commit('Change something unrelated')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertNotIn(historical_commit, result.stdout)

    def it_should_pass_a_removal_that_keeps_the_declaration_entry(self):
        # Removing a declared link takes two changes: the pull request that deletes the link
        # keeps the entry, because its own pre-deletion commit is judged against the final
        # declaration. The retained entry is then reported as stale rather than refused.

        # Arrange
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        self.fixture.commit('Carry a declared symbolic link on the target branch')
        self.open_pull_request()
        self.fixture.write('feature.md', 'unrelated change\n')
        self.fixture.commit('Change something while the link is still there')
        self.fixture.remove('.dockerignore')
        head_commit = self.fixture.commit('Remove the declared symbolic link')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn("Accepted symlink: '.dockerignore' -> '.containerignore'", result.stdout)
        self.assertIn("WARNING: Declared symlink '.dockerignore' is not a symbolic link in the"
                      " merged result", result.stdout)

    def it_should_refuse_a_removal_that_drops_the_declaration_entry(self):
        # The other half of the same rule: dropping the entry in the removing pull request
        # leaves its pre-deletion commit judged by a declaration that no longer covers the link.

        # Arrange
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', REASON)))
        self.fixture.commit('Carry a declared symbolic link on the target branch')
        self.open_pull_request()
        self.fixture.write('feature.md', 'unrelated change\n')
        pre_deletion_commit = self.fixture.commit('Change something while the link is still there')
        self.fixture.remove('.dockerignore')
        self.fixture.write(DECLARATION, declaration_document())
        head_commit = self.fixture.commit('Remove the link and its declaration entry')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertRefused(result, '.dockerignore', pre_deletion_commit)
        self.assertNotIn(f"was a symlink in commit {head_commit}", result.stdout)


class ReportRenderingTest(SymlinkDeclarationTestCase):

    def it_should_escape_a_control_character_in_an_accepted_target(self):
        # A link target is tree content, so it can carry a newline followed by the text of a
        # refusal. Printed as it stands it would add a line the tool never wrote.

        # Arrange
        forged = '.containerignore\nERROR: File forged is refused in commit 0000000'
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', forged)
        self.fixture.write(DECLARATION, declaration_document(('.dockerignore', forged, REASON)))
        head_commit = self.fixture.commit('Declare a link whose target carries a newline')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn("Accepted symlink: '.dockerignore' -> '.containerignore\\nERROR: File forged",
                      result.stdout)
        self.assertNotIn('\nERROR: File forged', result.stdout)

    def it_should_escape_a_control_character_in_a_declaration_reason(self):
        # The reason is tree content too, and reaches the same report.

        # Arrange
        forged = 'It exists.\nERROR: File forged is refused in commit 0000000'
        self.open_pull_request()
        self.fixture.write('.containerignore', 'target\n')
        self.fixture.link('.dockerignore', '.containerignore')
        self.fixture.write(DECLARATION,
                           declaration_document(('.dockerignore', '.containerignore', forged)))
        head_commit = self.fixture.commit('Declare a link with a reason carrying a newline')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', DECLARATION)

        # Assert
        self.assertReachedSigning(result)
        self.assertIn("Accepted symlink: '.dockerignore' -> '.containerignore':"
                      ' It exists.ERROR: File forged is refused in commit 0000000',
                      result.stdout)
        self.assertNotIn('\nERROR: File forged', result.stdout)


class DeclarationArgumentTest(SymlinkDeclarationTestCase):

    def it_should_reject_an_absolute_declaration_path(self):
        # Arrange
        self.open_pull_request()
        head_commit = self.fixture.commit('Change nothing in particular')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', '/etc/symlinks.json')

        # Assert
        self.assertEqual(result.returncode, EXIT_USAGE_ERROR, result.stdout + result.stderr)
        self.assertIn('is absolute', result.stderr)

    def it_should_reject_a_declaration_path_that_escapes_the_repository(self):
        # Arrange
        self.open_pull_request()
        head_commit = self.fixture.commit('Change nothing in particular')
        self.close_pull_request(head_commit)

        # Act
        result = self.fixture.merge('--symlinks', '../symlinks.json')

        # Assert
        self.assertEqual(result.returncode, EXIT_USAGE_ERROR, result.stdout + result.stderr)
        self.assertIn('escapes the repository', result.stderr)


def load_tests(loader, tests, pattern):
    """Collect the behaviour-named tests, which do not use the default 'test' prefix."""
    loader.testMethodPrefix = 'it_should'
    suite = unittest.TestSuite()
    for case in (DeclaredLinksTest, DeclarationSourceTest, CheckedRangeTest,
                 ReportRenderingTest, DeclarationArgumentTest):
        suite.addTests(loader.loadTestsFromTestCase(case))
    return suite


if __name__ == '__main__':
    unittest.main(verbosity=2)
