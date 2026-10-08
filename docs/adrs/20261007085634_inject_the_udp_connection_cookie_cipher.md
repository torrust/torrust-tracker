---
semantic-links:
  skill-links:
    - create-adr
    - handle-secrets
  related-artifacts:
    - docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/src/crypto/cookie_cipher.rs
    - packages/udp-core/src/container.rs
    - src/bootstrap/app.rs
---

<!-- skill-link: create-adr -->

# Inject the UDP Connection-Cookie Cipher Instead of Using Global Statics

## Scope

Repository-wide. The decision changes the public `udp-core` cookie API (`make` and `check`) and the
`udp-core` composition types. Those are consumed by `udp-server`, the tracker application's
bootstrap, and the test environments of other packages. A change to an inter-package contract
belongs in `docs/adrs/`.

## Description

BEP 15 connection IDs ("cookies") stop a client from announcing or scraping for an address it does
not control. `udp-core` builds a cookie by adding a fingerprint of the client socket address to the
issue time and encrypting the 8 bytes with Blowfish. The secrecy of the Blowfish key is the whole
protection: anyone who knows the key can forge a valid cookie for any address and time.

Until this decision, the key was a hidden global dependency:

- `crypto::ephemeral_instance_keys` defined `LazyLock` statics for a random seed, a random cipher,
  and a cipher keyed with 32 zero bytes. None was behind `#[cfg(test)]`.
- `crypto::keys` chose between the random and the zeroed cipher with `#[cfg(test)]` /
  `#[cfg(not(test))]` `pub use` aliases behind a `Keeper` trait and `Current`/`Instance` facades.
- `make` and `check` read the cipher through `Current`, so callers could not pass a key.
- The application bootstrap ran `check_seed()` to detect a test key in production, but it compared
  the seed, which no cookie uses, not the cipher.

So production safety rested on one `cfg` alias, the startup safeguard checked the wrong value, and
a public all-zero key was compiled into, and initialized in, every production binary. Issue #2458
reproduced the consequence: with the alias pointed at the zero cipher, the tracker started and
accepted a cookie forged with the public key.

The root cause is the global dependency, not one line. Because `make` and `check` could not receive
a key, deterministic tests needed a compile-time swap, and protecting production from that swap
needed a runtime check of a proxy value.

## Agreement

Create the cookie key as an ordinary value at startup and inject it into everything that issues or
validates connection IDs.

- **Key type.** `udp-core` defines one type that owns the Blowfish cipher, for example:

  ```rust
  pub struct CookieCipher(BlowfishLE);

  impl CookieCipher {
      /// Production constructor: a key drawn from a cryptographically secure RNG.
      pub fn random() -> Self { /* ... */ }

      /// Fixed key for deterministic tests. It does not exist outside this crate's test build.
      #[cfg(test)]
      pub(crate) fn fixed_for_testing() -> Self { /* ... */ }
  }
  ```

  It has no mutation API and no accessor that exposes key material. Encryption and decryption are
  crate-private operations used by the cookie builder.
- **Explicit API.** `connection_cookie::make` and `connection_cookie::check` take
  `&CookieCipher` as a parameter. There is no process-global key, no `Keeper` trait, no
  `Current`/`Instance` facade, and no `cfg`-selected alias.
- **Ownership.** `UdpTrackerCoreServices` creates the key once and holds it in an `Arc`. Every
  `UdpTrackerCoreContainer` built from those services gives clones of that `Arc` to
  `ConnectService`, `AnnounceService`, and `ScrapeService`, which take it as a required
  constructor argument. In the tracker application, `AppContainer` builds
  `UdpTrackerCoreServices` once and shares it with every UDP tracker instance, so the process has
  exactly one key. A standalone constructor that builds its own services, used by tests and
  standalone servers, is its own composition root and creates its own single key. No component
  creates a key for itself.
- **Validation outside the services.** `udp-server` does not receive the key. When connection-ID
  validation is disabled, its handlers still check the connection ID to observe invalid ones; they
  call the public `AnnounceService::authenticate` and `ScrapeService::authenticate`, which use the
  injected key. Keeping the key inside `udp-core`'s services means fewer components hold it, and
  the check cannot use a different key from the one the services use. The compiler enforces it:
  `UdpTrackerCoreServices::cookie_cipher` is `pub(crate)`, and a `compile_fail` doctest pins that
  code outside `udp-core` cannot read it.
- **Test key.** The fixed test key exists only under `#[cfg(test)]` in `udp-core`. A production
  build, including every downstream crate and its tests, cannot name it. A maintained
  `compile_fail` doctest pins this. Its error code is checked only by rustdoc on the nightly
  toolchain, which the CI unit job runs; stable rustdoc accepts any compile error. Tests in other
  crates use `CookieCipher::random()`: the `udp-server` handler tests share one random key through a
  test-only helper, as one composition root shares it, and each benchmark builds its own.
