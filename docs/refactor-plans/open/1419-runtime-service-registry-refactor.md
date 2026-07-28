---
doc-type: refactor-plan
status: open
related-issue: 1419
spec-path: docs/refactor-plans/open/1419-runtime-service-registry-refactor.md
last-updated-utc: 2026-07-28 11:54
semantic-links:
  skill-links:
    - create-refactor-plan
  related-artifacts:
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan - Runtime Service Registry for #1419

## Goal

Make runtime service discovery a reliable application capability by evolving `Registar` from a
health-check registration mechanism into the authoritative registry of started local services.
This removes the main-application integration tests' bind-IP heuristic and makes port-zero listener
configuration safe for all local services.

Related artifact: [issue #1419](../../issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)

## Constraints

- Preserve `ServiceBinding` as the local protocol-plus-socket-address representation; do not model
  public or proxy-facing URLs in this work.
- Keep tracker service roles out of `torrust-net-primitives` and `torrust-server-lib`.
- Use the existing `ServiceRegistrationForm` reporting path; do not add a parallel registry.
- Keep `JobManager` responsible for lifecycle only.
- Preserve the health API response's `service_binding`, `binding`, and `service_type` fields.
- Complete the standalone `torrust-server-lib` release before updating the tracker dependency.

## Items

### 1. [x] Record the runtime registry architectural boundary [High impact / Low effort]

**Problem**: `Registar` currently appears health-check-specific, leaving its relationship to
`AppContainer`, `JobManager`, and integration-test endpoint discovery implicit.

**Files**:

- `docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md`
- `docs/adrs/index.md`

**Change**: Record `Registar` as the authoritative local runtime service registry, its metadata
scope, ownership boundaries, and rejected alternatives.

---

### 2. [x] Add tracker-owned service role values [High impact / Low effort]

**Problem**: HTTP tracker, REST API, and UDP tracker each define a duplicated `TYPE_STRING`
constant. The generic registry needs a role name, but must not own tracker-specific variants.

**Files**:

- `packages/primitives/src/service_role.rs`
- `packages/primitives/src/lib.rs`
- `packages/axum-http-server/src/server.rs`
- `packages/axum-rest-api-server/src/server.rs`
- `packages/udp-server/src/server/launcher.rs`

**Change**: Add `ServiceRole` in `torrust-tracker-primitives`, with canonical role names exposed
through `as_str()`. Replace the three local constants with the appropriate enum variant. Decide and
document whether a non-self-checkable health-check API role is needed in the first migration.

---

### 3. [ ] Extend the standalone registry registration record [High impact / Medium effort]

**Problem**: `ServiceRegistration` stores only a binding and health-check function. Its consumers
cannot identify a service without executing network I/O through `spawn_check()`.

**Files**:

- `torrust-server-lib/src/registar.rs` (standalone repository)
- `torrust-server-lib` unit tests (standalone repository)

**Change**: Store the canonical tracker-supplied role name in `ServiceRegistration`, provide
read-only access to registration metadata, and expose a snapshot or role-query API from `Registar`
without exposing `HashMap` ordering as a contract. Separate immutable registration metadata from
per-health-check execution data. Release a new compatible `torrust-server-lib` version.

---

### 4. [ ] Migrate tracker registrations and health reporting [High impact / Medium effort]

**Problem**: Server packages construct role strings only inside health-check jobs, so the registry
and health API have duplicated, indirectly related metadata paths.

**Files**:

- `packages/axum-http-server/src/server.rs`
- `packages/axum-rest-api-server/src/server.rs`
- `packages/udp-server/src/server/states.rs`
- `packages/udp-server/src/server/launcher.rs`
- `packages/axum-health-check-api-server/src/handlers.rs`
- `packages/axum-health-check-api-server/src/resources.rs`
- root `Cargo.toml`
- `Cargo.lock`

**Change**: Upgrade to the released server-library version. Pass canonical role names through each
existing registration form. Build health reports from registration metadata plus health-check
execution results, keeping the external JSON field names and values stable. Add focused tests for
HTTP, HTTPS, REST API, and UDP registrations.

---

### 5. [ ] Make registration visibility an application readiness guarantee [High impact / Medium effort]

**Problem**: `tests/common/mod.rs` sleeps for 500 milliseconds after `app::run()` because
registration insertion happens asynchronously after a service reports through its form.

**Files**:

- `torrust-server-lib/src/registar.rs` (standalone repository)
- `src/app.rs`
- `tests/common/mod.rs`

**Change**: Make the registration protocol acknowledge insertion or otherwise provide a deterministic
readiness boundary. Remove the fixed test delay only after `app::run()` or the relevant bootstrap
step guarantees registrations are visible.

---

### 6. [ ] Replace IP-based endpoint discovery in main-application tests [High impact / Low effort]

**Problem**: The integration helpers classify services by wildcard versus loopback bind IP. This
breaks for valid configurations that use the same bind address for multiple HTTP services.

**Files**:

- `tests/common/mod.rs`
- `tests/stats.rs`
- `tests/scaffold.rs`
- `tests/servers/api/contract/stats/mod.rs`

**Change**: Query registrations by canonical role and use their final `ServiceBinding` values. Keep
only the client-side wildcard-to-loopback conversion needed to connect to a local listener. Configure
HTTP trackers, REST API, and health API with port zero without identity-by-address conventions.

---

### 7. [ ] Validate the cross-repository migration [High impact / Medium effort]

**Problem**: This change modifies a published dependency and every service-registration consumer.
Focused tests are needed to prove the new contract before broader workspace validation.

**Files**:

- `torrust-server-lib` test suite (standalone repository)
- `packages/axum-health-check-api-server/tests/`
- `tests/`

**Change**: Add or update unit and contract tests for registration metadata, role-based lookups,
health-report compatibility, and port-zero main-application suites. Run the focused tests after each
slice, then the repository quality gates and full required checks.

## Order of Execution

| Order | Status | Item                                                            | Impact | Effort |
| ----- | ------ | --------------------------------------------------------------- | ------ | ------ |
| 1     | [x]    | Record the runtime registry architectural boundary              | High   | Low    |
| 2     | [x]    | Add tracker-owned service role values                           | High   | Low    |
| 3     | [ ]    | Extend the standalone registry registration record              | High   | Medium |
| 4     | [ ]    | Migrate tracker registrations and health reporting              | High   | Medium |
| 5     | [ ]    | Make registration visibility an application readiness guarantee | High   | Medium |
| 6     | [ ]    | Replace IP-based endpoint discovery in main-application tests   | High   | Low    |
| 7     | [ ]    | Validate the cross-repository migration                         | High   | Medium |
