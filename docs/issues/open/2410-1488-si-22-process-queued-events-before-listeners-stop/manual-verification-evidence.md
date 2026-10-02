---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
last-updated-utc: "2026-10-02 15:38"
---

# Manual Verification Evidence

## Purpose

Record the reproduction of dropped events at shutdown and, after the fix, the
like-for-like recheck.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-01 15:13 (V0); 2026-10-01 20:48-20:59 (V1)
- Artifact under test: `develop` at `f6346f32`; tracker binary built with
  `cargo build --bin torrust-tracker` (`dev` profile: `opt-level = 1`)
- Operating system / environment: Linux; `rustc 1.101.0-nightly (21b707e3f 2026-09-30)`;
  SQLite 3 through the tracker's `sqlite3` driver; `curl`; `sqlite3` CLI
- Prerequisites and setup performed: a workspace build; V1 uses the disposable
  script `reproduce-lost-completions.sh` in this folder (see the spec's
  Disposable Verification Scripts). Its run directories are under
  `.tmp/si22-t0/` (git-ignored).

## Verification Processes

### V0 - Seam evidence: a ready event is dropped once cancellation is pending

- Goal: show the listener's cancel-first loop drops a ready event.
- Initial state: a scripted receiver holding one ready peer-download-completed
  event; a cancellation token already cancelled.
- Status: `DONE`
- Classification: seam-level evidence only. It observes the wrong outcome at
  the listener, but a public surface exists (the persisted `downloaded`
  count), so V1 is still required.

#### Steps Performed

1. `cargo test -p torrust-tracker-core --lib prioritize_a_pre_cancelled_token`

#### Observed Result

```text
test statistics::event::listener::tests::it_should_prioritize_a_pre_cancelled_token_over_a_ready_in_memory_event ... ok
test statistics::event::listener::tests::it_should_prioritize_a_pre_cancelled_token_over_a_ready_persistent_event ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 145 filtered out; finished in 0.01s
```

The persistent test asserts the downloads repository holds no count
(`expect_persisted_download_metrics_to_be(..., None)`) after a ready
completed-download event and a pending cancellation.

#### Conclusion

The listener drops a queued completed-download event instead of persisting
it when cancellation is pending.

### V1 - Real-artifact reproduction: lost completion at shutdown

- Goal: show that `completed` announces the tracker answered successfully
  are not persisted when the tracker binary stops on SIGTERM.
- Initial state: a fresh SQLite database; one HTTP tracker on
  `127.0.0.1:47070`; health-check API on `127.0.0.1:47313`;
  `persistent_torrent_completed_stat = true`; no UDP tracker or REST API. Each
  run uses one torrent (info-hash `SI22SI22SI22SI22SI22`) and N peers that
  first announce `started`, then `completed` (both with `left=0`). A
  completion is counted when a peer already in the swarm announces
  `completed` (`update_metadata_on_update`).
- Status: `DONE`
- Classification: **Reproduced.** The wrong outcome (answered completions
  missing from the database) was observed on the real binary, signalled
  through its own PID.

#### Steps Performed

1. `cargo build --bin torrust-tracker`
2. Control run, SIGTERM 1 s after the last response (nothing in flight):
   `PEERS=500 docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/reproduce-lost-completions.sh control`
3. Control run, SIGTERM 10 s after the last response:
   `PEERS=500 CONTROL_WAIT=10 .../reproduce-lost-completions.sh control`
4. Race runs, SIGTERM 0.5 s into the `completed` burst (run twice):
   `PEERS=2000 SIGTERM_DELAY=0.5 .../reproduce-lost-completions.sh race`
5. Restart on the first race run's database and scrape:
   `curl "http://127.0.0.1:47070/scrape?info_hash=SI22SI22SI22SI22SI22"`
6. Same, on a copy of that database, after one new `started` announce for
   the torrent.
7. Count persisted rows directly:
   `sqlite3 <db> "SELECT info_hash, completed FROM torrents; SELECT metric_name, value FROM torrent_aggregate_metrics;"`

#### Observed Result

| Run (UTC start)        | Mode    | SIGTERM                     | `completed` answered | Persisted (torrent / global) | Lost  | Exit status |
| ---------------------- | ------- | --------------------------- | -------------------- | ---------------------------- | ----- | ----------- |
| 20:48:13               | control | 1 s after the last response | 500                  | 467 / 467                    | 33    | 0           |
| 20:56:11               | control | 10 s after it               | 500                  | 500 / 500                    | 0     | 0           |
| 20:57:03               | race    | 0.5 s into the burst        | 856 (1,144 refused)  | 66 / 66                      | 790   | 0           |
| 20:57:05               | race    | 0.5 s into the burst        | 845 (1,155 refused)  | 70 / 70                      | 775   | 0           |

