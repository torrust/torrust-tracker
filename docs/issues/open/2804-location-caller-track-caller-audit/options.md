# Issue #2804 - T4: fix options and recommendation

Research record for task T4. It compares the four options of the spec against the inventory
(`inventory.md`, T1-T3) and recommends one. It changes no code. Facts below were read from the tree
at `f8b0f26e9` (`torrust/develop`) and from `torrust-located-error` 3.0.0 in the cargo registry.

## Facts the options depend on

1. **22 `Display` formats print a `{location}` field** (`grep '#[error(.*location'`): 4 in
   `announce.rs`, 2 in `scrape.rs`, 1 each in `peer_ip_resolver.rs`, `query.rs`, 2 in `auth.rs`, 3 in
   `primitives/src/peer.rs`, 1 in `tracker-core/src/error.rs`, 3 in `databases/error.rs`, 3 in
   `authentication/key/mod.rs`, 2 in `udp-server/src/error.rs`.
2. **`LocatedError` prints the location itself.** Its `Display` is
   `write!(f, "{}, {}", self.source, self.location)` (`torrust-located-error` 3.0.0, `lib.rs:58`).
   Every error wrapped by `Located(e).into()` or the class-C conversions shows a location whatever the
   wrapped enum does. Changing it needs a new release of that standalone crate.
3. **Most of those messages already carry the input**: `{param_name}`, `{param_value}`, `{raw_param}`,
   `{key}`, `{info_hash}`, `{driver}`. The location is the only part that can be wrong.
   `Key has expired`, `Missing authentication key` and `internal server error: {message}` have no input,
   but the message states the problem without the location.
4. **Tests do not depend on the location text.** `assert_bencoded_error`
   (`axum-http-server/tests/server/asserts.rs:10`) uses `contains` on the failure reason and prints the
   location only in its own failure message. The one test that compares a location string is
   `udp-server/src/event.rs:436` (it builds the expected value from a `Location` it captured).
5. **A field kept but not printed is not a semver break.** Removing `{location}` from a
   `#[error(..)]` string changes the text, not the type. Removing the field itself changes a public
   enum variant.

## Comparison

| Criterion | 1. `#[track_caller]` chain | 2. Capture at the call site | 3. Context instead of location | 4. Remove the field |
| --- | --- | --- | --- | --- |
| Correct for trait impls reached through `parse`/`try_into` (P1-P4, W) | No: points into `core/src/str/mod.rs` (D1, E6) | No: the constructor is still called from the impl | Yes | Yes |
| Correct for `async fn` (P11-P13, P15, P16) | No: attribute is a no-op (E8) | Only if the site is moved to a sync wrapper | Yes | Yes |
| Correct for closures (P5b, P14) | No (E3, E4) | Only if the closure is rewritten | Yes | Yes |
| Correct for plain sync helpers (P5a, P6, P7, P8-P10) | Yes, if every caller up to a meaningful one is attributed | Yes | Yes | Yes |
| Reaches users correctly | A caller line in the workspace, still a path to a user | Same | No path in the text | No path |
| Fixes `LocatedError` text (fact 2) | Only for the 3 existing `.into()` impls | No | No by itself: needs the wrapper's `Display` changed or the wrapper dropped on these paths | Same as 3 |
| Public API break | None | A new constructor; existing literals break | None if the field stays and only `Display` changes | Yes for 22 variants |
| Churn | One attribute per helper, plus all callers up the chain | New constructor or macro per error type | One format string per variant | One format string, one field, every constructor and test |
| Keeps the developer hint | Yes, when correct | Yes, when correct | In `Debug` and logs while the field stays | No |

## Semver per published crate (AC6)

Versions are the ones in the workspace `Cargo.toml` files at `f8b0f26e9`. "Display only" means no
public type changes and is a patch change for any crate; the bump below applies only to option 4.

| Crate | Version | Variants with a printed location | Option 3 (Display only, field kept) | Option 4 (field removed) |
| --- | --- | --- | --- | --- |
| `torrust-tracker-primitives` | 3.1.0 | 3 (`peer.rs`) | patch | major (4.0.0) |
| `torrust-tracker-configuration` | 3.0.0 | none printed in the inventory | none | none for the printed fields; check in the slice |
| `torrust-tracker-http-protocol` | 0.1.0 | 10 | patch | minor (0.2.0) |
| `torrust-tracker-http-core` | 0.1.0 | through `LocatedError` | patch | minor |
| `torrust-tracker-axum-http-server` | 0.1.0 | through `LocatedError` | patch | minor |
| `torrust-tracker-core` | 0.1.0 | 7 | patch | minor |
| `torrust-tracker-udp-server` | 0.1.0 | 2 | patch | minor |

