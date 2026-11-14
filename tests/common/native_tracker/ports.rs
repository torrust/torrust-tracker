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

/// How many random candidates are tried for each port before giving up. Most of the range is free,
/// so exhausting it means the machine has no usable loopback.
const DRAWS_PER_PORT: usize = 1_000;

/// Returns `N` distinct loopback TCP ports from [`SCENARIO_PORTS`] that were free when this
/// function returned.
///
/// Each port is probed by binding it, and every listener is held until the last port is chosen, so
/// no port is returned twice: a candidate that repeats a port already held fails to bind and is
/// skipped. The listeners are released before the function returns, so a port can still be taken
/// before the tracker binds it, but only by a process that listens on that exact port. That includes
/// a child process that another thread forked while the listener was open and that has not yet
/// called `exec`, which is why a caller must never assume that a port it was just given can be bound
/// again at once. Concurrent runs draw different random ports, and the kernel never assigns this
/// range by default.
///
/// # Panics
///
/// Panics when `N` is larger than the range holds, and when [`DRAWS_PER_PORT`] consecutive
/// candidates yield no free port.
pub fn reserve_free_ports<const N: usize>() -> [u16; N] {
    let random = RandomState::new();
    let span = u64::from(SCENARIO_PORTS.end - SCENARIO_PORTS.start);
    let mut candidates = (0..).map(|draw: u64| {
        let offset = u16::try_from(random.hash_one(draw) % span).expect("an offset below the span should fit a port");
        SCENARIO_PORTS.start + offset
    });

    let held = reserve_from(N, &mut candidates, DRAWS_PER_PORT, bind_loopback);

    let mut ports = [0; N];
    for (port, (reserved, _listener)) in ports.iter_mut().zip(&held) {
        *port = *reserved;
    }

    ports
}

/// Binds `port` on the loopback interface, or returns nothing when it cannot be bound.
fn bind_loopback(port: u16) -> Option<TcpListener> {
    TcpListener::bind((Ipv4Addr::LOCALHOST, port)).ok()
}

/// Reserves `count` distinct ports from `candidates`, giving each port up to `draws_per_port` of
/// them and keeping whatever `try_bind` returns for it, so the port stays taken until the caller
/// drops the result. `try_bind` is a parameter so the budget logic can be tested without sockets.
fn reserve_from<L>(
    count: usize,
    candidates: &mut impl Iterator<Item = u16>,
    draws_per_port: usize,
    mut try_bind: impl FnMut(u16) -> Option<L>,
) -> Vec<(u16, L)> {
    let available = usize::from(SCENARIO_PORTS.end - SCENARIO_PORTS.start);
    assert!(
        count <= available,
        "cannot reserve {count} ports from a range of {available} ports"
    );

    let mut held = Vec::with_capacity(count);
    while held.len() < count {
        let reserved = candidates
            .by_ref()
            .take(draws_per_port)
            .find_map(|port| try_bind(port).map(|listener| (port, listener)))
            .unwrap_or_else(|| {
                panic!(
                    "found only {} free loopback ports in {SCENARIO_PORTS:?} within {draws_per_port} draws per port while reserving {count}",
                    held.len()
                )
            });
        held.push(reserved);
    }

    held
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::net::{Ipv4Addr, TcpListener};

    use super::{DEFAULT_EPHEMERAL_START, SCENARIO_PORTS, bind_loopback, reserve_free_ports, reserve_from};

    /// A binder that has no sockets: it accepts a port once, and refuses `busy` ports and ports it
    /// already handed out, as a held listener would.
    fn fake_binder(busy: &[u16]) -> impl FnMut(u16) -> Option<()> {
        let mut taken: HashSet<u16> = busy.iter().copied().collect();

        move |port| taken.insert(port).then_some(())
    }

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
    fn it_should_not_bind_a_port_that_a_live_listener_of_ours_holds() {
        let held = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let held_port = held.local_addr().unwrap().port();

        assert!(bind_loopback(held_port).is_none());
    }

    #[test]
    fn it_should_skip_a_candidate_that_cannot_be_bound() {
        let mut candidates = [10_001, 10_002].into_iter();

        let held = reserve_from(1, &mut candidates, 2, fake_binder(&[10_001]));

        assert_eq!(held.iter().map(|(port, ())| *port).collect::<Vec<_>>(), [10_002]);
    }

    #[test]
    fn it_should_give_each_port_its_own_draw_budget_even_when_the_candidates_repeat() {
        // The first port is found at once and the second only on the fourth candidate, after three
        // draws that hit a port already held. A budget shared by both ports would have run out.
        let mut candidates = [10_001, 10_001, 10_001, 10_002].into_iter();

        let held = reserve_from(2, &mut candidates, 3, fake_binder(&[]));

        assert_eq!(held.iter().map(|(port, ())| *port).collect::<Vec<_>>(), [10_001, 10_002]);
    }

    #[test]
    #[should_panic(expected = "found only 1 free loopback ports in 10000..32768 within 2 draws per port while reserving 2")]
    fn it_should_name_the_range_the_budget_and_the_count_when_a_port_cannot_be_found() {
        let mut candidates = [10_001, 10_001, 10_001].into_iter();

        drop(reserve_from(2, &mut candidates, 2, fake_binder(&[])));
    }

    #[test]
    #[should_panic(expected = "cannot reserve 22769 ports from a range of 22768 ports")]
    fn it_should_refuse_to_reserve_more_ports_than_the_range_holds() {
        let _ = reserve_free_ports::<22_769>();
    }
}
