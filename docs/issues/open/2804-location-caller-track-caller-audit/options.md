---
doc-type: issue-options
issue-spec: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
last-updated-utc: "2026-11-11 11:12"
---

# Issue #2804 - T4: fix options and recommendation

Research record for task T4. It compares the four options of the spec against the inventory
(`inventory.md`, T1-T3) and recommends one. It changes no code. Facts below were read from the tree
at `f8b0f26e9` (`torrust/develop`), from `torrust-located-error` 3.0.0 in the cargo registry and from
the publish plan in `docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`.

## Facts the options depend on

1. **23 `Display` formats print a `{location}` field.** Found with a search that follows each
   attribute to its closing `)]` (see Evidence): 4 in `announce.rs`, 2 in `scrape.rs`, 2 in
   `peer_ip_resolver.rs` (one is a three-line attribute), 1 in `query.rs`, 2 in `auth.rs`, 3 in
   `primitives/src/peer.rs`, 1 in `tracker-core/src/error.rs`, 3 in `databases/error.rs`, 3 in
   `authentication/key/mod.rs`, 2 in `udp-server/src/error.rs`. That is 11 in `http-protocol`, 3 in
   `primitives`, 7 in `tracker-core` and 2 in `udp-server`. One more production variant has a
   `location` field that its format does not print: `MissingTlsConfig` in `axum-server/src/tls.rs:19`,
   so 24 production variants carry the field.
2. **`LocatedError` prints the location itself.** Its `Display` is
   `write!(f, "{}, {}", self.source, self.location)` (`torrust-located-error` 3.0.0, `lib.rs:58`).
   Every error wrapped by `Located(e).into()` or the class-C conversions shows a location whatever the
   wrapped enum does. Changing it needs a new release of that standalone crate. In the workspace
   `src/` trees only `axum-server` (`tls.rs:25`), `configuration`, `http-protocol` and `tracker-core` name a
   `LocatedError`; `http-core` and `axum-http-server` do not.
3. **Few of those messages carry the offending input.** Of the 23 formats, 8 print it
   (`{raw_param}` twice, `{param_name}`/`{param_value}` in four variants, `{key}`, `{info_hash}`),
   3 print a free-text `{message}` (two in `peer.rs`, `Internal` in `udp-server`), and 12 print
   neither: `InvalidKeyFormat`, `CannotExtractKeyParam`, both `MissingParams`, both
   `PeerIpResolutionError` variants, the three `{driver}` variants (which name the database, not the
   input), `KeyExpired`, `MissingAuthKey` and `AuthRequired`. Several of those 12 end in
   `... in {location}`, so removing the location leaves a message that needs new wording, and in a few
   cases a new field.
4. **Tests do not depend on the location text.** `assert_bencoded_error`
   (`axum-http-server/tests/server/asserts.rs:10`) uses `contains` on the failure reason and prints the
   location only in its own failure message. The one test that compares a location string is
   `udp-server/src/event.rs:436` (it builds the expected value from a `Location` it captured).
5. **A field kept but not printed is not a semver break.** Removing `{location}` from a
   `#[error(..)]` string changes the text, not the type. Removing the field itself changes a public
   enum variant.
6. **No production site can be fixed by option 1.** The inventory rules out P1-P4 (trait impls),
   P5a and P6 (reached only through `TryFrom<Query>`), P5b and P14 (closures) and P11-P13, P15 and P16
   (`async`). The remaining production sites are reached from `async` code, where a `#[track_caller]`
   chain ends at another fixed library line: P8 and P9 are called only inside the `async` blocks of
   the axum `from_request_parts` impls (`announce_request.rs:52-54`, `scrape_request.rs:54`), P10
   through `extract_key` from the same kind of block (`authentication_key.rs:80-99`), and P7 through
   `resolve_remote_client_addr` from `async fn handle_announce` and `handle_scrape`
   (`http-core/src/services/announce.rs:204`, `:215`; `scrape.rs:102`, `:115`). So all 38 sites are
   out of reach of option 1. The only correct uses are the three `#[track_caller]` `From` impls that
   already exist.

## Comparison

