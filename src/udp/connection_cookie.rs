use std::net::SocketAddr;

use aquatic_udp_protocol::ConnectionId;

use super::errors::ServerError;
use crate::protocol::clock::time_extent::{Extent, TimeExtent};

pub type Cookie = [u8; 8];

pub type SinceUnixEpochTimeExtent = TimeExtent;

pub const COOKIE_LIFETIME: TimeExtent = TimeExtent::from_sec(2, &60);

#[must_use]
pub fn from_connection_id(connection_id: &ConnectionId) -> Cookie {
    connection_id.0.to_le_bytes()
}

#[must_use]
pub fn into_connection_id(connection_cookie: &Cookie) -> ConnectionId {
    ConnectionId(i64::from_le_bytes(*connection_cookie))
}

#[must_use]
pub fn make_connection_cookie(remote_address: &SocketAddr) -> Cookie {
    let time_extent = cookie_builder::get_last_time_extent();

    //println!("remote_address: {remote_address:?}, time_extent: {time_extent:?}, cookie: {cookie:?}");
    cookie_builder::build(remote_address, &time_extent)
}

pub fn check_connection_cookie(
    remote_address: &SocketAddr,
    connection_cookie: &Cookie,
) -> Result<SinceUnixEpochTimeExtent, ServerError> {
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

        hasher.finish().to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::cookie_builder::{self};
    use crate::protocol::clock::time_extent::{self, Extent};
    use crate::protocol::clock::{StoppedClock, StoppedTime};
    use crate::udp::connection_cookie::{check_connection_cookie, make_connection_cookie, Cookie, COOKIE_LIFETIME};

    // #![feature(const_socketaddr)]
    // const REMOTE_ADDRESS_IPV4_ZERO: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

    #[test]
    fn it_should_make_a_connection_cookie() {
        let cookie = make_connection_cookie(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));

        // Note: This constant may need to be updated in the future as the hash is not guaranteed to to be stable between versions.
        const ID_COOKIE: Cookie = [23, 204, 198, 29, 48, 180, 62, 19];

        assert_eq!(cookie, ID_COOKIE)
    }

    #[test]
    fn it_should_make_the_same_connection_cookie_for_the_same_input_data() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let time_extent_zero = time_extent::ZERO;

        let cookie = cookie_builder::build(&remote_address, &time_extent_zero);
        let cookie_2 = cookie_builder::build(&remote_address, &time_extent_zero);

        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

        //remote_address: 127.0.0.1:8080, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [212, 9, 204, 223, 176, 190, 150, 153]
        //remote_address: 127.0.0.1:8080, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [212, 9, 204, 223, 176, 190, 150, 153]

        assert_eq!(cookie, cookie_2)
    }

    #[test]
    fn it_should_make_the_different_connection_cookie_for_different_ip() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let remote_address_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 0);
        let time_extent_zero = time_extent::ZERO;

        let cookie = cookie_builder::build(&remote_address, &time_extent_zero);
        let cookie_2 = cookie_builder::build(&remote_address_2, &time_extent_zero);

        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
        println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

        //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
        //remote_address: 255.255.255.255:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [217, 87, 239, 178, 182, 126, 66, 166]

        assert_ne!(cookie, cookie_2)
    }

    #[test]
    fn it_should_make_the_different_connection_cookie_for_different_ip_version() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let remote_address_2 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
        let time_extent_zero = time_extent::ZERO;

        let cookie = cookie_builder::build(&remote_address, &time_extent_zero);
        let cookie_2 = cookie_builder::build(&remote_address_2, &time_extent_zero);

        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
        println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

        //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
        //remote_address: [::]:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [99, 119, 230, 177, 20, 220, 163, 187]

        assert_ne!(cookie, cookie_2)
    }

    #[test]
    fn it_should_make_the_different_connection_cookie_for_different_socket() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let remote_address_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 1);
        let time_extent_zero = time_extent::ZERO;

        let cookie = cookie_builder::build(&remote_address, &time_extent_zero);
        let cookie_2 = cookie_builder::build(&remote_address_2, &time_extent_zero);

        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
        println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

        //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
        //remote_address: 0.0.0.0:1, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [38, 8, 0, 102, 92, 170, 220, 11]

        assert_ne!(cookie, cookie_2)
    }

    #[test]
    fn it_should_make_the_different_connection_cookie_for_different_time_extents() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let time_extent_zero = time_extent::ZERO;
        let time_extent_max = time_extent::MAX;

        let cookie = cookie_builder::build(&remote_address, &time_extent_zero);
        let cookie_2 = cookie_builder::build(&remote_address, &time_extent_max);

        println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
        println!("remote_address: {remote_address:?}, time_extent: {time_extent_max:?}, cookie: {cookie_2:?}");

        //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
        //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 18446744073709551615.999999999s, amount: 18446744073709551615 }, cookie: [87, 111, 109, 125, 182, 206, 3, 201]

        assert_ne!(cookie, cookie_2)
    }

    #[test]
    fn it_should_make_different_cookies_for_the_next_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make_connection_cookie(&remote_address);

        StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

        let cookie_next = make_connection_cookie(&remote_address);

        assert_ne!(cookie, cookie_next)
    }

    #[test]
    fn it_should_be_valid_for_this_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make_connection_cookie(&remote_address);

        check_connection_cookie(&remote_address, &cookie).unwrap();
    }

    #[test]
    fn it_should_be_valid_for_the_next_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make_connection_cookie(&remote_address);

        StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

        check_connection_cookie(&remote_address, &cookie).unwrap();
    }

    #[test]
    fn it_should_be_valid_for_the_last_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make_connection_cookie(&remote_address);

        StoppedClock::local_set(&COOKIE_LIFETIME.total().unwrap().unwrap());

        check_connection_cookie(&remote_address, &cookie).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_should_be_not_valid_after_their_last_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make_connection_cookie(&remote_address);

        StoppedClock::local_set(&COOKIE_LIFETIME.total_next().unwrap().unwrap());

        check_connection_cookie(&remote_address, &cookie).unwrap();
    }
}
