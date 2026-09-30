---
doc-type: mutation-evidence
issue: 2283
package: torrust-tracker-udp-server
status: completed
measured-utc: 2026-09-30T06:21:00Z
---

# UDP Server Mutation Evidence

This is a bounded mutation-testing assessment for Issue #2283. It samples
`handle_packet`, the package's high-risk packet parsing and error-response routing seam. It is not
a package score, coverage target, or CI gate.

## Configuration

```text
cargo mutants \
  --package torrust-tracker-udp-server \
  --all-features \
  --re 'handle_packet' \
  --timeout 300 \
  --no-times \
  --output .tmp/2283-handler-packet-mutants.out \
  -- --lib handlers::tests
```

`cargo-mutants 27.0.0` generated three mutants. The focused baseline without mutations ran all three
`handlers::tests` tests. The timeout applies per mutant; ignored `.tmp` artifacts retain only
working-session logs.

## Results

| Mutation | Outcome | Interpretation |
| --- | --- | --- |
| Replace `handle_packet` with `(Default::default(), None)` | Unviable | `Response` has no `Default` implementation; removing the function body also makes imports unused. |
| Replace `handle_packet` with `(Default::default(), Some(Default::default()))` | Unviable | `Response` and `UdpRequestKind` have no `Default` implementations; removing the function body also makes imports unused. |
| Delete the `Error::InvalidRequest` arm that extracts the transaction ID | Caught | `it_should_preserve_the_transaction_id_for_a_sendable_parse_error_without_a_request_kind` fails because the generated error response no longer preserves the request transaction ID. |

There were no surviving viable mutants and no follow-up test or production change is selected.

## Scope And Limitations

The sample intentionally excludes successful connect, announce, and scrape handling; UDP socket
I/O; response serialization; event consumers; and lifecycle behavior. Those contracts have their
own focused boundaries, while lifecycle remains owned by #1488.

An initial probe of the newly added `connection_id_validation_policy` mapper generated only a
`Default::default()` replacement. It was unviable because that call is not permitted in the
function's `const` context, so it did not provide a viable assertion challenge and is not counted
as the T6 sample.
