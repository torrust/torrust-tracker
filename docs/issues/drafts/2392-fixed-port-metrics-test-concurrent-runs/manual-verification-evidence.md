---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md
last-updated-utc: "2026-11-16 22:23"
---

# Manual Verification Evidence

## Purpose

Record the real-artifact reproduction made while drafting this issue (V1). The recheck (V2) is
added when the issue is implemented. This is evidence from commands actually performed; nothing
here is simulated.

## Preserving Verification Artifacts

The runs wrote their output to a `mktemp -d` directory that was removed afterwards. Everything a
reviewer needs (commands, exit statuses, failure lines) is recorded inline below.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16 22:03:41 to 22:04:10 (a first, unrecorded run on 2026-11-16 at
  17:30 against `b009b6425` gave the same 20 of 20 result)
- Artifact under test: `develop` at `0373f05f8` merged into the #2394 audit branch; the branch adds
  documentation only, so the test code is `develop`'s
- Operating system / environment: Linux 7.0.0-43-generic, stable Rust toolchain
  `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`
- Prerequisites and setup performed: nothing else on the machine listening on ports
  `17091`-`17094`; the test binary built with
  `cargo +stable test --test metrics-fixed-ports --no-run`, which printed
  `Executable tests/metrics/fixed_ports.rs (target/debug/deps/metrics_fixed_ports-8f5d0020928c0968)`

## Verification Processes

### V1 - Reproduce the collision

- Goal: show that a second run of `metrics-fixed-ports` on the same machine fails while the first
  holds its fixed ports.
- Initial state: the test binary built; ports `17091`-`17094` free.
- Status: `DONE`

#### Steps Performed

1. V1a, port held by another process. A Python listener bound `0.0.0.0:17091` and stayed open
   while the binary ran once:

   <!-- cspell:ignore returncode -->

   ```bash
   python3 - target/debug/deps/metrics_fixed_ports-8f5d0020928c0968 <<'PY'
   import socket, subprocess, sys
   s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
   s.bind(("0.0.0.0", 17091)); s.listen()
   r = subprocess.run([sys.argv[1]], capture_output=True, text=True)
   print("exit status:", r.returncode)
   PY
   ```

2. V1b, two runs at the same time. The binary was started twice in the background and both were
   awaited, 20 times in a row (each pair finished before the next started):

   ```bash
   bin=target/debug/deps/metrics_fixed_ports-8f5d0020928c0968
   d=$(mktemp -d)
   for i in $(seq 1 20); do
     "$bin" >"$d/a$i" 2>&1 & pa=$!
     "$bin" >"$d/b$i" 2>&1 & pb=$!
     wait $pa; ea=$?; wait $pb; eb=$?
     echo "pair $i: exit a=$ea b=$eb"
   done
   grep -L 'test result: ok' "$d"/* | wc -l   # failed runs
   grep -l AddrInUse "$d"/* | wc -l          # runs with AddrInUse
   ```

#### Observed Result

V1a: exit status `101`.

```text
tracker application should start: ServiceStartup { service: "HTTP tracker", source: Listener { source: Bind { source: Os { code: 98, kind: AddrInUse, message: "Address already in use" } } } }
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

V1b: in every pair exactly one run exited `101` and the other `0`. Run a failed in pairs 4-7, 9-11,
15, 16, 19 and 20 (11 pairs), run b in the other 9. 40 runs, 20 failed, 20 with `AddrInUse`. Which
listener lost the race varied, for example:

```text
tracker application should start: ServiceStartup { service: "UDP tracker", source: Listener { source: Bind { source: Os { code: 98, kind: AddrInUse, message: "Address already in use" } } } }
```

#### Conclusion

Reproduced, as the issue's V1 expects: two concurrent runs on one machine always collide.

## Failures and Follow-up

None. V2 (the recheck after the fix) is recorded when the issue is implemented.