- **Startup checks.** `check_seed()`, the unused random seed, and the key steps of
  `initialize_static()` are removed. The guarantees they were meant to give now hold by
  construction and are pinned by tests.
- **Diagnostics.** The key type implements `Debug` by hand as `CookieCipher([REDACTED])`, the
  representation the secrecy ADR uses, and tests assert that exact output and the absence of a
  unique test key's bytes. Every `#[instrument]` function that receives the key skips it.
- **Memory.** Enable the `blowfish` crate's `zeroize` feature, so the key schedule is wiped when a
  key is dropped, and build the cipher from raw key bytes held in a `zeroize::Zeroizing` buffer,
  filled in place, so they are wiped as soon as the cipher exists. `zeroize` was already in the
  dependency graph through `secrecy`; `udp-core` depends on it directly with default features off,
  so this adds no new crate. Wiping is best effort: it cannot reach copies the compiler or the
  operating system may have made, and a test cannot observe it without undefined behavior.

### Deviation from the Secrecy ADR

[Adopt `secrecy` for sensitive values](20260822094338_adopt_secrecy_for_sensitive_values.md) asks
for `secrecy` types to be used directly. The cookie key cannot follow it literally:

- `secrecy::SecretBox<S>` requires `S: Zeroize`. `blowfish` 0.10 implements only `ZeroizeOnDrop`,
  behind its `zeroize` feature, so `SecretBox<BlowfishLE>` does not compile.
- Wrapping only the 32 raw key bytes in `SecretBox` would make every `make` and `check` rerun the
  Blowfish key schedule, which is far more expensive than one block encryption, on the hottest UDP
  path.

The key type therefore keeps the secrecy ADR's intent by hand: the same redacted representation,
no exposing accessor, the same test assertions, and wiping on drop. If a later `blowfish` or
`secrecy` release makes `SecretBox<BlowfishLE>` possible, prefer it and revisit this section.

## Alternatives Considered

**Keep the statics and gate the zero key behind `#[cfg(test)]`.** This removes the public test key
from production builds, but `make` and `check` still read a global, so tests cannot choose a key,
and the key stays hidden from readers of every caller. It fixes the symptom and keeps the cause.

**Derive the cipher from the seed and keep `check_seed()`.** This makes the existing safeguard
check the value that matters, but only at runtime, only in the tracker binary, and only against a
mistake in one alias. The zero key would still be compiled into production.

**Keep a `static` for the production key because it is safer.** A `static` gives no protection that
a process-local immutable value lacks: both live in the same address space, and safe Rust can
mutate neither. Injection removes the real risk, a public key reachable from production, and adds
two duties that this decision makes explicit: never log the key, and share one key per composition
root.

**Wrap the cipher in `secrecy`.** Not possible today; see the deviation section.

## Consequences

### Positive

- The fixed test key is unreachable from production code, and a test proves it.
- Tests choose their key explicitly instead of depending on how a crate was compiled.
- Every cookie consumer shows its dependency on the key in its signature.
- The key cannot appear in `Debug` output or tracing spans, and it is wiped on drop.
- Sharing one key across several tracker processes, for example behind a load balancer, becomes
  possible later by injecting a configured key. It is out of scope here.

### Negative

- Every cookie consumer's constructor or signature changes, including tests and benchmarks in
  other crates.
- A component that builds its own key would silently reject every cookie issued by another.
  A container collaboration test guards the single shared key.
- The redaction is hand-written instead of coming from `secrecy`.

## Affected Code

- [`connection_cookie.rs`](../../packages/udp-core/src/connection_cookie.rs): `make`, `check`, and
  the cookie builder.
- [`crypto/`](../../packages/udp-core/src/crypto/): the key type, which replaces `keys.rs` and
  `ephemeral_instance_keys.rs`.
- [`container.rs`](../../packages/udp-core/src/container.rs): key ownership and wiring.
- [`services/`](../../packages/udp-core/src/services/): the services that hold the key, and the
  public `authenticate` methods.
- [`app.rs`](../../src/bootstrap/app.rs): removal of `check_seed()` and the key initialization.
- `udp-server` handlers, which call `authenticate` on the disabled-validation path, and the
  `udp-server` tests and benchmarks.
- Every caller of `torrust_tracker_udp_core::initialize_static()`, which has nothing left to
  initialize and is removed: the tracker bootstrap and the `udp-server` and `axum-rest-api-server`
  servers and test environments.

## Date

2026-10-07

## References

- Issue #2458: [Inject the UDP connection-cookie cipher](../issues/open/2458-inject-udp-cookie-cipher/ISSUE.md)
- Issue #1348: `udp-core` package tests, where the problem was found
- [Adopt `secrecy` for sensitive values](20260822094338_adopt_secrecy_for_sensitive_values.md)
- Commit `e3562f069` "udp: symmetric encrypted cookie" (2024-11-18), which introduced the cipher
  and `check_seed()`
- [BEP 15: UDP Tracker Protocol](https://www.bittorrent.org/beps/bep_0015.html)
