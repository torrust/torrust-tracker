---
date-analyzed: 2026-10-08
source: Trivy GitHub Code Scanning
status: non-affecting
review-cadence: quarterly
requires-recheck-when: the tracker begins using an affected C/C++ API or the production execution model changes
semantic-links:
  skill-links:
    - catalog-security-vulnerabilities
  related-artifacts:
    - Containerfile
    - docs/security/analysis/github-security/reviews/2026-10-08.md
---

# Runtime CVEs from the 2026-10-08 GitHub Code Scanning Review

## Context

The GitHub Code Scanning review returned 27 runtime-image alerts that were not already covered by
an individual active investigation. They affect glibc or GCC runtime packages present in the
distroless production image.

Package presence does not establish reachability. The affected APIs and deployment conditions were
compared with the tracker runtime before assigning a non-affecting verdict.

## Finding Groups

### Historical glibc findings

| CVE | Alert | Relevant missing condition |
| --- | ---: | --- |
| CVE-2010-4756 | 17 | Attacker-controlled glob pattern reaches the affected implementation |
| CVE-2018-20796 | 18 | Attacker-controlled regular expression reaches the affected implementation |
| CVE-2019-1010022 | 19 | Tracker invokes the affected glibc API and condition |
| CVE-2019-1010023 | 20 | Tracker invokes the affected glibc API and condition |
| CVE-2019-1010024 | 21 | Tracker invokes the affected glibc API and condition |
| CVE-2019-1010025 | 22 | Tracker invokes the affected glibc API and condition |
| CVE-2019-9192 | 23 | Runtime executes `ldd` against attacker-controlled ELF files |

These historical low-severity findings require APIs or execution conditions absent from the
tracker service.

### Function- or deployment-specific glibc findings

| CVE | Alert | Relevant missing condition |
| --- | ---: | --- |
| CVE-2026-6791 | 25 | Attacker-controlled input reaches the affected function |
| CVE-2026-6368 | 26 | Attacker-controlled input reaches the affected function |
| CVE-2026-19542 | 33 | Attacker-controlled `fopen` mode reaches the affected parser |
| CVE-2026-19499 | 52 | Tracker uses the affected `strfmon` path |
| CVE-2026-77117 | 53 | Tracker uses the affected `tdelete` path |
| CVE-2026-18374 | 54 | Tracker uses the affected `wordexp` path |
| CVE-2026-80489 | 55 | Tracker converts attacker-controlled text through the affected iconv character set |
| CVE-2026-89092 | 62 | Production runtime enables the affected `nscd` behavior |
| CVE-2026-8674 | 63 | Tracker uses the affected function with attacker-controlled input |
| CVE-2026-86805 | 67 | Runtime executes a setuid/setgid binary through the affected `$ORIGIN` path |
| CVE-2026-95818 | 68 | Tracker uses the affected function and trigger condition |
| CVE-2026-97399 | 106 | Tracker uses the affected function and trigger condition |

The production image contains no tracker-controlled setuid/setgid execution path, does not run
`nscd`, and does not invoke the specialized APIs named above with attacker-controlled input.

### GCC runtime package findings

| CVE | Alerts | Packages | Relevant missing condition |
| --- | --- | --- | --- |
| CVE-2026-102010 | 138–141 | `gcc-14-base`, `libgcc-s1`, `libgomp1`, `libstdc++6` | Tracker executes the affected C++ binary-heap `erase_if` path |
| CVE-2026-95619 | 142–145 | `gcc-14-base`, `libgcc-s1`, `libgomp1`, `libstdc++6` | Tracker executes the affected aligned-`new` path with attacker-controlled size |

The four package occurrences per CVE are duplicate package-level reports. The Rust tracker does not
use the affected C++ APIs.

## Verdict

These 27 findings are non-affecting under the current tracker code and production execution model.
They remain cataloged because a future native dependency, FFI call, privileged execution path, or
runtime-service change could invalidate the verdict.

The two native zlib findings are excluded from this verdict and tracked separately because the
release binary links `libz.so.1`.

## Recheck Triggers

- A new C or C++ dependency or FFI boundary is added.
- Tracker code begins calling one of the affected APIs.
- The production image adds `nscd`, setuid/setgid binaries, or execution of untrusted ELF files.
- Runtime input begins reaching glob, regular-expression, iconv, `wordexp`, or specialized glibc
  parsing paths.
- A new advisory demonstrates impact without the currently missing condition.

## Evidence

- GitHub Code Scanning alerts 17–23, 25–26, 33, 52–55, 62–63, 67–68, 106, and 138–145.
- `Containerfile` production runtime composition.
- Dated coverage reconciliation:
  [`../github-security/reviews/2026-10-08.md`](../github-security/reviews/2026-10-08.md).
