---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1417
    - issue #1978
    - packages/configuration/src/v3_0_0/public_url.rs
    - packages/configuration/src/v3_0_0/http_tracker.rs
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
---

# Use Newtypes for Domain-Constrained Configuration Field Types

## Description

Configuration struct fields that carry a domain constraint — a constraint
beyond "it is a string" or "it is a number" — must be represented as typed
newtypes rather than as `String`, `u32`, or any other primitive. The constraint
is encoded in the type; consuming code never re-validates it.

## Context

The `public_url` field added to `HttpTracker`, `UdpTracker`, and `HttpApi` in
issue #1417 provided a concrete test case. Three approaches were considered:

### Option A — `Option<String>` with a custom serde deserializer

```rust
#[serde(default, deserialize_with = "deserialize_optional_http_public_url")]
pub public_url: Option<String>,
```

Validation fires at deserialization but is then forgotten. After parsing the
config, consuming code holds a raw `String` with no type-level guarantee. It
must either trust the string or re-parse and re-validate it — both bad.

### Option B — `Option<url::Url>`

`url::Url` is already a parsed URL, so structural validity is guaranteed. But
the scheme constraint disappears: nothing in the type prevents a `udp://` URL
from sitting in `HttpTracker.public_url`. Additional runtime checks would still
be required in consumers.

### Option C — `Option<HttpUrl>` / `Option<UdpUrl>` newtypes ✓

```rust
pub public_url: Option<HttpUrl>,   // only http:// or https://
pub public_url: Option<UdpUrl>,    // only udp://
```

The scheme invariant is encoded in the type. The `Deserialize` impl validates
at the configuration boundary; after that the invariant is permanent and no
re-validation is needed anywhere.

## Agreement

**Use a typed newtype for every configuration field whose value space is smaller
than the raw primitive.**

Concretely:

1. The newtype wraps a validated inner value (e.g. `url::Url`, `IpAddr`).
2. The newtype implements `Serialize` / `Deserialize` directly, so no
   `#[serde(deserialize_with = ...)]` attribute is needed on the struct field.
3. Validation happens at deserialization time (the configuration boundary);
   code inside the application that receives the typed value can rely on the
   invariant without further checks.
4. The newtype exposes only the API that consuming code needs (e.g. `as_str()`,
   `as_url()`, `Display`) — it does not expose interior mutability that could
   bypass the invariant.

### Choosing the right granularity

Use the narrowest type that captures the _actual_ constraint without introducing
false specificity.

For URL scheme constraints:

| Situation                 | Type                          |
| ------------------------- | ----------------------------- |
| Must be `http` or `https` | `HttpUrl`                     |
| Must be `udp`             | `UdpUrl`                      |
| Must be `ws` or `wss`     | `WebSocketUrl` (hypothetical) |

Do **not** create a service-specific subtype (e.g. `HttpTrackerUrl`,
`UdpTrackerUrl`) unless the service protocol imposes a constraint _on the URL
itself_ beyond the scheme — for example a mandatory path prefix required by a
BEP specification. Scheme-level types are the correct granularity for general
validation.

### Compile-time vs runtime validation

URL _string content_ is runtime data (it comes from a configuration file), so
structural validation is necessarily runtime. However, the _kind_ guarantee
("`HttpUrl` is always http/https") lives in the type system, which means:

- The application never observes an invalid state.
- Callers that accept `HttpUrl` document their requirements at the type level,
  not with doc-comments or runtime panics.

## Alternatives Considered

### Keep `String` + custom serde helper

Rejected because the invariant evaporates after deserialization. Any code path
that receives the value must defensively re-validate.

### Use bare `url::Url`

Rejected because structural validity is not the only constraint. Scheme
constraints (and future constraints such as mandatory ports or allowed paths)
cannot be expressed in `url::Url` alone.

### Service-specific URL newtypes (`HttpTrackerUrl`, `UdpTrackerUrl`)

Rejected for the current case because there is no URL-format constraint specific
to tracker services (e.g. no mandatory `/announce` path required by BEP 3/15).
If a future service type does impose such a constraint, a service-specific newtype
becomes appropriate at that point.

## Consequences

- **Positive**: Domain constraints are visible in struct field types; no hidden
  serde attribute is needed.
- **Positive**: Consuming code receives a guarantee from the type system, not
  from documentation.
- **Positive**: Invalid configuration is rejected at the deserialization
  boundary with a descriptive error message; it can never propagate into the
  running application.
- **Negative**: Adding a new constrained field type requires writing a newtype
  with its own `Serialize`/`Deserialize` impl and tests instead of reusing a
  primitive.

## Date

2026-07-21
