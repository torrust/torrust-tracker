---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md
last-updated-utc: "2026-11-15 21:07"
---

# Manual Verification Evidence

## Purpose

Record real verification of the slice S1 fix (finding F2, `PeerRole` JSON casing, V1-V3, 2026-11-11) and the
tracker-wide scenarios M1 and M2 of the issue (S4, 2026-11-15). Nothing here is invented: every output below was copied
from the runs listed.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-11, runs between 14:30:15 and 14:30:21.
- Artifact under test: the patches `test(primitives): [#2824] pin the lowercase JSON form of
  PeerRole` and `fix(primitives): [#2824] serialize PeerRole in lowercase`, and the follow-up
  `test(primitives): [#2824] compare the PeerRole JSON with its Display form` (patches named by
  subject; each run used a detached checkout of the commit with that subject).
- Operating system / environment: Linux 7.0.0-43-generic; `rustc 1.99.1 (58cf8f9c3 2026-10-13)`,
  `cargo 1.99.1 (5f94df478 2026-08-27)` (stable toolchain).
- Prerequisites: the runs shared one cargo target directory; the red and green runs used a
  temporary detached `git worktree` that was removed afterwards.

## Verification Processes

### V1 - F2 reproduction at the nearest seam

- Goal: show that `PeerRole` serialized as `"Seeder"` and `"Leecher"` while `Display` and the
  `peer_role` metric label (`swarm-coordination-registry` `statistics/event/handler.rs`) use
  `seeder` and `leecher`.
- Classification: nothing in production serializes a `PeerRole` (no struct in the workspace holds
  one; `grep` over `packages` and `src` finds `PeerRole` only in `peer.rs` and that handler), so the
  reproduction is the nearest-seam unit test on `serde_json::to_value`. A throwaway test on
  2026-11-10 (not kept) gave the same `"Seeder"` result.
- Status: `DONE`

#### Steps Performed

1. `git checkout --detach` the commit with subject `test(primitives): [#2824] pin the lowercase
   JSON form of PeerRole` (the old attribute `#[serde(rename_all_fields = "lowercase")]` is still
   present), then `cargo test -p torrust-tracker-primitives --lib peer_role_serialization`
   (started 14:30:15.347, ended 14:30:15.814).

#### Observed Result

```text
== red: test(primitives): [#2824] pin the lowercase JSON form of PeerRole

running 2 tests
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... FAILED
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... FAILED
...
  left: String("Seeder")
 right: String("seeder")
...
  left: String("Leecher")
 right: String("leecher")
...
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.00s
```

#### Conclusion

Both tests fail on the test commit for the reason F2 describes, so they reproduce the defect.

### V2 - The fix turns both tests green

- Goal: show the one-attribute change fixes the reproduction without breaking other unit tests.
- Status: `DONE`

#### Steps Performed

1. `git checkout --detach` the commit with subject `fix(primitives): [#2824] serialize PeerRole in
   lowercase`, then `cargo test -p torrust-tracker-primitives --lib` (started 14:30:20.151, ended
   14:30:20.435).

#### Observed Result

