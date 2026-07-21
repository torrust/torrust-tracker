# Docker Image Scan Results

Historical security scan results for the Torrust Tracker Docker image.

## Current Status Summary

| Image             | Stage                 | MEDIUM  | HIGH  | CRITICAL | Exposure   | Last Scan    | Details                    |
| ----------------- | --------------------- | ------- | ----- | -------- | ---------- | ------------ | -------------------------- |
| `torrust-tracker` | release               | 5       | 0     | 0        | Production | Jul 20, 2026 | [View](torrust-tracker.md) |
| Build stages      | `chef`/`tester`/`gcc` | 299-309 | 51-65 | 4        | Build only | Jul 20, 2026 | [View](build-images.md)    |

## Build and Scan

```bash
# Build the production release image
docker build -t torrust-tracker:local -f Containerfile .

# Scan
trivy image --severity HIGH,CRITICAL torrust-tracker:local
```

Build stages are scanned after base-image or package changes and during quarterly review.
Their consolidated history is kept separate from production findings because they are
ephemeral, unpublished images.

See [`../README.md`](../README.md) for detailed scanning instructions.
