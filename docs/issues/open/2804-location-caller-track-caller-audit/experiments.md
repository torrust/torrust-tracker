---
doc-type: issue-experiments
issue-spec: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
last-updated-utc: "2026-11-10 21:51"
---

# Experiments E1 to E12: where `Location::caller()` reports what

Programs, commands and output behind the table in [`inventory.md`](inventory.md). Toolchain: stable
`rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`, on Linux x86_64. `torrust-located-error` 3.0.0.

## E1 to E8: plain `rustc`

Command (one file, no dependencies): `rustc --edition 2021 track_caller_shapes.rs -o shapes && ./shapes`.

```rust
use std::future::Future;
use std::panic::Location;

#[track_caller]
fn loc() -> &'static Location<'static> {
    Location::caller()
}

fn helper() -> &'static Location<'static> {
    loc()
}

fn via_closure() -> &'static Location<'static> {
    let f = || loc();
    f()
}

fn via_map() -> &'static Location<'static> {
    Some(1).map(|_| loc()).unwrap()
}

trait Tr {
    fn m(&self) -> &'static Location<'static>;
}
struct S;
impl Tr for S {
    #[track_caller]
    fn m(&self) -> &'static Location<'static> {
        Location::caller()
    }
}
fn generic<X: Tr>(x: &X) -> &'static Location<'static> {
    x.m()
}

#[track_caller]
const fn const_tc() -> &'static Location<'static> {
    Location::caller()
}

#[track_caller]
async fn async_tc() -> &'static Location<'static> {
    Location::caller()
}

fn main() {
    println!("E1 direct call of a track_caller fn      : {}", loc());
    println!("E2 helper without track_caller           : {}", helper());
    println!("E3 closure calling a track_caller fn     : {}", via_closure());
    println!("E4 map(|_| loc())                        : {}", via_map());
    println!("E5 track_caller trait impl, direct call  : {}", S.m());
    println!("E6 same method through a generic bound   : {}", generic(&S));
    println!("E7 track_caller const fn                 : {}", const_tc());
    let waker = std::task::Waker::noop();
    let mut cx = std::task::Context::from_waker(waker);
    let mut fut = Box::pin(async_tc());
    if let std::task::Poll::Ready(l) = fut.as_mut().poll(&mut cx) {
        println!("E8 track_caller async fn                 : {l}");
    }
}
```

Compiler output: `warning: #[track_caller] on async functions is a no-op` (lint
`ungated_async_fn_track_caller`). Program output:

```text
E1 direct call of a track_caller fn      : track_caller_shapes.rs:47:63
E2 helper without track_caller           : track_caller_shapes.rs:10:5
E3 closure calling a track_caller fn     : track_caller_shapes.rs:14:16
E4 map(|_| loc())                        : track_caller_shapes.rs:19:21
E5 track_caller trait impl, direct call  : track_caller_shapes.rs:51:65
E6 same method through a generic bound   : track_caller_shapes.rs:33:7
E7 track_caller const fn                 : track_caller_shapes.rs:53:63
E8 track_caller async fn                 : track_caller_shapes.rs:43:5
```

Reading: E1, E5 and E7 report the line of the call in `main` (47, 51 and 53). E2, E3, E4 and E6
report a line inside the intermediate function or closure (10, 14, 19, 33). E8 reports line 43,
inside the `async fn` body, so the attribute does nothing.

## E9 to E12: `torrust-located-error`

Command: the program below was placed as `packages/configuration/tests/located_error_shapes.rs`
(deleted after the run) and run with
`cargo test -p torrust-tracker-configuration --test located_error_shapes -- --nocapture`.

```rust
use std::sync::Arc;

use torrust_located_error::{DynError, Located, LocatedError};

#[derive(Debug, thiserror::Error)]
#[error("boom")]
struct Boom;

fn helper() -> LocatedError<'static, Boom> {
    Located(Boom).into()
}

fn helper_dyn() -> LocatedError<'static, dyn std::error::Error + Send + Sync> {
    (Arc::new(Boom) as DynError).into()
}

#[test]
fn located() {
    let direct: LocatedError<'static, Boom> = Located(Boom).into();
    println!("E9  Located(..).into() written inline   : {direct}");
    println!("E10 Located(..).into() inside a helper  : {}", helper());
    println!("E11 DynError .into() inside a helper    : {}", helper_dyn());
    let closure = || -> LocatedError<'static, Boom> { Located(Boom).into() };
    println!("E12 Located(..).into() inside a closure : {}", closure());
}
```

```text
E9  Located(..).into() written inline   : boom, packages/configuration/tests/located_error_shapes.rs:19:61
E10 Located(..).into() inside a helper  : boom, packages/configuration/tests/located_error_shapes.rs:10:19
E11 DynError .into() inside a helper    : boom, packages/configuration/tests/located_error_shapes.rs:14:34
E12 Located(..).into() inside a closure : boom, packages/configuration/tests/located_error_shapes.rs:23:69
```

Reading: `Display` of a `LocatedError` is `{source}, {location}`; E9 reports the call line (19), E10 and
E11 the line inside the helper (10, 14), and E12 the closure line (23).
