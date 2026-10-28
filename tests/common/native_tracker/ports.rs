//! Reserves loopback ports for scenarios that must name a bind address before the tracker starts.
//!
//! A hand-picked port is not safe in the kernel's ephemeral range, which is where `43152` and its
//! neighbors sit: the kernel also hands that range to port-zero bindings and to the source side of
//! every outgoing connection, so any process on the machine can be holding the port when the tracker
//! child tries to bind it, and the child then exits with `Address already in use`. A port taken from
//! the kernel with port zero comes from the same range, so it only narrows the window. This module
//! draws random ports below the default Linux ephemeral range instead, where only a deliberate
//! listener can hold one on a default configuration, and probes each by binding it. A host that
//! lowers `net.ipv4.ip_local_port_range` below that default narrows the guarantee. It must not
//! start, probe, or stop a tracker.

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::net::{Ipv4Addr, TcpListener};
use std::ops::Range;

/// Where Linux's ephemeral port range begins by default (`net.ipv4.ip_local_port_range`). A host can
/// configure a lower start; the scenarios assume the default.
const DEFAULT_EPHEMERAL_START: u16 = 32_768;

/// The ports a scenario may use. It ends where the default ephemeral range begins, so on a default
/// configuration no kernel-assigned port can land in it.
const SCENARIO_PORTS: Range<u16> = 10_000..DEFAULT_EPHEMERAL_START;

/// How many candidates are tried before giving up. Most of the range is free, so reaching it means
/// the machine has no usable loopback.
const MAX_DRAWS: u64 = 1_000;

/// Returns `N` distinct loopback TCP ports from [`SCENARIO_PORTS`] that were free when this
/// function returned.
///
/// Each port is probed by binding it, and every listener is held until the last port is chosen, so
/// no port is returned twice. The listeners are released before the function returns, so a port can
/// still be taken before the tracker binds it, but only by a process that listens on that exact
/// port: concurrent runs draw different random ports, and the kernel never assigns this range.
///
/// # Panics
///
/// Panics when `N` exceeds [`MAX_DRAWS`], because the search budget could not cover the request, and
/// when the [`MAX_DRAWS`] candidates, shared by all `N` ports, hold fewer than `N` free ports.
pub fn reserve_free_ports<const N: usize>() -> [u16; N] {
    assert!(
        u64::try_from(N).is_ok_and(|requested| requested <= MAX_DRAWS),
        "cannot reserve {N} ports with a budget of {MAX_DRAWS} draws"
    );

    let random = RandomState::new();
    let span = u64::from(SCENARIO_PORTS.end - SCENARIO_PORTS.start);
    let mut candidates = (0..MAX_DRAWS).map(|draw| {
        let offset = u16::try_from(random.hash_one(draw) % span).expect("an offset below the span should fit a port");
        SCENARIO_PORTS.start + offset
    });

    let mut held = Vec::with_capacity(N);
    while held.len() < N {
        let listener = bind_first_free(&mut candidates).unwrap_or_else(|| {
            panic!(
                "found only {} free loopback ports in {SCENARIO_PORTS:?} within {MAX_DRAWS} draws while reserving {N}",
                held.len()
            )
        });
        held.push(listener);
    }

    let mut ports = [0; N];
    for (port, listener) in ports.iter_mut().zip(&held) {
        *port = listener
            .local_addr()
            .expect("a bound listener should report its address")
            .port();
    }

    ports
}

/// Binds the first candidate that is free, consuming the candidates up to and including it.
fn bind_first_free(candidates: &mut impl Iterator<Item = u16>) -> Option<TcpListener> {
    candidates.find_map(|port| TcpListener::bind((Ipv4Addr::LOCALHOST, port)).ok())
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::net::{Ipv4Addr, TcpListener};

    use super::{DEFAULT_EPHEMERAL_START, SCENARIO_PORTS, bind_first_free, reserve_free_ports};

    #[test]
    fn it_should_reserve_the_requested_number_of_distinct_ports() {
        let ports = reserve_free_ports::<8>();

        assert_eq!(ports.iter().collect::<HashSet<_>>().len(), 8, "ports were {ports:?}");
    }

    #[test]
    fn it_should_reserve_only_ports_below_the_default_ephemeral_range() {
        for _ in 0..20 {
            for port in reserve_free_ports::<3>() {
                assert!(
                    SCENARIO_PORTS.contains(&port) && port < DEFAULT_EPHEMERAL_START,
                    "port {port} is outside the scenario range and could be handed out by the kernel by default"
                );
            }
        }
    }

    #[test]
    fn it_should_skip_a_candidate_that_a_live_listener_holds() {
        let held = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let held_port = held.local_addr().unwrap().port();

        assert!(bind_first_free(&mut std::iter::once(held_port)).is_none());
    }

    #[test]
    #[should_panic(expected = "cannot reserve 1001 ports with a budget of 1000 draws")]
    fn it_should_refuse_to_reserve_more_ports_than_it_can_search_for() {
        let _ = reserve_free_ports::<1001>();
    }

    #[test]
    fn it_should_return_nothing_when_there_are_no_candidates() {
        assert!(bind_first_free(&mut std::iter::empty()).is_none());
    }
}
