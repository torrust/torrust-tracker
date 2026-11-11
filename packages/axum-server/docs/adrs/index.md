---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/axum-server/docs/adrs/README.md
    - .github/skills/dev/planning/create-adr/SKILL.md
---

# Axum Server ADR Index

| ADR | Date | Title | Short Description |
| --- | --- | --- | --- |
| [20261111095222](20261111095222_close_idle_connections_without_a_408_response.md) | 2026-11-11 | Close idle connections without a 408 response | A connection that sends no complete first request is closed silently after the first-request timeout; no `408 Request Timeout` is sent. |
