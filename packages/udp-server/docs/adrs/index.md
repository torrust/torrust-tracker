---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/udp-server/docs/adrs/README.md
    - .github/skills/dev/planning/create-adr/SKILL.md
---

# UDP Server ADR Index

| ADR | Date | Title | Short Description |
| --- | --- | --- | --- |
| [20260907152707](20260907152707_keep_oldest_first_udp_request_eviction.md) | 2026-09-07 | Keep oldest-first UDP request eviction | Preserve the bounded, oldest-first overload decision instead of scanning all request handles before evicting active work. |