No run logged a database error (`Failed to increase ...`) or a lagged
receiver. Shutdown log of the 20:48:13 control run (colour codes removed):

```text
2026-10-01T20:48:14.722347Z INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...
2026-10-01T20:48:14.722438Z INFO TRACKER_CORE: Received cancellation request, shutting down tracker core event listener.
2026-10-01T20:48:14.722545Z INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=http_instance_0_127.0.0.1:47070
2026-10-01T20:48:14.724606Z INFO TRACKER_CORE: Received cancellation request, shutting down tracker core persistent completed statistics event listener.
2026-10-01T20:48:14.724640Z INFO torrust_tracker_lib::bootstrap::jobs::manager: Job completed after cooperative cancellation job=tracker_core_persistent_completed_statistics_event_listener
2026-10-01T20:48:14.724654Z INFO torrust_tracker: Torrust tracker successfully shutdown.
```

In the race runs, SIGTERM (20:57:05.212388) came about 1 ms after the last
answered announce (20:57:05.211), and no `/announce` response was logged after
the persistent listener stopped. Persisted rows of the 20:57:03 race run:

```text
5349323253493232534932325349323253493232|66
torrents_downloads_total|66
```

Scrape after a restart on that database, before and after one new announce
for the torrent:

```text
d5:filesd20:SI22SI22SI22SI22SI22d8:completei0e10:downloadedi0e10:incompletei0eeee
d5:filesd20:SI22SI22SI22SI22SI22d8:completei0e10:downloadedi66e10:incompletei1eeee
```

#### Conclusion

Reproduced; the control run isolates backlog loss, window (a). When SIGTERM arrives, the
persistent listener is still working through a backlog of completion events,
because it persists them more slowly than the HTTP tracker accepts them:
about 134 per second in the first race run (66 persisted in the 0.49 s
between the first `completed` announce and SIGTERM, while 856 were answered),
and about 370 per second in the first control run (467 in 1.26 s). These are
rough figures from a `dev` build with `info` logging on one machine. The
listener finishes the database write in progress (its cancellation is logged
about 2 ms after the other listeners'), then drops every queued event. With 10
s between the last response and SIGTERM the backlog had drained and nothing
was lost, which confirms the cause. The process still logs "successfully
shutdown" and exits with code 0, and nothing reports the loss.

Window (b) (events published by requests still draining after the listeners
stopped) was not isolated: in these runs every answered request completed
before the listeners were cancelled. It remains a hypothesis from the code.

After a restart, the client-visible symptom is a `downloaded` count that is
too low: 66 instead of 856 for this torrent. The persisted count only
appears once the torrent is announced again; before that, the scrape reports
`downloaded=0`.

## Failures and Follow-up

- The first control run already lost completions, although nothing was in
  flight. The 10-second control run confirmed the cause (a backlog dropped at
  cancellation). No scenario failed.
- Design follow-ups, recorded in the spec for maintainer decision (D14): the
  measured persistence rate makes a full-buffer drain (65,536 events) take
  minutes, far beyond the shared deadline; and persisted per-torrent counts
  are only visible after the torrent is announced again.

## Review Qualifications and Script Smoke Test

The race-run subtraction is a deficit, not an exact count of all lost events:
a request can publish an event even if the client fails to receive its response.
Likewise a failed response is not necessarily a refused connection. The original
table's "refused" labels mean failed client responses; no transport classification
was captured. The control with every response received is the clearer isolation
of window (a). No run isolates the exact contribution of window (b).

On 2026-10-02 the script gained mode/input checks, unique run directories,
owned-process cleanup, bounded readiness/shutdown observation, and validation
that every `started` announce succeeded. Its summary now labels the subtraction
`answered_minus_persisted`. Original V1 numbers above are unchanged historical
results from the earlier script, not reruns of the revised version.

Smoke command on the same `develop` baseline (`f6346f32`), at 13:55 UTC:

```sh
PEERS=3 CONCURRENCY=2 CONTROL_WAIT=0 HTTP_PORT=48070 HEALTH_PORT=48313 \
  docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/reproduce-lost-completions.sh control
```

```text
tracker_exit_status=0
started_ok=3
completed_ok=3 completed_failed=0
persisted_torrent_completed=3
persisted_global_downloads=3
answered_minus_persisted=0
```

`bash -n` and `shellcheck` passed. This verifies script mechanics for a small
normal run; it is not a post-fix recheck or a new throughput measurement.
Failure/interrupt cleanup has static checks but was not fault-injected here.
