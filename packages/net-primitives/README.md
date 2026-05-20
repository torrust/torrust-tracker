# Torrust Net Primitives

Generic networking primitive types for [Torrust](https://torrust.com/) projects.

This crate provides low-level networking types that are reusable across Torrust projects
without pulling in tracker-specific dependencies.

## Types

- `service_binding::ServiceBinding` — represents a network address binding (protocol + socket address).
- `service_binding::Protocol` — supported network protocols (`UDP`, `HTTP`, `HTTPS`).

## Documentation

[Crate documentation](https://docs.rs/torrust-net-primitives).

## License

The project is licensed under the terms of the [GNU AFFERO GENERAL PUBLIC LICENSE](./LICENSE).
