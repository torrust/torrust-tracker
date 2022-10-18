use std::net::SocketAddr;

use aquatic_udp_protocol::ConnectionId;

use crate::protocol::clock::time_extent::{Extent, TimeExtent};
use crate::udp::ServerError;

pub type Cookie = [u8; 8];

pub type SinceUnixEpochTimeExtent = TimeExtent;

pub const COOKIE_LIFETIME: TimeExtent = TimeExtent::from_sec(2, &60);

pub fn from_connection_id(connection_id: &ConnectionId) -> Cookie {
    connection_id.0.to_le_bytes()
}

pub fn into_connection_id(connection_cookie: &Cookie) -> ConnectionId {
    ConnectionId(i64::from_le_bytes(*connection_cookie))
}

pub fn make_connection_cookie(remote_address: &SocketAddr) -> Cookie {
    let time_extent = cookie_builder::get_last_time_extent();

    let cookie = cookie_builder::build(remote_address, &time_extent);
    println!("remote_address: {remote_address:?}, time_extent: {time_extent:?}, cookie: {cookie:?}");
    cookie
}

pub fn check_connection_cookie(
    remote_address: &SocketAddr,
    connection_cookie: &Cookie,
) -> Result<SinceUnixEpochTimeExtent, ServerError> {
    let last_time_extent = cookie_builder::get_last_time_extent();
    let cookie = cookie_builder::build(remote_address, &last_time_extent);
    println!("remote_address: {remote_address:?}, time_extent: {last_time_extent:?}, cookie: {cookie:?}");

    // we loop backwards testing each time_extent until we find one that matches.
    // (or the lifetime of time_extents is exhausted)
    for offset in 0..=COOKIE_LIFETIME.amount {
        let checking_time_extent = cookie_builder::get_last_time_extent().decrease(offset).unwrap();

        let checking_cookie = cookie_builder::build(remote_address, &checking_time_extent);
        //println!("remote_address: {remote_address:?}, time_extent: {checking_time_extent:?}, cookie: {checking_cookie:?}");

        if *connection_cookie == checking_cookie {
            return Ok(checking_time_extent);
        }
    }
    Err(ServerError::InvalidConnectionId)
}

mod cookie_builder {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::net::SocketAddr;

    use super::{Cookie, SinceUnixEpochTimeExtent, COOKIE_LIFETIME};
    use crate::protocol::clock::time_extent::{DefaultTimeExtentMaker, Extent, MakeTimeExtent, TimeExtent};
    use crate::protocol::crypto::keys::seeds::{DefaultSeed, SeedKeeper};

    pub(super) fn get_last_time_extent() -> SinceUnixEpochTimeExtent {
        DefaultTimeExtentMaker::now(&COOKIE_LIFETIME.increment)
            .unwrap()
            .unwrap()
            .increase(COOKIE_LIFETIME.amount)
            .unwrap()
    }

    pub(super) fn build(remote_address: &SocketAddr, time_extent: &TimeExtent) -> Cookie {
        let seed = DefaultSeed::get_seed();

        let mut hasher = DefaultHasher::new();

        remote_address.hash(&mut hasher);
        time_extent.hash(&mut hasher);
        seed.hash(&mut hasher);

        //println!("remote_address: {remote_address:?}, time_extent: {time_extent:?}, seed: {seed:?}");

        hasher.finish().to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::cookie_builder::get_last_time_extent;
    use crate::protocol::clock::time_extent::Extent;
    use crate::protocol::clock::{StoppedClock, StoppedTime};
    use crate::udp::connection_cookie::{check_connection_cookie, make_connection_cookie, Cookie, COOKIE_LIFETIME};

    fn make_test_socket_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080)
    }

    fn make_test_cookie(remote_address: Option<&SocketAddr>) -> Cookie {
        make_connection_cookie(remote_address.unwrap_or(&make_test_socket_addr()))
    }

    /// this test is strange, the default hasher seems to make different values depending if we are running stable ot nightly
    #[test]
    fn it_should_make_a_connection_cookie() {
        // remote_address: 127.0.0.1:8080, time_extent: 60,
        // seed: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        const ID_STABLE: Cookie = [223, 239, 248, 49, 67, 103, 97, 83];
        const ID_NIGHTLY: Cookie = [45, 59, 50, 101, 97, 203, 48, 19];

        let test_cookie = make_test_cookie(None);
        println!("{test_cookie:?}");

        assert!((test_cookie == ID_STABLE || (test_cookie == ID_NIGHTLY)))
    }

    #[test]
    fn it_should_make_different_connection_cookie_with_different_remote_addresses() {
        let test_remote_address_1 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 1);
        let test_remote_address_2 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 2);
        let test_remote_address_3 = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 1);

        assert_ne!(
            make_test_cookie(Some(&test_remote_address_1)),
            make_test_cookie(Some(&test_remote_address_2))
        );

        assert_ne!(
            make_test_cookie(Some(&test_remote_address_1)),
            make_test_cookie(Some(&test_remote_address_3))
        );

        assert_ne!(
            make_test_cookie(Some(&test_remote_address_2)),
            make_test_cookie(Some(&test_remote_address_3))
        )
    }

    #[test]
    fn it_should_make_different_cookies_for_the_next_time_extent() {
        let cookie_now = make_test_cookie(None);

        StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

        let cookie_next = make_test_cookie(None);

        assert_ne!(cookie_now, cookie_next)
    }

    #[test]
    fn it_should_be_valid_for_this_time_extent() {
        let cookie_now = make_test_cookie(None);

        check_connection_cookie(&make_test_socket_addr(), &cookie_now).unwrap();
    }

    #[test]
    fn it_should_be_valid_for_the_next_time_extent() {
        let cookie_now = make_test_cookie(None);

        StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

        check_connection_cookie(&make_test_socket_addr(), &cookie_now).unwrap();
    }

    #[test]
    fn it_cookies_should_be_valid_for_the_last_time_extent() {
        let cookie_now = make_test_cookie(None);

        StoppedClock::local_set(&COOKIE_LIFETIME.total().unwrap().unwrap());

        check_connection_cookie(&make_test_socket_addr(), &cookie_now).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_cookies_should_be_not_valid_after_their_last_time_extent() {
        let cookie_now = make_test_cookie(None);

        let last_time_extent = get_last_time_extent().increase(COOKIE_LIFETIME.amount).unwrap();

        StoppedClock::local_set(&last_time_extent.total_next().unwrap().unwrap());

        check_connection_cookie(&make_test_socket_addr(), &cookie_now).unwrap();
    }
}
