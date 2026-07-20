# Container Build Images - Security Scans

Security scan history for the foundational `chef`, `tester`, and `gcc` stages in the
Torrust Tracker `Containerfile`. These images are ephemeral build inputs. They are not
published or deployed, so their findings must not be interpreted as production exposure.

## Current Status

| Stage    | Base                 | Size       | Debian packages | UNKNOWN | LOW | MEDIUM | HIGH | CRITICAL | Total |
| -------- | -------------------- | ---------- | --------------- | ------- | --- | ------ | ---- | -------- | ----- |
| `chef`   | `rust:slim-trixie`   | 1,067.4 MB | 145             | 77      | 617 | 309    | 65   | 4        | 1,072 |
| `tester` | `rust:slim-trixie`   | 975.9 MB   | 123             | 79      | 579 | 301    | 51   | 4        | 1,014 |
| `gcc`    | `debian:trixie-slim` | 274.3 MB   | 114             | 77      | 577 | 299    | 51   | 4        | 1,008 |

## July 20, 2026 - Slim Build Stages

- Trivy version: 0.69.3
- Vulnerability database version: 2
- Database updated: 2026-07-20 13:19:47 UTC
- Detected OS: Debian 13.6
- Scan scope: OS and language-specific vulnerabilities reported by `trivy image --scanners vuln`

### Image identities

| Stage    | Local image ID                                                            |
| -------- | ------------------------------------------------------------------------- |
| `chef`   | `sha256:e8fac2fe73835c3c5a2762c4491dc48dd70fdfd03234955d9c28f67dcdd3aeda` |
| `tester` | `sha256:73696ee861456424ee3099d2c1a6b97071d93fb7a198ee28431e9d62dea30056` |
| `gcc`    | `sha256:3ff67dd07ecdc8208a37c6628235ef8ebaebfa1d469a72d1a055d23cb80a0485` |

The resolved base-image digests were:

- `rust:slim-trixie`: `sha256:5c6f46a6e4472ab1ca7ba7d494e6677f2f219ebc02f32025d3986f057635ec9c`
- `debian:trixie-slim`: `sha256:020c0d20b9880058cbe785a9db107156c3c75c2ac944a6aa7ab59f2add76a7bd`

### Comparison with replaced images

| Stage  | Previous image / result           | Final image / result              | Reduction                                  |
| ------ | --------------------------------- | --------------------------------- | ------------------------------------------ |
| `chef` | 1,662.7 MB / 455 packages / 2,148 | 1,067.4 MB / 145 packages / 1,072 | 595.3 MB / 310 packages / 1,076 findings   |
| `gcc`  | 1,556.4 MB / 464 packages / 2,165 | 274.3 MB / 114 packages / 1,008   | 1,282.1 MB / 350 packages / 1,157 findings |

The tester already used the slim Rust base. Its setup-only `curl` dependency is now purged;
the final stage retains `sqlite3`, `time`, and `cargo-nextest` and has 123 Debian packages.

## Reproduction

```bash
docker build --target chef --tag torrust-tracker:chef-local --file Containerfile .
docker build --target tester --tag torrust-tracker:tester-local --file Containerfile .
docker build --target gcc --tag torrust-tracker:gcc-local --file Containerfile .

docker image inspect torrust-tracker:chef-local --format '{{.Id}} {{.Size}}'
docker run --rm --entrypoint dpkg-query torrust-tracker:chef-local \
  -W '-f=${binary:Package}\n' | wc -l
trivy image --scanners vuln torrust-tracker:chef-local
```

Repeat the inspect, package-query, and Trivy commands for `tester-local` and `gcc-local`.
Scanner totals are comparable only when the Trivy version and vulnerability database are
held constant.

## Risk Context

The stages run only during `docker build`, expose no services, and are discarded after the
release image is assembled. Their packages can still affect build-chain integrity, so they
are scanned after base or package changes and during quarterly review. Daily automation
continues to scan the separately documented production image.

See the durable impact analysis in
[`../../analysis/build/2026-06-10_containerfile-trixie-cves.md`](../../analysis/build/2026-06-10_containerfile-trixie-cves.md).
