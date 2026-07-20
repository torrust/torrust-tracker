# Torrust Tracker - Security Scans

Security scan history for the `torrust-tracker` Docker image.

## Current Status

| Stage   | MEDIUM | HIGH | CRITICAL | Status   | Last Scan    |
| ------- | ------ | ---- | -------- | -------- | ------------ |
| release | 5      | 0    | 0        | ✅ Clean | Jul 20, 2026 |

## Build & Scan Commands

**Build the image**:

```bash
docker build -t torrust-tracker:local -f Containerfile .
```

**Run Trivy security scan**:

```bash
trivy image --severity HIGH,CRITICAL torrust-tracker:local
```

**Full scan with all severities**:

```bash
trivy image --severity MEDIUM,HIGH,CRITICAL torrust-tracker:local
```

## Scan History

### July 20, 2026 - Post-build-stage-minimization verification

**Image**: `torrust-tracker:1463-gcc`
**Image ID**: `sha256:3b8859fd30f921d4be511efb5a9578252841c624bf3f895057cd46b795666687`
**Runtime base digest**: `gcr.io/distroless/cc-debian13@sha256:3be83724bcda99b72307e8d3cea256b3cfa5678b5198c1351bf66d2bc60d9cf9`
**Trivy Version**: 0.69.3
**Vulnerability DB Updated**: 2026-07-20 13:19:47 UTC
**Base OS**: Debian 13.6 (trixie, distroless/cc-debian13)
**Status**: ✅ **Clean** - 5 MEDIUM, 0 HIGH, 0 CRITICAL

The rebuilt release image is 188.6 MB and contains 13 OS packages. A full-severity scan
also reported 7 LOW findings, for 12 findings total. The five MEDIUM findings and affected
packages (`libc6` and `zlib1g`) are unchanged from the June baseline below. The independent
chef, tester, and GCC image reductions therefore did not regress the deployed artifact.

The image was also started locally and its built-in health check returned repeated
`200 OK` responses with container status `healthy`.

### June 29, 2026 - Baseline

**Image**: `torrust-tracker:local`
**Trivy Version**: 0.69.3
**Scan Mode**: `--severity MEDIUM,HIGH,CRITICAL`
**Base OS**: Debian 13.5 (trixie, distroless/cc-debian13)
**Status**: ✅ **Clean** — 5 MEDIUM, 0 HIGH, 0 CRITICAL

#### Summary

This is the baseline scan for the Torrust Tracker production runtime image. The image uses
`gcr.io/distroless/cc-debian13` as the runtime base, which is a minimal distroless image.
All 5 findings are MEDIUM-severity CVEs in OS base libraries (`libc6` and `zlib1g`).

#### Vulnerability Details (all MEDIUM)

| CVE            | Package | Installed Version           | Title                                                                          |
| -------------- | ------- | --------------------------- | ------------------------------------------------------------------------------ |
| CVE-2026-5435  | libc6   | 2.41-12+deb13u3             | glibc: Out-of-bounds write via TSIG record processing                          |
| CVE-2026-5450  | libc6   | 2.41-12+deb13u3             | glibc: Heap Buffer Overflow in `scanf` with `%mc` format specifier             |
| CVE-2026-5928  | libc6   | 2.41-12+deb13u3             | glibc: Information disclosure or denial of service via `ungetwc` function      |
| CVE-2026-6238  | libc6   | 2.41-12+deb13u3             | glibc: Application crash or uninitialized memory read via crafted DNS response |
| CVE-2026-27171 | zlib1g  | 1:1.3.dfsg+really1.3.1-1+b1 | zlib: Denial of Service via infinite loop in CRC32 combine functions           |

#### Analysis

- **CVE-2026-5435** (TSIG record processing): Affects DNS TSIG record handling in glibc.
  The tracker server performs **no DNS resolution** in its production code paths. Peer IP
  resolution is done from HTTP headers (`X-Forwarded-For`) or socket addresses, not DNS.
  The only DNS resolution occurs inside `sqlx` lazily when connecting to MySQL/PostgreSQL
  databases, which does not use glibc's TSIG record processing path. **Non-affecting**.

- **CVE-2026-5450** (scanf `%mc`): Heap buffer overflow in the `scanf` family with the
  `%mc` format specifier. The tracker codebase contains **zero uses** of `scanf`, `sscanf`,
  or any C stdio input functions. Input parsing is done entirely in safe Rust.
  **Non-affecting**.

- **CVE-2026-5928** (ungetwc): Information disclosure or DoS via `ungetwc`. The tracker
  does not use wide character I/O functions (`ungetwc`, `fgetwc`, etc.). **Non-affecting**.

- **CVE-2026-6238** (DNS response): Crash or uninitialized memory read via crafted DNS
  response. This affects glibc's internal DNS resolution (`gethostbyname`, `res_nquery`,
  etc.). The tracker server performs **no DNS resolution** directly — peer IP resolution
  is from HTTP headers/socket addresses, not hostname lookup. Database hostname resolution
  is done lazily inside `sqlx` at first query time, which does not trigger the affected
  code path. **Non-affecting**.

- **CVE-2026-27171** (zlib CRC32): DoS via infinite loop in CRC32 combine function.
  The tracker uses `tower-http` with `compression-full` for optional HTTP response
  compression middleware (gzip, brotli, zstd). However, this uses `flate2` → `miniz_oxide`
  (pure Rust implementation of zlib), **not** the system `zlib1g` library. The system `zlib1g`
  is only pulled in as a transitive dependency of distroless base OS packages, not used by
  the tracker binary itself. Additionally, CRC32 combine is a specialized function not
  exercised by normal compression/decompression. **Non-affecting**.

#### Next Steps

- All 5 MEDIUM CVEs are **non-affecting** for the current runtime — accepted risk.
- Re-scan quarterly to track OS package updates from the distroless base image.
- If the base image is updated, re-scan and update this report.
- Consider filing issues for specific CVEs if they become fixable (e.g., via Debian security
  updates to the distroless base).
