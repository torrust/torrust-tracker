# aquatic-peer-id (internal fork)

This is a **temporary internal fork** of [`aquatic_peer_id`](https://crates.io/crates/aquatic_peer_id)
version 0.9.0, copied verbatim under its original [Apache 2.0 license](LICENSE).

## Why does this fork exist?

The Torrust Tracker workspace is replacing its dependency on the external `aquatic_udp_protocol`
crate with an in-house implementation (see [issue #1732](https://github.com/torrust/torrust-tracker/issues/1732)).
This package is an intermediate step: it pins the exact 0.9.0 source so we can migrate
gradually without breaking the build.

## Original author

Joakim Frostegård ([@greatest-ape](https://github.com/greatest-ape))

## License

Apache 2.0 — see [LICENSE](LICENSE).