| Criterion | 1. `#[track_caller]` chain | 2. Capture at the call site | 3. Context instead of location | 4. Remove the field |
| --- | --- | --- | --- | --- |
| Correct for trait impls reached through `parse`/`try_into` (P1-P4, W) | No: points into `core/src/str/mod.rs` (D1, E6) | No: the constructor is still called from the impl | Yes | Yes |
| Correct for `async fn` (P11-P13, P15, P16) | No: attribute is a no-op (E8) | Only if the site is moved to a sync wrapper | Yes | Yes |
| Correct for closures (P5b, P14) | No (E3, E4) | Only if the closure is rewritten | Yes | Yes |
| Correct for sync helpers reached through `TryFrom` or `async` code (P5a, P6, P7-P10) | No: the chain ends at the `TryFrom` impl or an `async` block (fact 6) | No: same callers | Yes | Yes |
| Reaches users correctly | A workspace line at best, still a path to a user | Same | No path in the text | No path |
| Fixes `LocatedError` text (fact 2) | No: the three existing `#[track_caller]` impls make the location correct but it is still printed (`inventory.md`, T2 table) | No | No by itself: needs the wrapper's `Display` changed or the wrapper dropped on these paths | Same as 3 |
| Public API break | None | A new constructor; existing literals break | None if the field stays and only `Display` changes | Yes for 24 variants |
| Churn | One attribute per helper, plus all callers up the chain | New constructor or macro per error type | One format string per variant, and new wording or a new field for the 12 without input (fact 3) | One format string, one field, every constructor and test |
| Keeps the developer hint | Yes, when correct | Yes, when correct | In `Debug` and logs while the field stays | No |

## Semver per published crate (AC6)

Versions are from the workspace `Cargo.toml` files at `f8b0f26e9` and from the publish plan
(`2583-1669-first-publish-of-workspace-crates/ISSUE.md`, `:70`, `:91`, `:130`, checklist rows): the
`0.1.0` crates have never been published and get their first version at `0.1.0` with no bump, so no
consumer sees a step for them. `torrust-tracker-primitives` and `torrust-tracker-configuration` were
published at 3.0.0 and already owe 4.0.0 (`:196`, `:201`). "Display only" changes no public type.

| Crate | Version now, version planned | Variants printing a location | Option 3 (Display only, field kept) | Option 4 (field removed) |
| --- | --- | --- | --- | --- |
| `torrust-tracker-primitives` | 3.1.0, planned 4.0.0 | 3 (`peer.rs`) | patch, inside the planned 4.0.0 | none beyond the planned 4.0.0 |
| `torrust-tracker-configuration` | 3.0.0, planned 4.0.0 | none | none | none |
| `torrust-tracker-http-protocol` | 0.1.0, first publish | 11 | none (first publish) | none (first publish) |
| `torrust-tracker-core` | 0.1.0, first publish | 7 | none (first publish) | none (first publish) |
| `torrust-tracker-udp-server` | 0.1.0, first publish | 2 | none (first publish) | none (first publish) |
| `torrust-tracker-axum-server` | 0.1.0, first publish | 0 (`MissingTlsConfig` has an unprinted field; `BadTlsConfig` has a `LocatedError` source, `tls.rs:19`, `:25`) | none | none |
| `torrust-tracker-http-core`, `torrust-tracker-axum-http-server` | 0.1.0, first publish | 0 (no `location` field and no `LocatedError` in `src/`) | none | none |

Consequence for option 4: while a crate has not been published, removing the field costs it nothing,
and the only breaking step it adds, for `primitives`, is one the plan already schedules. The cost of
option 4 is then the churn and the lost hint, not the semver step. This holds only if the slices land
before the first publish of each crate; once a crate is published, the table rows for it become a
patch (option 3) or a minor/major step (option 4).

## Recommendation

**Option 3: stop printing `{location}` in `Display` and keep the capture only where it is correct, which
is nowhere in production today (fact 6).** Option 1 is not part of the recommendation.

1. **Stop printing `{location}` in `Display`** for the 23 variants. `Display` is what reaches HTTP,
   UDP, REST, events and logs (T2), so this closes the user-visible part. Where the message loses its
   only context (the 12 formats without input, fact 3), give it wording that names what failed, and add
   a field only where the wording cannot. Whether the field itself is kept or removed is open decision
   1 below.
