# Docker Image Scan Results

Historical security scan results for the Torrust Tracker Docker image.

## Current Status Summary

| Image             | Stage   | MEDIUM | HIGH | CRITICAL | Status   | Last Scan    | Details                    |
| ----------------- | ------- | ------ | ---- | -------- | -------- | ------------ | -------------------------- |
| `torrust-tracker` | release | 5      | 0    | 0        | ✅ Clean | Jun 29, 2026 | [View](torrust-tracker.md) |

## Build and Scan

```bash
# Build the production release image
docker build -t torrust-tracker:local -f Containerfile .

# Scan
trivy image --severity HIGH,CRITICAL torrust-tracker:local
```

See [`../README.md`](../README.md) for detailed scanning instructions.
