## Debugging Tools

This directory contains developer-facing scripts for investigating problems that
are easier to isolate outside the normal test and CI flows.

These scripts are useful when you need to:

- reproduce a failure manually before changing Rust code
- inspect container logs, mounted files, and published ports
- validate assumptions about third-party tools such as qBittorrent
- confirm a fix in a smaller environment before running the full E2E runner

Subdirectories group scripts by topic. qBittorrent-specific helpers live in
`qbt/`.
