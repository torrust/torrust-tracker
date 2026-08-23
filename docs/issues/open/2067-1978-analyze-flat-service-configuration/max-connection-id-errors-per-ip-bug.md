# Confirmed Bug: UDP Connection-ID Error Limit Is Mis-scoped

> **Status:** Confirmed during issue #2067 analysis; no fix is included here.
>
> **Parent analysis:** [ISSUE.md](ISSUE.md)
>
> **Decision record:** [analysis.md](analysis.md)

## Summary

`max_connection_id_errors_per_ip` is declared on every UDP listener configuration, implying that
each `[[udp_trackers]]` entry can control its own connection-ID error limit. The runtime does not
honor that meaning. It reads only the first configured UDP listener's value, then constructs one
shared `BanService` used by every UDP listener in the process.

This is a configuration-model bug: either a value is listener-specific and every listener must
receive an independent service configured with its own value, or it controls a shared service and
must be represented once as shared/global configuration. The current first-entry-wins behavior is
neither model and makes security behavior depend silently on configuration order.

## Reproduction

The following values imply two different listener policies:

```toml
[[udp_trackers]]
bind_address = "127.0.0.1:6969"
max_connection_id_errors_per_ip = 1

[[udp_trackers]]
bind_address = "127.0.0.1:6970"
max_connection_id_errors_per_ip = 100
```

`src/container.rs` selects the first entry's value:

```rust
let max_connection_id_errors = configuration
    .udp_trackers
    .as_ref()
    .and_then(|trackers| trackers.first())
    .map_or(default_max_connection_id_errors, |config| {
        config.max_connection_id_errors_per_ip
    });
```

It passes that one value to `UdpTrackerCoreServices::initialize_from`. That function creates one
`Arc<RwLock<BanService>>`, and each `UdpTrackerCoreContainer` receives a clone of the same arc.
Consequently both listeners use the limit `1`; the second listener's configured `100` is ignored.
Reordering the TOML entries changes the application-wide limit without changing the shared-service
design.

## Evidence

| Fact                                            | Source                                                                                                 |
| ----------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| Field is placed on each listener                | `packages/configuration/src/v2_0_0/udp_tracker.rs`, `packages/configuration/src/v3_0_0/udp_tracker.rs` |
| First UDP listener value is selected            | `src/container.rs::AppContainer::initialize`                                                           |
| One shared ban service is created               | `packages/udp-core/src/container.rs::UdpTrackerCoreServices::initialize_from`                          |
| All UDP containers clone that service           | `packages/udp-core/src/container.rs::UdpTrackerCoreContainer::initialize_from_services`                |
| Shared ban state is intentional security design | `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`                                 |

The ADR explicitly states that settings affecting shared services must themselves be global and
uses global `connection_id_validation` as its example. The same reasoning applies to the error
limit held by the shared `BanService`.

## Recommended Follow-up Scope

Create a separate bug sub-issue of EPIC #1978. Its preferred correction is:

1. Move `max_connection_id_errors_per_ip` from `UdpTracker` to the shared
   `UdpTrackerServer` configuration.
2. Remove the per-listener field from the final v3 schema, defaults, fixtures, documentation, and
   constructors.
3. Make `AppContainer` pass the one shared `udp_tracker_server` value to
   `UdpTrackerCoreServices::initialize_from`.
4. Add tests proving that multiple UDP listeners use the same declared global limit and that
   configuration order cannot change it.
5. Update the v2-to-v3 migration guidance because the field moves from each listener to the shared
   section.

Do not implement this bug fix as part of #2067. The next step is to draft and review a dedicated
sub-issue specification before creating its GitHub issue.

## Rejected Interim Option

Validating that every listener repeats the same value would prevent inconsistent input but would
still duplicate one global policy in every listener block. It is an inferior schema because it
retains ambiguity and raises maintenance cost. The field should be represented once where the
shared service is configured.
