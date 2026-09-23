# Retired-Script Captures

These files record what the four Bash and `jq` helpers retired by #2318 printed for
`../review-threads.json` at commit `ef234623`, the last `develop` commit that carried them.
They are the parity reference for the Rust binary, not its expected byte output: the binary
follows the CLI output contract (one JSON object on stdout, JSON diagnostics on stderr), so
`tests/cli.rs` compares data extracted from these captures with the binary's JSON.

| File | Retired script | Read by `tests/cli.rs` |
| ---- | -------------- | ---------------------- |
| `fetch.stdout`, `fetch.stderr` | `get-pr-review-threads.sh` | No; `fetch` needs `gh`, and the progress line on stderr was dropped |
| `list-unresolved.jsonl` | `list-unresolved-threads.sh` | Yes; each NDJSON line is one `threads` element |
| `show-unresolved.txt` | `show-unresolved-thread-bodies.sh` | Yes; thread IDs and comment URLs |
| `reply-status.stdout`, `reply-status.stderr`, `reply-status.exit-status` | `check-thread-reply-status.sh` | Yes; per-thread rows, summary counts, exit code |
