---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md
last-updated-utc: 2026-10-05
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-05 15:00-15:04.
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`, built
  from branch `2246-2243-review-domain-numeric-conversions` at commit
  `docs(issues): [#2246] reconcile domain conversion outcomes in #2158 inventory` (all code changes
  applied).
- Operating system / environment: Linux; repository root as current working directory.
- Setup: an isolated configuration at `.tmp/m1-2246/tracker.toml` with a single UDP tracker on
  `127.0.0.1:16969`, SQLite under `.tmp/m1-2246/`, and
  `[core.announce_policy] max_peers_per_announce = 2` so the cap is observable with a small swarm.

## Verification Processes

### V1 - M1: Announce `numwant` Boundary

- Goal: Confirm that `PeersWanted::from_client_request` (A129) still resolves UDP `numwant`
  values end to end: non-positive means "as many as possible", positive values are capped at the
  tracker limit.
- Initial state: tracker running; four leechers (ports 7001-7004) announced on info-hash
  `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/m1-2246/tracker.toml ./target/debug/torrust-tracker \
     > .tmp/m1-2246/tracker.log 2>&1
   ```

2. Seeded four peers:

   ```sh
   for i in 1 2 3 4; do
     ./target/debug/tracker_client udp announce 127.0.0.1:16969 9c38422213e30bff212b30c360d26f9a02136422 \
       --peer-id=-qB0000000000000000$i --port 700$i --event started --left 100
   done
   ```

3. Announced from a fifth peer with each `numwant` value and counted the returned peers:

   ```sh
   for w in 0 -1 1 2 3 2147483647; do
     ./target/debug/tracker_client udp announce 127.0.0.1:16969 9c38422213e30bff212b30c360d26f9a02136422 \
       --peer-id=-qB00000000000000009 --port 7009 --left 100 --peers-wanted=$w \
       | jq -c '.AnnounceIpv4 | {peers: (.peers | length), leechers, seeders}'
   done
   ```

#### Observed Result

| `numwant` | Returned peers | Expected |
| --------- | -------------- | -------- |
| 0 | 2 | As many as possible, capped at 2 |
| -1 | 2 | As many as possible, capped at 2 |
| 1 | 1 | Exactly 1 |
| 2 | 2 | Exactly 2 |
| 3 | 2 | Capped at 2 |
| 2147483647 (`i32::MAX`) | 2 | Capped at 2 |

Every response reported `leechers: 5, seeders: 0`.

#### Conclusion

Positive values cap at the tracker limit; zero and negative values return as many peers as
possible. M1 passes.