The table is a working estimate. T5 fixes the bump per crate when it slices, because the printed-field
count of a crate is the number of public variants it would change.

## Recommendation

**Option 3, in its non-breaking form, plus option 1 for the three plain-sync cases that are cheap.**

1. **Stop printing `{location}` in `Display`** for the 22 variants. Keep the field and the capture
   where they are. `Display` is what reaches HTTP, UDP, REST, events and logs (T2), so this closes
   the user-visible part in every crate with no public type change and a patch bump.
2. **Treat `LocatedError` separately.** It prints the location (fact 2), so a variant whose source
   is a `LocatedError` still shows a path. Decide per path in T5, with the maintainer, between:
   (a) not formatting the located source into client text (use the variant's own message), or
   (b) a new `torrust-located-error` release whose `Display` omits the location and keeps it in
   `Debug`. (b) is the smallest per-site change but touches a standalone repository.
3. **Keep the location only where it is correct**, that is where the field is built by plain sync
   helpers (P5a, P6, P7-P10) and `#[track_caller]` reaches a meaningful caller. There it stays in
   `Debug` and in `tracing` fields as a developer hint. For trait impls, closures and `async fn`
   (P1-P4, P5b, P11-P16, W) the capture is wrong by construction, so the slice deletes the capture
   and the field only where a breaking release is already planned; otherwise it leaves them and the
   ADR forbids new ones.
4. **Never write `Location::caller()` in a trait impl, a closure or an `async fn`.** That is the
   rule T9 enforces and the ADR records.

### Why not the others

- **Option 1 alone** leaves a path in user text and cannot work for 27 of the 38 sites (trait impls,
  async, closures, E3 to E8).
- **Option 2** adds a constructor per type and has the same limits through std (D1).
- **Option 4** is a major bump of `torrust-tracker-primitives` for a hint that `Debug` and `tracing`
  already give, and it is not needed to stop the user-visible exposure.

### What this leaves open for the maintainer

1. Accept the non-breaking form (patch bumps) or do the removal together with the next major release.
2. For `LocatedError`: (a) per-path message, or (b) a new `torrust-located-error` release.
3. Whether the `Debug` location of the wrong sites (kept in the field) is acceptable until a
   breaking release, since a wrong hint in logs still misleads maintainers.

## Slicing proposal for T5 (to confirm)

| Slice | Crates | Change | Red test | Version step |
| --- | --- | --- | --- | --- |
| S1 | `primitives` | `Display` of `ParsePeerRoleError`, `IdConversionError` without location | `"x".parse::<PeerRole>()` message has no library path; `Id::try_from` likewise. Closes finding F3 | patch |
| S2 | `http-protocol` | 10 variants, plus the two `torrust-info-hash` conversion sites | response body of a malformed announce and scrape has no library path | patch |
| S3 | `http-core`, `axum-http-server` | `LocatedError` decision (a or b), key and whitelist reasons | same, through the HTTP server | patch |
| S4 | `tracker-core` | 7 variants, database conversion (class C reaches REST and UDP) | REST and UDP reply for a whitelist/database failure has no library path | patch |
| S5 | `udp-server`, `axum-server`, `configuration` | 2 UDP variants, TLS startup text | UDP `ErrorResponse` for a rejected announce | patch |
| S6 | all | Enforcement test and skill update | grep or test from T3 fails on a new `Location::caller()` in a trait impl, closure or `async fn` | none |

## Evidence

- Display formats: `grep -rn '#\[error(.*location' --include=*.rs packages` (22 lines).
- `LocatedError` text: `torrust-located-error-3.0.0/src/lib.rs:58`.
- Test helper: `packages/axum-http-server/tests/server/asserts.rs:10-23`.
- Earlier experiments: `experiments.md` (E3, E4, E6, E8, E9 to E12) and `manual-verification-evidence.md` (D1).