```text
== green: fix(primitives): [#2824] serialize PeerRole in lowercase
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... ok
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... ok
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

Both regression tests pass and all 49 unit tests pass (47 before the slice, plus the 2 new ones).

### V3 - Recheck at the branch head

- Goal: confirm the result still holds after the review fix that also compares the JSON value
  with the `Display` string.
- Status: `DONE`

#### Steps Performed

1. On the branch head, the commit with subject `test(primitives): [#2824] compare the PeerRole JSON
   with its Display form`, run `cargo test -p torrust-tracker-primitives` (ended 14:30:20.814).

#### Observed Result

```text
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... ok
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... ok
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

#### Conclusion

The unit tests (49) and the doctests (2 + 1) pass at the head.

### M1 - Default announce interval (S4)

- Goal: a tracker started with no announce policy section answers announces with the documented default interval
  (120 s), over HTTP and UDP.
- Environment: 2026-11-15, branch `2824-1347-add-tests-primitives-final` at the commit
  `test(primitives): [#2824] record the no-change files and compare instance id JSON by value`; Linux
  7.0.0-43-generic, stable `rustc 1.99.1`; debug builds of `torrust-tracker` and `tracker_client`.
- Configuration: `.tmp/manual-2824/tracker.toml` (git-ignored), `schema_version = "3.0.0"`, SQLite in `.tmp/`, one
  UDP tracker on `127.0.0.1:16979`, one HTTP tracker on `127.0.0.1:17079`, the REST API on `127.0.0.1:11219`. It has
  no `[core.announce_policy]` section, so the defaults apply.
- Status: `DONE`

#### Steps Performed (M1)

1. 07:54:01 - `./target/debug/torrust-tracker --config-toml-path ./.tmp/manual-2824/tracker.toml` (started in the
   background; the HTTP and API ports were listening, `ss -tlnp`).
2. 07:54:20.672 - `tracker_client http announce http://127.0.0.1:17079 1111111111111111111111111111111111111111
   --left 100 --port 6001 '--peer-id=-qB00000000000000011' --compact 0`
3. 07:54:20.679 - `tracker_client udp announce 127.0.0.1:16979 1111111111111111111111111111111111111111`

#### Observed Result (M1)

```text
== M1 http 07:54:20.672
{"complete":0,"incomplete":1,"interval":120,"min interval":120,"peers":[]}
== M1 udp 07:54:20.679
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":1,"seeders":1,"peers":["127.0.0.1:6001"]}}
```

#### Conclusion (M1)

Both protocols carry the 120 s default (`interval` and `min interval` over HTTP, `announce_interval` over UDP). The
peer cap (74) is not a response field; it is protected by the unit test of the default in S3.

### M2 - Seeder and leecher counts (S4)

- Goal: one seeder and one leecher announced with `tracker_client` give matching complete/incomplete counts in both
  scrapes and in the REST API torrent endpoint, with peer JSON that matches the announces.
- Environment and configuration: the same tracker process as M1, on a different info hash.
- Status: `DONE`

#### Steps Performed (M2)

1. 07:54:20.684 - a first attempt passed `--ip 10.0.0.1` and `--ip 10.0.0.2`; the tracker answered
   `{"failure reason":"Client-supplied peer IPs are disabled"}` (the configuration does not enable them), so nothing
   was stored (the scrape at 07:54:20.698 read 0 / 0 and the API said `"torrent not known"`). Repeated without `--ip`:
2. 07:54:25.719 - seeder: `tracker_client http announce http://127.0.0.1:17079
   9c38422213e30bff212b30c360d26f9a02136422 --event completed --left 0 --port 6881
   '--peer-id=-qB00000000000000001' --compact 0`
3. 07:54:25.726 - leecher: the same with `--event started --left 1000 --port 6882 '--peer-id=-qB00000000000000002'`
4. 07:54:25.733 - `tracker_client http scrape http://127.0.0.1:17079 9c38422213e30bff212b30c360d26f9a02136422`
5. 07:54:25.739 - `tracker_client udp scrape 127.0.0.1:16979 9c38422213e30bff212b30c360d26f9a02136422`
6. 07:54:25.744 - `curl -s http://127.0.0.1:11219/api/v1/torrent/9c38422213e30bff212b30c360d26f9a02136422 -H
   "Authorization: Bearer MyAccessToken"`
7. 07:54:35 - the tracker was stopped by its PID (`kill`); its log has no `ERROR` line.

#### Observed Result (M2)

```text
== M2 seeder 07:54:25.719
{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}
== M2 leecher 07:54:25.726
{"complete":1,"incomplete":1,"interval":120,"min interval":120,"peers":[{"ip":"127.0.0.1","peer id":[45,113,66,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,49],"port":6881}]}
== M2 http scrape 07:54:25.733
{"9c38422213e30bff212b30c360d26f9a02136422":{"complete":1,"downloaded":0,"incomplete":1}}
== M2 udp scrape 07:54:25.739
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":1,"completed":0,"leechers":1}]}}
== M2 rest 07:54:25.744
{"info_hash":"9c38422213e30bff212b30c360d26f9a02136422","seeders":1,"completed":0,"leechers":1,"peers":[{"peer_id":{"id":"0x2d71423030303030303030303030303030303031","client":null},"peer_addr":"127.0.0.1:6881","updated":1794729265724,"updated_milliseconds_ago":1794729265724,"updated_at_ms":1794729265724,"uploaded":0,"downloaded":0,"left":0,"event":"Completed"},{"peer_id":{"id":"0x2d71423030303030303030303030303030303032","client":null},"peer_addr":"127.0.0.1:6882","updated":1794729265730,"updated_milliseconds_ago":1794729265730,"updated_at_ms":1794729265730,"uploaded":0,"downloaded":0,"left":1000,"event":"Started"}]}
```

#### Conclusion (M2)

One seeder and one leecher everywhere: `complete` 1 / `incomplete` 1 over HTTP, `seeders` 1 / `leechers` 1 over UDP and
in the REST API. The two REST peers carry the announced peer ids, ports, `left` (0 and 1000) and events. Two
observations, neither a finding of this package: the download count stays 0 because the seeder's first announce was
already `completed` (it was never a leecher of this swarm), and the deprecated `updated` and
`updated_milliseconds_ago` REST fields equal `updated_at_ms`, as the REST torrent module documents. The `event` text
(`"Completed"`) is the derived `Debug` form of the domain `AnnounceEvent`: `rest-api-runtime-adapter` builds it with
`format!("{:?}", value.event)`, and no test there pins the text. Recorded as a follow-up candidate in `lessons.md`; the
REST contract is outside this issue.

## Failures and Follow-up

None. The M2 first attempt failed on a client option this configuration does not allow, not on the
tracker; it is recorded above. The EPIC "Latest" values and the slice coverage measurements are recorded in
`coverage-evidence.md` and the EPIC.
