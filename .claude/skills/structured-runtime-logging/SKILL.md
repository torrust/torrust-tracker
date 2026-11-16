---
name: structured-runtime-logging
description: "Use when adding or changing logs for runtime service identity, service startup, listener bindings, or tracing instrumentation. Prefer explicit structured tracing fields over Rust Debug-formatted metadata."
metadata:
  author: torrust
  version: "1.0"
---

# Structured Runtime Logging

When logging runtime service identity, emit stable tracing fields instead of
recording `RuntimeServiceMetadata`, `ConfigurationInstanceId`, or related
structs through `Debug` formatting.

Use the canonical fields:

- `service_role` — the canonical role identifier, such as `http_tracker`.
- `instance_index` — the canonical zero-based configuration instance index.
- `service_binding` — the final protocol and bound socket address, after the
  listener has successfully bound.

## Correct Form

Exclude metadata from automatic `#[instrument]` capture and add canonical
fields explicitly:

```rust
#[instrument(
    skip(metadata),
    fields(
        service_role = metadata.service_role().as_str(),
        instance_index = metadata.configuration_instance_id().instance_index(),
    )
)]
```

When a listener binds, log its final `service_binding` as an explicit field.

```rust
tracing::info!(
  service_binding = %service_binding.url(),
  "Started HTTP tracker"
);
```

The resulting event has stable, queryable fields:

```text
INFO start_job{service_role="http_tracker" instance_index=1}: Started HTTP tracker service_binding=http://0.0.0.0:7171
```

## Incorrect Form

Do not let `#[instrument]` capture the metadata parameter automatically, and
do not log the metadata with `?` or `%` formatting:

```rust
#[instrument]
async fn start(metadata: RuntimeServiceMetadata) {
  tracing::info!(?metadata, "Started HTTP tracker");
}
```

This creates log output coupled to the Rust struct's `Debug` representation,
such as `metadata=RuntimeServiceMetadata { configuration_instance_id: ... }`.
It is not a stable, queryable log contract.

For example, automatic span capture and `?metadata` produce implementation
detail in the log instead of canonical fields:

```text
INFO start_job{idx=1 metadata=RuntimeServiceMetadata { configuration_instance_id: ConfigurationInstanceId { service_role: HttpTracker, instance_index: 1 } }}: Started HTTP tracker metadata=RuntimeServiceMetadata { configuration_instance_id: ConfigurationInstanceId { service_role: HttpTracker, instance_index: 1 } }
```

Do not make Rust field names, struct nesting, or a `Debug` implementation an
observability contract.
