# Runner Server Setup Log

<!-- cspell:ignore passwordauthentication kbdinteractiveauthentication permitrootlogin publickey -->

Step-by-step record of how the self-hosted GitHub Actions runner server for issue #2323 was set
up, so it can be reproduced or rebuilt. See [ISSUE.md](ISSUE.md) for the plan (task T2 and T3).

Conventions:

- `<runner-ip>` stands for the server's public IPv4 address. Do not commit the real address,
  passwords, private keys, or runner registration tokens to this repository.
- `desktop$` marks commands run on the maintainer's machine; `server#` marks commands run on the
  server as `root`.

## Server

| Property   | Value                                             |
| ---------- | ------------------------------------------------- |
| Provider   | Hetzner Cloud                                     |
| Location   | Falkenstein, Germany                              |
| Resources  | 8 vCPU, 16 GB RAM, 320 GB local disk              |
| Traffic    | 20 TB outgoing per month included                 |
| Price      | €69.49 per month                                  |
| Hostname   | `torrust-runner-01`                               |
| OS         | Ubuntu 26.04.1 LTS, kernel `7.0.0-34-generic`     |
| Disk (`/`) | 300 GB usable                                     |
| Network    | Public IPv4 and IPv6                              |
| SSH access | Dedicated Ed25519 key; password login disabled    |

## 1. First Login

The server was created without an SSH key, so Hetzner emailed a root password and enabled
password login.

A plain `ssh root@<runner-ip>` failed with:

```text
Received disconnect from <runner-ip> port 22:2: Too many authentication failures
```

Cause: the local SSH agent offered every loaded key before trying the password, and the server
disconnected after reaching its authentication-attempt limit. Fix: disable public-key
authentication for this login so the client goes straight to the password:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

The Hetzner web console is an alternative when SSH is not reachable at all.

## 2. Create a Dedicated SSH Key

Use a key only for this server, protected by a passphrase. `-a 100` increases the key-derivation
rounds that protect the private key file.

```bash
desktop$ ssh-keygen -t ed25519 -a 100 -f ~/.ssh/torrust_ci_runner_ed25519 -C "torrust-runner-01"
```

Store the passphrase in a password manager. The private key never leaves the desktop.

## 3. Install the Public Key on the Server

```bash
desktop$ ssh-copy-id -o PubkeyAuthentication=no -o PreferredAuthentications=password \
  -i ~/.ssh/torrust_ci_runner_ed25519.pub root@<runner-ip>
```

Result (2026-09-24): `Number of key(s) added: 1`. The follow-up login command that
`ssh-copy-id` prints repeats `-o PubkeyAuthentication=no`, so it would not test the new key; use
the check in step 4 instead. Confirm on the server that only the intended key is installed:

```bash
server# cat /root/.ssh/authorized_keys
```

Expected: one `ssh-ed25519` line ending in `torrust-runner-01`. Verified on 2026-09-24.

Manual alternative, if `ssh-copy-id` is not available:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip> \
  'umask 077; mkdir -p ~/.ssh; cat >> ~/.ssh/authorized_keys' < ~/.ssh/torrust_ci_runner_ed25519.pub
```

## 4. Configure the SSH Client

Test the key directly first, in a **new** terminal, keeping the password session open as a
fallback:

```bash
desktop$ ssh -i ~/.ssh/torrust_ci_runner_ed25519 -o IdentitiesOnly=yes root@<runner-ip>
```

Result (2026-09-24): key login works and shows the Ubuntu 26.04.1 LTS welcome banner.

Then add a host entry to `~/.ssh/config` on the desktop. `IdentitiesOnly yes` makes the client
offer only this key, which avoids the "Too many authentication failures" error.

```text
Host torrust-runner-01
    HostName <runner-ip>
    User root
    IdentityFile ~/.ssh/torrust_ci_runner_ed25519
    IdentitiesOnly yes
```

Verify the alias:

```bash
desktop$ ssh torrust-runner-01
```

Result (2026-09-24): host entry added.

## 5. Disable Password Login

Only after step 4 succeeds. OpenSSH uses the first value it reads for each option, and the
`sshd_config.d/` files are read in lexical order, so a `00-` prefix takes precedence over
provider defaults such as `50-cloud-init.conf`.

```bash
server# cat > /etc/ssh/sshd_config.d/00-torrust-hardening.conf <<'EOF'
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin prohibit-password
EOF
server# sshd -t && systemctl reload ssh
server# sshd -T | grep -E '^(passwordauthentication|kbdinteractiveauthentication|permitrootlogin) '
```

Result (2026-09-24, Ubuntu 26.04.1 LTS):

```text
permitrootlogin prohibit-password
passwordauthentication no
kbdinteractiveauthentication no
```

Older OpenSSH versions print `without-password` for `prohibit-password`; both mean the same. On
distributions where the service is named `sshd`, use `systemctl reload sshd`.

Confirm from the desktop that password login is now refused:

```bash
desktop$ ssh -o PubkeyAuthentication=no -o PreferredAuthentications=password root@<runner-ip>
```

Expected: `Permission denied (publickey).`

Result (2026-09-24): `root@<runner-ip>: Permission denied (publickey).` Password login is
disabled.

## 6. Update the OS and Enable Automatic Security Updates

```bash
server# apt update && apt full-upgrade -y
server# apt install -y unattended-upgrades
server# cat /etc/apt/apt.conf.d/20auto-upgrades
server# systemctl is-enabled unattended-upgrades
server# test -f /var/run/reboot-required && cat /var/run/reboot-required
```

Expected: `20auto-upgrades` contains `APT::Periodic::Update-Package-Lists "1";` and
`APT::Periodic::Unattended-Upgrade "1";`, and the service is `enabled`. If it is missing, run
`dpkg-reconfigure -plow unattended-upgrades`. Reboot if `/var/run/reboot-required` exists.

Result (2026-09-24):

- `full-upgrade` installed kernel `7.0.0-34-generic` (running kernel was `7.0.0-30-generic`).
- `unattended-upgrades` 2.12ubuntu9 was already installed; `20auto-upgrades` has both settings
  set to `"1"`, and the service is `enabled`.
- `apt` reported `Not Upgrading: 1` (one package held back) and `*** System restart required ***`.

Reboot and confirm the new kernel and the held-back package:

```bash
server# reboot
desktop$ ssh torrust-runner-01
server# uname -r
server# apt list --upgradable
```

Expected: `uname -r` prints `7.0.0-34-generic`.

Result after reboot (2026-09-24): key login with `ssh torrust-runner-01` works, and the login
banner reports kernel `7.0.0-34-generic`. The held-back package is still to be identified with
`apt list --upgradable`.

The root password is still needed for the Hetzner web console; keep it in the password manager.

## Next Steps

- Restrict inbound traffic with a Hetzner Cloud Firewall: allow only SSH (TCP 22). The runner
  needs outbound HTTPS only; it never accepts inbound connections from GitHub.
- Create a non-root `runner` user; the GitHub runner refuses to run as `root` by default.
- Install Docker and add the `runner` user to the `docker` group.
- Install and register the GitHub Actions runner (T3), logged separately in
  `runner-agent-installation.md`.
