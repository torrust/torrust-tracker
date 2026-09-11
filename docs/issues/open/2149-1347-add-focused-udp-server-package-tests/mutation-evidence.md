---
doc-type: mutation-evidence
issue: 2149
package: torrust-tracker-udp-server
status: completed
measured-utc: 2026-09-11
---

# UDP Server Mutation Evidence

This is a bounded mutation-testing assessment for Issue #2149. It samples the direct
source-port-zero guard in `packages/udp-server/src/server/processor.rs`, a changed
high-risk transport boundary. It is not a package score, a coverage target, or a CI gate.

## Configuration

```text
cargo mutants \
  --package torrust-tracker-udp-server \
  --all-features \
  --re 'Processor::process_request' \
  --timeout 300 \
  --no-times \
  --output .tmp/udp-server-processor-mutants.out \
  -- --lib server::processor::tests
```

The tool generated two mutants for `Processor::process_request`. The baseline ran the focused
processor unit-test module before mutated executions. The overall command timeout was 300 seconds;
the local ignored output directory retains tool logs only for this working session.

## Results

| Mutation | Outcome | Interpretation |
| --- | --- | --- |
| Replace `client_socket_addr.port() == 0` with `!= 0` | Caught | The focused port-zero response-suppression, discard-event, and handler-bypass tests reject an inverted guard. |
| Replace `Processor::process_request` body with `()` | Unviable | The generated mutation cannot satisfy the method's async control flow/type requirements; it is not a test-suite survivor. |

There were no surviving viable mutants. No follow-up test or production change is selected.

## Limitations And Decision

The sample intentionally excludes normal packet dispatch, response serialization, socket-send
failure, event-consumer behavior, launcher admission, and lifecycle paths. Those have their own
owners and test boundaries; expanding this run would turn a focused review signal into a slow,
tool-specific backlog. Mutation testing is recorded as supplemental evidence only and does not
replace focused behavior assertions, clean coverage scopes, integration contracts, or the normal
quality gate.
