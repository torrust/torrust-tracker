---
semantic-links:
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - contrib/dev-tools/git/hooks/pre-push.sh
    - contrib/dev-tools/git/install-git-hooks.sh
---

# Git Hooks

The repository's pre-commit and pre-push hooks run local validation before Git creates a commit
or updates a remote branch. The pre-push hook runs nightly checks and the full stable test suite,
so it can take several minutes.

## SSH Idle Timeouts During Pushes

Git can open its SSH connection to the remote before it runs the pre-push hook. If an SSH route
closes idle connections while the hook is running, a successful hook can be followed by an error
such as `Connection to ssh.github.com closed by remote host` or a push exit status of `141`.

Configure periodic SSH traffic for this checkout to prevent an idle timeout without changing your
machine-wide SSH behavior:

```sh
git config --local core.sshCommand 'ssh -o ServerAliveInterval=30 -o ServerAliveCountMax=20'
```

Verify the repository-local setting with:

```sh
git config --local --get core.sshCommand
```

To apply the same behavior to all GitHub SSH connections, add these options to a `Host github.com
ssh.github.com` entry in `~/.ssh/config` instead:

```text
Host github.com ssh.github.com
  ServerAliveInterval 30
  ServerAliveCountMax 20
```

Use only one configuration approach unless you need a different setting for this repository. The
repository-local Git configuration is the preferred option when the timeout affects one checkout.
