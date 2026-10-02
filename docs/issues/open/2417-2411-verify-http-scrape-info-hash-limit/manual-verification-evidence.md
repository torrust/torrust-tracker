---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
last-updated-utc: "2026-10-02 15:38"
---

# HTTP Scrape Limit Verification

## Environment

- Linux, `develop` at `f6346f32`; existing dev-profile tracker and client binaries.
- `rustc 1.101.0-nightly (21b707e3f 2026-09-30)`; client reports version `0.1.0`.
- Local HTTP tracker `127.0.0.1:48070`, health API `127.0.0.1:48313`, SQLite,
  public mode, persistent completed statistics enabled, `info` logging.
- Reused the isolated configuration from SI-22's three-peer script smoke test:
  `.tmp/si22-t0/20261002T135541Z-control-eQbY3F/tracker.toml`. No external load.
- The client already decodes bencoded responses; `jq` counts its JSON entries.
  Requested hashes were distinct and absent from the database.

## V1: Documented 74-Hash Cap

Status: `DONE`. Classification: **Reproduced** documentation/behavior mismatch.
The local tracker returned 75 entries for 75 hashes, and 1000 for 1000.
This establishes no 74-hash count cap on these requests, not absence of every
possible transport or resource limit, nor a demonstrated denial of service.

Start command (waited for readiness before requests):

```sh
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH \
  ./target/debug/torrust-tracker \
  -c .tmp/si22-t0/20261002T135541Z-control-eQbY3F/tracker.toml
```

Initial boundary probes used distinct numeric 20-byte hashes:

```sh
set -o pipefail
for hash_count in 74 75 1000; do
    hashes=()
    for ((hash_index=1; hash_index<=hash_count; hash_index++)); do
        printf -v info_hash '%040x' "$hash_index"
        hashes+=("$info_hash")
    done
    ./target/debug/tracker_client http scrape http://127.0.0.1:48070/scrape \
        "${hashes[@]}" | jq -c --argjson requested "$hash_count" \
        '{requested: $requested, returned: length}' || break
done
```

```text
{"requested":74,"returned":74}
{"requested":75,"returned":75}
```

The 1000-hash numeric probe failed in the client with `Parsed Url is not a valid
Uri`: percent-encoding the mostly zero bytes created a much longer URL. This
was not a server rejection. The following compact ASCII-hash probe succeeded:

```sh
hashes=()
for ((hash_index=1; hash_index<=1000; hash_index++)); do
    printf -v ascii_hash '%020d' "$hash_index"
    info_hash=$(printf '%s' "$ascii_hash" | xxd -p)
    hashes+=("$info_hash")
done
./target/debug/tracker_client http scrape http://127.0.0.1:48070/scrape \
    "${hashes[@]}" | jq -c '{requested: 1000, returned: length}'
```

```text
{"requested":1000,"returned":1000}
```

The tracker logged HTTP `200 OK`, latency `2` ms, at
`2026-10-02T15:34:06.591905Z`, request ID
`67cafcfb-8c36-4821-aa05-353c08e397e3`. The request URI contained the ASCII hashes
`00000000000000000001` through `00000000000000001000`. URL length was not
separately captured. An initial one-hash probe mistakenly used `/announce` and
returned a missing-`peer_id` error; all results above use `/scrape`.

The owned tracker was stopped with SIGINT; its log reported cooperative job
completion and successful shutdown at `2026-10-02T15:35:02.176203Z`.

## Remaining Verification

- UDP control (M2) has not run; its parser limit remains source evidence only.
- Behavior choice remains a maintainer decision: truncate, reject, or document
  a different intentional HTTP contract. No runtime fix was made here.
- After that decision, prove the maintained regression test red if behavior
  changes, then green, and repeat these exact probes. Record results here.
- No new disposable script was created; the shell commands above are the full
  probe and use the maintained client for protocol parsing.
