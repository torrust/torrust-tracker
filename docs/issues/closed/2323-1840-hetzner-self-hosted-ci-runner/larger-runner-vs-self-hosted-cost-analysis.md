# Should torrust-tracker use a larger GitHub runner or self-hosted (Hetzner)?

> **Provenance**: AI-assisted analysis prepared for @josecelano on 2026-09-16, copied into this
> issue folder on 2026-09-24. The content is unchanged except for blank-line fixes required by
> markdownlint. Its **Recommendation** and **Resolution** sections were superseded by the decision
> recorded in issue #2323 (self-hosted Hetzner runner for the container test job); see
> [ISSUE.md](ISSUE.md). Its runner premise is also outdated: standard Linux runners for public
> repositories have 4 vCPUs, not 2, so its per-core job times do not hold. ISSUE.md derives the
> larger-runner cost from the T1 baseline instead.

## Environment

- Repo: `torrust/torrust-tracker` (public)
- Date: 2026-09-16
- Trigger: PR checks reported as taking up to ~40 minutes

## Question

Would moving to a GitHub Team subscription and using a larger GitHub-hosted
runner make PR checks pass faster, and is it worth the cost compared to a
self-hosted runner (e.g. on Hetzner)?

## Diagnosis Steps

- [x] List recent workflow runs via `gh run list --repo torrust/torrust-tracker`
- [x] Identify which workflow was actually ~40 min (it was **"Container"**, not
      "Testing" — "Testing" only takes ~12-13 min)
- [x] Break down job/step timing via `gh run view <id> --json jobs` and
      `gh api repos/.../actions/jobs/<id>` to find the slow step
- [x] Count monthly PR-triggered run volume for the slow workflow
- [x] Compare GitHub-hosted larger runner pricing vs. self-hosted (Hetzner) pricing

## Root Cause

In the `Container` workflow, the job **`Test (Docker) (release)`** takes
~34m48s total, and one single step — **`Build Tracker Image`** (a Docker
`release`-profile build via `docker/build-push-action`) — accounts for
**~28 minutes (82%)** of that time. Everything else (E2E tests, qBittorrent
tests, checkout, buildx setup) is a few minutes combined.

This step compiles the whole workspace with a release/LTO profile on every PR,
even though a `type=gha` Docker layer cache is configured — worth confirming
whether that cache is actually being hit before spending money on a bigger
runner.

Run volume: **378 PR-triggered "Container" runs in the last 30 days** (plus 55
push-triggered runs), queried via:

```bash
gh api "repos/torrust/torrust-tracker/actions/workflows/<id>/runs?event=pull_request&created=>...&per_page=1" --jq '.total_count'
```

## Options Considered

### 1. GitHub-hosted larger runners

- Requires **GitHub Team** ($4/user/month) or Enterprise Cloud — larger
  runners are not available on Free/Pro, and **included free minutes don't
  apply to larger runners** (billed from minute 1, even for public repos).
- Rates (per-minute, x64 Linux): 2-core $0.006 (current, free on public
  repos), 4-core $0.012, 8-core $0.022, 16-core $0.042, 32-core $0.082.
- Estimated cost for the `Container` workflow's slow job, at 378 runs/month:

  | Runner | Est. build step | Est. job total | Cost/month |
  |---|---|---|---|
  | 2-core (current) | 28 min | ~35 min | $0 (free, public repo) |
  | 4-core | ~15 min | ~22 min | ~$99 |
  | 8-core | ~9 min | ~16 min | ~$132 |
  | 16-core | ~6 min | ~13 min | ~$208 |

  Plus Team seat cost ($4/user/month).

### 2. Self-hosted runner (Hetzner Cloud)

Real Hetzner pricing (excl. VAT):

  | Instance | vCPU | RAM | Type | Price/month |
  |---|---|---|---|---|
  | CPX32 | 4 (shared) | 8 GB | Regular Performance | €35.99 (~$39) |
  | CPX42 | 8 (shared) | 16 GB | Regular Performance | €69.99 (~$76) |
  | CCX23 | 4 (dedicated) | 16 GB | General Purpose | €86.49 (~$94) |
  | CCX33 | 8 (dedicated) | 32 GB | General Purpose | €138.99 (~$151) |

- Self-hosted runner minutes are **always free** on GitHub's Actions billing
  (no per-minute charge), but you pay the flat Hetzner server cost regardless
  of usage — this only wins financially at higher run volumes than 378/month.
- At current volume, self-hosted (~$151/mo for CCX33) is roughly **on par**
  with the GitHub 8-core option (~$132/mo) — not "much cheaper."
- **Security risk**: `torrust-tracker` is public and accepts external PRs.
  Self-hosted runners execute PR workflow code on your own infrastructure —
  a malicious PR could exfiltrate secrets or pivot into other systems. GitHub
  discourages self-hosted runners for public repos for exactly this reason.
  Mitigating this properly requires ephemeral/single-use VMs and restricting
  self-hosted runners to trusted-only events (e.g. `push` to `develop`/`main`,
  never `pull_request` from forks).

## Recommendation

1. **First (free)**: verify the `type=gha` Docker cache is actually being hit
   on the `Build Tracker Image` step — a 28-minute release build with caching
   configured suggests a possible cache-miss bug, which would be the highest
   ROI fix.
2. **If still slow**: adopt a GitHub-hosted **8-core** larger runner
   (`linux_8_core`, ~$132/month at current volume) for just the `Container`
   workflow's slow job — best cost-per-minute-saved ratio, no added security
   surface, no infra to maintain. Start at 4-core to validate real-world
   improvement before going to 8-core.
3. **Skip self-hosted** — at this run volume the cost is roughly equal to
   GitHub's larger runner, so it isn't worth taking on the security exposure
   and maintenance burden of running CI for a public repo with external
   contributors on your own infrastructure.

## Resolution

Decision made: pursue GitHub-hosted larger runner (starting at 4-core) after
first confirming the Docker build cache hit rate. Self-hosted on Hetzner
rejected due to marginal cost savings not justifying the security tradeoff.