2. **Treat `LocatedError` separately.** It prints the location (fact 2), so a variant whose source
   is a `LocatedError` still shows a path. Decide per path in T5, with the maintainer, between:
   (a) not formatting the located source into client text (use the variant's own message), or
   (b) a new `torrust-located-error` release whose `Display` omits the location and keeps it in
   `Debug`. (b) is the smallest per-site change but touches a standalone repository.
3. **Never write `Location::caller()` in a trait impl, a closure, an `async fn` or code reached from
   them.** That is the rule T9 enforces and the ADR records; with fact 6 it means no new production
   capture of a location.

### Why not the others

- **Option 1** cannot fix any of the 38 production sites (fact 6) and still leaves a path in user
  text.
- **Option 2** adds a constructor per type and has the same limits through std (D1) and the same
  callers (fact 6).
- **Option 4** is not needed to stop the user-visible exposure. It is cheaper than first thought
  (the semver steps are already planned or absent, see above), so it is a real alternative to keeping
  the field: it removes a misleading `Debug` value, at the price of the developer hint and wider churn.

### What this leaves open for the maintainer

1. Keep the field and stop printing it (option 3), or remove the field as well (option 4) in the
   same slices, given that the publish plan makes the semver step free for every crate that is not yet
   published. This is the main decision for T5.
2. For `LocatedError`: (a) per-path message, or (b) a new `torrust-located-error` release.
3. If the field is kept, whether a wrong `Debug` location in logs is acceptable until it is removed.

## Slicing proposal for T5 (to confirm)

| Slice | Crates | Change | Red test | Version step |
| --- | --- | --- | --- | --- |
| S1 | `primitives` | `Display` of `ParsePeerRoleError`, `IdConversionError` without location | `"x".parse::<PeerRole>()` message has no library path; `Id::try_from` likewise. Closes finding F3 | inside the planned 4.0.0 |
| S2 | `http-protocol` | 11 variants. The `torrust-info-hash` case is excluded from this slice: its two sites are in a registry crate (`inventory.md`, standalone crates) and the workspace wrapping sites (`announce.rs:543`, `scrape.rs:84`) print a second location until S3 settles `LocatedError` | response body of a malformed announce and scrape has no library path, for the `Display` variants of the slice | first publish |
| S3 | `http-core`, `axum-http-server`, `http-protocol` wrapping sites | `LocatedError` decision (a or b), key and whitelist reasons, and the info-hash case of S2 | same, through the HTTP server, including a malformed `info_hash` | first publish |
| S4 | `tracker-core` | 7 variants, database conversion (class C reaches REST and UDP); the whitelist variant also reaches UDP announce replies | REST reply for a whitelist/database failure and UDP reply for a non-whitelisted announce have no library path | first publish |
| S5 | `axum-server`, `configuration` | TLS startup text (`MissingTlsConfig`, `BadTlsConfig`) | startup error text for a missing certificate has no library path | first publish |
| S6 | `udp-server` | The two variants `Internal` and `AuthRequired` are built only in `#[cfg(test)]` code, so they need no red test of their own; their `Display` is changed with S4's test as the coverage of the UDP path | none of its own (recorded) | first publish |
| S7 | all | Enforcement test and skill update | grep or test from T3 fails on a new `Location::caller()` in a trait impl, closure or `async fn` | none |

## Evidence

- Display formats, following each attribute to its closing `)]`:
  `rg -U --multiline-dotall --count-matches '#\[error\([^\]]*location[^\]]*\)\]' packages --glob '*.rs'`
  (23 matches over 10 files, at `f8b0f26e9`). A one-line `grep` misses
  `peer_ip_resolver.rs:117-120`.
- `LocatedError` text: `torrust-located-error-3.0.0/src/lib.rs:58`.
- Test helper: `packages/axum-http-server/tests/server/asserts.rs:10-23`.
- Publish plan: `docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`.
- Earlier experiments: `experiments.md` (E3, E4, E6, E8, E9 to E12) and `manual-verification-evidence.md` (D1).
