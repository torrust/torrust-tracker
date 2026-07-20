# Docker Image Security

This directory covers security scanning for the Torrust Tracker Docker image.

## Purpose

Regular security scanning ensures that the tracker's container image is free from known
vulnerabilities. This documentation provides:

- Instructions for running security scans on the tracker image
- Scan history and current status
- Vulnerability management decisions

## Automated Scanning

See the [Security Scan workflow](../../../.github/workflows/security-scan.yaml) for automated
scheduled scanning via GitHub Actions.

## Manual Scanning with Trivy

### Installation

```bash
# macOS
brew install trivy

# Linux (Debian/Ubuntu)
sudo apt-get install trivy

# Or use Docker
docker run --rm aquasec/trivy:latest image <image-name>
```

### Scan Commands

**Build the image**:

```bash
docker build -t torrust-tracker:local -f Containerfile .
```

**Scan for HIGH and CRITICAL only** (standard production check):

```bash
trivy image --severity HIGH,CRITICAL torrust-tracker:local
```

**Scan with all severities** (full report):

```bash
trivy image --severity MEDIUM,HIGH,CRITICAL torrust-tracker:local
```

### Build-stage scans

Build and scan the foundational stages after changing their base images or installed
packages, and during the quarterly security review:

```bash
for stage in chef tester gcc; do
  docker build --target "$stage" --tag "torrust-tracker:$stage-local" \
    --file Containerfile .
  trivy image --scanners vuln "torrust-tracker:$stage-local"
done
```

Record these results in [`scans/build-images.md`](scans/build-images.md), separately from
the deployed release image. Use the same Trivy version and vulnerability database for all
comparisons.

### Severity Levels

- `CRITICAL`: Exploitable vulnerabilities with severe impact
- `HIGH`: Significant vulnerabilities requiring attention
- `MEDIUM`: Moderate vulnerabilities (tracked for awareness)

## Scan Results

See [`scans/`](scans/) for the full scan history.
