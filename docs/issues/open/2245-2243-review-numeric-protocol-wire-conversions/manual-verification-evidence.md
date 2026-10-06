---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
last-updated-utc: "2026-10-06 14:20"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-06 10:19 (defect reproduction) and 11:45-11:48 (fix verification).
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`. The
  reproduction used `develop` before any #2245 change. The verification used branch
  `2245-2243-review-numeric-protocol-wire-conversions` at commit
  `fix(udp-server): [#2245] clamp announce wire fields instead of wrapping negative`.
- Operating system / environment: Linux; repository root as current working directory.
- Setup: the configuration below, saved as `.tmp/m1-2245/tracker-over.toml`. The
  `.tmp/m1-2245/tracker-max.toml` variant is identical except for `interval = 2147483647`.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
listed = false
private = false
tracker_usage_statistics = false

[core.announce_policy]
interval = 2147483648
interval_min = 120
max_peers_per_announce = 2

[core.database]
driver = "sqlite3"
path = "./.tmp/m1-2245/sqlite3.db"

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16970"
```

## Verification Processes

### V1 - M1: Announce Interval Bounds

- Goal: Confirm that the UDP announce response encodes `announce_interval` as a valid signed
  32-bit value at and beyond `i32::MAX` (A171).
- Initial state: a fresh tracker for each configuration; one announce on info-hash
  `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker with one configuration, for example:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/m1-2245/tracker-over.toml ./target/debug/torrust-tracker \
     > .tmp/m1-2245/tracker-over.log 2>&1
   ```

2. Announced once:

   ```sh
   ./target/debug/tracker_client udp announce 127.0.0.1:16970 9c38422213e30bff212b30c360d26f9a02136422 \
     --peer-id=-qB00000000000000001 --port 7001 --event started --left 100
   ```

3. Stopped the tracker, then repeated steps 1-2 for the other configuration and build.

#### Observed Result

Before the fix, on `develop`, with `interval = 2147483648` (the tracker logged
`"interval": 2147483648` at startup):

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":-2147483648,"leechers":1,"seeders":0,"peers":[]}}
```

After the fix, with `interval = 2147483647`:

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

After the fix, with `interval = 2147483648`:

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

| `interval` | Before the fix | After the fix | Expected |
| ---------- | -------------- | ------------- | -------- |
| 2147483647 (`i32::MAX`) | not run | 2147483647 | Unchanged |
| 2147483648 (`i32::MAX + 1`) | -2147483648 | 2147483647 | Clamped to `i32::MAX` |

#### Conclusion

The fix clamps an out-of-range interval to `i32::MAX` instead of wrapping it to a negative value,
and leaves in-range values unchanged. M1 passes. The tracker still accepts the out-of-range
configuration; rejecting it at load is the follow-up recorded in the issue spec.

## Regression-Test Boundary

The maintained regression test is
`it_should_clamp_an_out_of_range_interval_and_peer_counts_for_both_address_families` in
`packages/udp-server/src/handlers/announce.rs`, with the helper's tests in
`handlers::tests::wire_i32_conversion` in `packages/udp-server/src/handlers/mod.rs`. The issue
spec's Regression Test Strategy explains why `build_response` is the smallest deterministic seam
that covers all six call sites.

Both tests were added after the fix, so each red run follows the `fix-bug` skill's mutate-then-restore
procedure. Each mutation lived only in the working tree: the file was copied to `.tmp/` first,
mutated, tested, and restored from the copy. Nothing was staged.

### Red Run: Call Sites Back to the Original Cast

Run on 2026-10-06 at 13:18 UTC, on top of commit
`refactor(udp-server): [#2245] share one BEP 15 i32 clamp between announce and scrape`, with the new
test written but not yet committed. Each of the six `saturating_wire_i32(x)` calls in
`build_response` was replaced with the original `I32::new(i64::from(x) as i32)`:

```sh
cargo test -q -p torrust-tracker-udp-server --lib it_should_clamp_an_out_of_range
```

```text
handlers::announce::tests::announce_request::it_should_clamp_an_out_of_range_interval_and_peer_counts_for_both_address_families --- FAILED
assertion `left == right` failed
  left: AnnounceIpv4(AnnounceResponse { fixed: AnnounceResponseFixedData { transaction_id: TransactionId(I32(0)), announce_interval: AnnounceInterval(I32(-2147483648)), leechers: NumberOfPeers(I32(-2147483648)), seeders: NumberOfPeers(I32(-1)) }, peers: [] })
 right: AnnounceIpv4(AnnounceResponse { fixed: AnnounceResponseFixedData { transaction_id: TransactionId(I32(0)), announce_interval: AnnounceInterval(I32(2147483647)), leechers: NumberOfPeers(I32(2147483647)), seeders: NumberOfPeers(I32(2147483647)) }, peers: [] })
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 212 filtered out; finished in 0.00s
```

### Red Run: Wrapping Helper

Run on 2026-10-06 at 14:15 UTC, at commit
`test(udp-server): [#2245] pin the announce wire clamp at every build_response call site`. The
helper's body was replaced with `I32::new(value as i32)`:

```sh
cargo test -q -p torrust-tracker-udp-server --lib wire_i32_conversion
```

```text
handlers::tests::wire_i32_conversion::it_should_clamp_instead_of_wrapping_a_value_beyond_the_signed_wire_field --- FAILED
assertion `left == right` failed
  left: -2147483648
 right: 2147483647
test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 210 filtered out; finished in 0.00s
```

### Green Run

Run on 2026-10-06 at 13:18 UTC, after restoring the call sites and before committing the test. The
helper's red run at 14:15 UTC was restored from its copy, and the working tree was clean again.

```sh
cargo test -p torrust-tracker-udp-server --lib -- it_should_clamp_an_out_of_range wire_i32_conversion
```

```text
test handlers::tests::wire_i32_conversion::it_should_keep_a_value_that_fits_in_the_signed_wire_field ... ok
test handlers::tests::wire_i32_conversion::it_should_clamp_instead_of_wrapping_a_value_beyond_the_signed_wire_field ... ok
test handlers::announce::tests::announce_request::it_should_clamp_an_out_of_range_interval_and_peer_counts_for_both_address_families ... ok
test handlers::tests::wire_i32_conversion::it_should_keep_the_largest_value_that_fits_in_the_signed_wire_field ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 209 filtered out; finished in 0.00s
```

The like-for-like recheck against the real artifact is M1 above, repeated after the fix.
