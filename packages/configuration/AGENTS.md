# torrust-tracker-configuration — AI Assistant Instructions

For full project context see the [root AGENTS.md](../../AGENTS.md).

## Package Purpose

Defines and loads all tracker configuration. Version `3.0.0` structs live under
`src/v3_0_0/`. Version `2.0.0` structs live under `src/v2_0_0/` and are kept for
backward compatibility.

---

## Rules Specific to This Package

### Rule: Use typed newtypes for domain-constrained configuration fields

**This is the most common mistake to avoid in this package.**

When adding a configuration field that has a domain constraint — a rule that makes
the valid value space smaller than the raw primitive — you **must** use a typed
newtype, not a raw primitive.

**Wrong**:

```rust
// ✗ Option<String> carries no invariant — consuming code must re-validate.
pub public_url: Option<String>,

// ✗ url::Url is parsed but the scheme is not constrained.
pub public_url: Option<url::Url>,
```

**Correct**:

```rust
// ✓ HttpUrl guarantees http:// or https:// at the type level.
pub public_url: Option<HttpUrl>,

// ✓ UdpUrl guarantees udp:// at the type level.
pub public_url: Option<UdpUrl>,
```

**Implementation checklist** when adding a new constrained field type:

1. Define the newtype in the appropriate module (scheme-constrained URL types live
   in `src/v3_0_0/public_url.rs`).
2. Implement `new(inner) -> Result<Self, String>` — validate the constraint.
3. Implement `parse(s: &str) -> Result<Self, String>` — parse then validate.
4. Implement `Serialize` — delegate to the inner value's string form.
5. Implement `Deserialize` — call `Self::parse` and map errors to `de::Error::custom`.
6. Implement `Display`, `AsRef<str>` (and `AsRef<InnerType>` if useful) for
   ergonomic access in consuming code.
7. Write tests: accept valid value, reject invalid value, round-trip through TOML.
8. Use `#[serde(default)]` on the struct field — **no** `deserialize_with` attribute
   is needed because the type's `Deserialize` impl handles validation.

**Granularity rule**: Use the narrowest type that captures the _actual_ constraint.
Do **not** create a service-specific subtype (e.g. `HttpTrackerUrl`) unless the
service protocol imposes a constraint on the URL itself beyond the scheme
(e.g. a mandatory path required by a BitTorrent Enhancement Proposal).

Full rationale:
[ADR 20260721100000](../../docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md)

---

### Rule: Deny unknown fields in all v3 config structs

Every `v3_0_0` configuration struct must carry `#[serde(deny_unknown_fields)]`.
This rejects typos and stale keys at deserialization time instead of silently
ignoring them.

---

### Rule: Field defaults via associated functions, not `Default::default()`

Each struct field that has a non-obvious default must be wired through a private
associated function used as the `#[serde(default = "...")]` target:

```rust
#[serde(default = "HttpTracker::default_bind_address")]
pub bind_address: SocketAddr,

fn default_bind_address() -> SocketAddr { ... }
```

This makes the default value explicit and independently testable.
