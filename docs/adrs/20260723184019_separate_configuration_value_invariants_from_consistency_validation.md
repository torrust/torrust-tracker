---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - issue #1453
    - issue #1978
    - packages/configuration/src/validator.rs
    - packages/configuration/src/v3_0_0/types.rs
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
---

# Separate Configuration Value Invariants from Consistency Validation

## Description

Configuration validation has two different responsibilities that must not be
conflated.

A **value invariant** depends only on one value: for example, an IP-ban reset
interval must be no shorter than one hour. It must be rejected while the value
is constructed or deserialized, so invalid configuration cannot enter the
application.

A **configuration consistency rule** depends on a relationship between two or
more options: for example, a private-mode section is valid only when private
mode is enabled. It can only be assessed after the relevant configuration
sections have been assembled.

The existing `SemanticValidationError` and `Validator` names are broader than
their intended responsibility. Contributors have therefore added single-value
constraints to this cross-field validation layer.

## Agreement

Use these three layers for configuration validation:

| Layer                          | Use when                                                         | Mechanism                                             | Example                                        |
| ------------------------------ | ---------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------- |
| Value invariant                | One value has a constrained domain                               | Typed validated newtype, `TryFrom`, and `Deserialize` | A reset interval must be at least 3600 seconds |
| Configuration consistency      | A valid value combination depends on two or more options         | `Validator` and `SemanticValidationError`             | A private-mode section requires private mode   |
| Runtime/environment validation | Validity depends on the filesystem, network, or deployment state | Bootstrap/runtime check                               | A TLS certificate file is readable             |

For value invariants, use a typed newtype as established by
[the constrained configuration field types ADR](20260721100000_use_newtypes_for_constrained_configuration_field_types.md).
Use a reusable generic validated type when the invariant is a generally useful
shape, and wrap it in a domain-specific newtype at the configuration field
boundary when the domain needs tailored diagnostics or an intentional API.
For example:

```rust
pub struct IpBansResetIntervalInSecs(AtLeastU64<3_600>);
```

`Validator` is reserved for configuration consistency rules. It must not be
used merely because a value needs validation.

The naming debt remains visible at the module boundary:

```rust
// code-review: Rename `SemanticValidationError` and `Validator` to
// configuration-consistency names when a coordinated public API migration is scheduled.
```

Do not rename these public types as incidental work. A future coordinated API
migration should rename them to names such as `ConfigurationConsistencyError`
and `ConfigurationConsistencyValidator`.

## Alternatives Considered

### Add one-field rules to `Validator`

Rejected because a primitive field remains constructible in an invalid state,
and the validation step can be forgotten by callers. It also mixes two distinct
responsibilities in an already ambiguously named module.

### Use a field-local serde deserializer with a primitive `u64`

Rejected because direct Rust construction can still violate the invariant, and
the constrained domain is invisible in the configuration struct's API.

### Create only a dedicated interval newtype

Rejected because lower-bound numeric constraints are reusable. A generic
`AtLeastU64` establishes a small, tested pattern while the domain newtype retains
clear intent at the field boundary.

## Consequences

- **Positive**: Invalid single values are rejected during construction and
  deserialization, not after configuration assembly.
- **Positive**: Configuration field types expose their domain constraints.
- **Positive**: Cross-field validation has a narrow, documented responsibility.
- **Negative**: A constrained scalar needs a small type and serialization code
  instead of a primitive field.
- **Negative**: Existing validator names remain temporarily ambiguous until a
  coordinated public API migration is scheduled.

## Date

2026-07-23

## References

- [Issue #1453](https://github.com/torrust/torrust-tracker/issues/1453) — IP-ban reset interval configuration and duplicate cleanup task
- [Configuration Overhaul EPIC #1978](https://github.com/torrust/torrust-tracker/issues/1978)
- [Use Newtypes for Domain-Constrained Configuration Field Types](20260721100000_use_newtypes_for_constrained_configuration_field_types.md)
