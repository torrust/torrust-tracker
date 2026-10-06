---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
last-updated-utc: "2026-10-06 16:00"
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

Superseded by V2 and V3 below.

## Environment for V2 and V3

- Linux, branch `2417-2411-verify-http-scrape-info-hash-limit` with the fix
  applied on top of the commit `docs(adrs): [#2417] cap scrape info hashes per
  protocol` (uncommitted at run time); dev-profile binaries.
- `rustc 1.101.0-nightly (282215592 2026-10-04)`.
- Isolated config `.tmp/2417-manual/tracker.toml` (git-ignored): SQLite at
  `.tmp/2417-manual/sqlite3.db`, public mode, persistent completed statistics
  enabled, UDP `127.0.0.1:48969`, HTTP `127.0.0.1:48070`, health API
  `127.0.0.1:48313`, `info` logging. Health check reported both trackers OK.

```sh
env -u TORRUST_TRACKER_CONFIG_TOML -u TORRUST_TRACKER_CONFIG_TOML_PATH \
  ./target/debug/torrust-tracker -c .tmp/2417-manual/tracker.toml
```

## V2: UDP Control (M2)

Status: `DONE`. Classification: **Confirmed**; UDP keeps the first 74.

The maintained client could not run this probe: `tracker_client udp scrape`
rejects a 75th argument because its CLI declares `num_args = 1..=74`
(`console/tracker-client/src/console/clients/udp/app.rs` and
`console/tracker-client/src/console/clients/unified/udp.rs`). The datagram was
sent with an inline Python snippet instead (no script file was created):

```sh
python3 - <<'EOF'
import socket, struct
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.settimeout(5); s.connect(("127.0.0.1", 48969))
s.send(struct.pack(">qii", 0x41727101980, 0, 1))
action, tx, connection_id = struct.unpack(">iiq", s.recv(2048))
hashes = b"".join(b"%020d" % i for i in range(1, 76))
sent = s.send(struct.pack(">qii", connection_id, 2, 2) + hashes)
data = s.recv(65535)
action, tx = struct.unpack(">ii", data[:8])
print(f"connect action={action}; scrape sent_bytes={sent} hashes_sent={(sent - 16) // 20}")
print(f"response action={action} bytes={len(data)} entries={(len(data) - 8) // 12}")
EOF
```

```text
connect action=2; scrape sent_bytes=1516 hashes_sent=75
response action=2 bytes=896 entries=74
```

The first line's `action` label reuses the scrape response variable; the
connect exchange succeeded, since a scrape response requires a valid connection
ID. 1516 bytes carried 75 hashes; the 896-byte response (8 + 74 x 12) has 74
entries. The server's 1496-byte receive buffer already drops the 75th hash, so
this probe cannot isolate the parser cap; the `handle_packet` unit test does
(mutating the cap to 75 fails that test while the socket test still passes).

## V3: HTTP Recheck After the Fix (T6)

Status: `DONE`. Classification: **Fixed**; HTTP keeps the first 100.

Same compact ASCII hashes as V1, for 74, 75, and 1000:

```sh
set -o pipefail
for hash_count in 74 75 1000; do
    hashes=()
    for ((hash_index=1; hash_index<=hash_count; hash_index++)); do
        printf -v ascii_hash '%020d' "$hash_index"
        hashes+=("$(printf '%s' "$ascii_hash" | xxd -p)")
    done
    ./target/debug/tracker_client http scrape http://127.0.0.1:48070/scrape \
        "${hashes[@]}" | jq -c --argjson requested "$hash_count" \
        '{requested: $requested, returned: length, last: (keys | sort | last)}' || break
done
```

```text
{"requested":74,"returned":74,"last":"3030303030303030303030303030303030303734"}
{"requested":75,"returned":75,"last":"3030303030303030303030303030303030303735"}
{"requested":1000,"returned":100,"last":"3030303030303030303030303030303030313030"}
```

The last key of the 1000-hash response is `00000000000000000100`: the first 100
were kept. The tracker logged HTTP `200 OK` at
`2026-10-05T13:07:56.756693Z`, request ID
`e0adc46b-efed-421d-bd5b-8b20ff6b634c`. The owned tracker was stopped with
SIGINT and logged a successful shutdown at `2026-10-05T13:15:20.726927Z`.

## V4: Maintained Client Without the 74 Cap (T7)

Status: `DONE`. Classification: **Confirmed**; the client sends what it is
given and reports truncation.

Same isolated config and a fresh database, after removing the client's
`num_args = 1..=74`. Stdout and stderr were captured separately, following the
global CLI output contract ADR.

```sh
# UDP: 75 distinct ASCII hashes, with the datagram sizes traced
hashes=()
for ((i=1; i<=75; i++)); do
    printf -v a '%020d' "$i"
    hashes+=("$(printf '%s' "$a" | xxd -p)")
done
strace -f -e trace=sendto,sendmsg -o .tmp/2417-manual/udp-client-k.strace \
  ./target/debug/tracker_client udp scrape 127.0.0.1:48969 "${hashes[@]}" \
  > .tmp/2417-manual/udp-k.stdout 2> .tmp/2417-manual/udp-k.stderr
jq -c '{returned: (.Scrape.torrent_stats | length)}' .tmp/2417-manual/udp-k.stdout

# HTTP: 1000 distinct ASCII hashes, then the first 74, each to its own files
hashes=()
for ((i=1; i<=1000; i++)); do
    printf -v a '%020d' "$i"
    hashes+=("$(printf '%s' "$a" | xxd -p)")
done
./target/debug/tracker_client http scrape http://127.0.0.1:48070/scrape "${hashes[@]}" \
  > .tmp/2417-manual/http-k.stdout 2> .tmp/2417-manual/http-k.stderr
./target/debug/tracker_client http scrape http://127.0.0.1:48070/scrape "${hashes[@]:0:74}" \
  > .tmp/2417-manual/http-74.stdout 2> .tmp/2417-manual/http-74.stderr
jq -c '{returned: length}' .tmp/2417-manual/http-k.stdout .tmp/2417-manual/http-74.stdout
```

| Probe | Exit | Datagrams sent | Stdout entries | Stderr |
| --- | --- | --- | --- | --- |
| UDP, 75 hashes | 0 | 16 bytes (connect), 1516 bytes (75 hashes) | 74 (`.Scrape.torrent_stats`) | one warning record |
| HTTP, 1000 hashes | 0 | - | 100 | one warning record |
| HTTP, 74 hashes | 0 | - | 74 | empty |

```ndjson
{"warning":{"kind":"scrape_response_truncated","requested":75,"returned":74,"message":"requested 75 info hashes, tracker returned 74; the tracker may truncate scrapes"}}
{"warning":{"kind":"scrape_response_truncated","requested":1000,"returned":100,"message":"requested 1000 info hashes, tracker returned 100; the tracker may truncate scrapes"}}
```

M2 is now reproducible with the maintained client. The owned tracker was stopped
with SIGINT and logged a successful shutdown at `2026-10-05T15:11:38.559047Z`.
