//! Logic for generating and verifying connection IDs.
//!
//! The UDP tracker requires the client to connect to the server before it can
//! send any data. The server responds with a random 64-bit integer that the
//! client must use to identify itself.
//!
//! This connection ID is used to avoid spoofing attacks. The client must send
//! the connection ID in all requests to the server. The server will ignore any
//! requests that do not contain the correct connection ID.
//!
//! The simplest way to implement this would be to generate a random number when
//! the client connects and store it in a hash table. However, this would
//! require the server to store a large number of connection IDs, which would be
//! a waste of memory. Instead, the server generates a connection ID based on
//! the client's IP address and the current time. This allows the server to
//! verify the connection ID without storing it.
//!
//! This module implements this method of generating connection IDs. It's the
//! most common way to generate connection IDs. The connection ID is generated
//! using a time based algorithm and it is valid for a certain amount of time
//! (usually two minutes). The connection ID is generated using the following:
//!
//! ```text
//! connection ID = hash(client IP + current time slot + secret seed)
//! ```
//!
//! Time slots are two minute intervals since the Unix epoch. The secret seed is
//! a random number that is generated when the server starts. And the client IP
//! is used in order generate a unique connection ID for each client.
//!
//! The BEP-15 recommends a two-minute time slot.
//!
//! ```text
//! Timestamp (seconds from Unix epoch):
//! |------------|------------|------------|------------|
//! 0            120          240          360          480
//! Time slots (two-minutes time extents from Unix epoch):
//! |------------|------------|------------|------------|
//! 0            1            2            3            4
//! Peer connections:
//! Peer A       |-------------------------|
//! Peer B                    |-------------------------|
//! Peer C              |------------------|
//! Peer A connects at timestamp 120 slot 1 -> connection ID will be valid from timestamp 120 to 360
//! Peer B connects at timestamp 240 slot 2 -> connection ID will be valid from timestamp 240 to 480
//! Peer C connects at timestamp 180 slot 1 -> connection ID will be valid from timestamp 180 to 360
//! ```
//! > **NOTICE**: connection ID is always the same for a given peer
//! (socket address) and time slot.
//!
//! > **NOTICE**: connection ID will be valid for two time extents, **not two
//! minutes**. It'll be valid for the the current time extent and the next one.
//!
//! Refer to [`Connect`](crate::servers::udp#connect) for more information about
//! the connection process.
//!
//! ## Advantages
//!
//! - It consumes less memory than storing a hash table of connection IDs.
//! - It's easy to implement.
//! - It's fast.
//!
//! ## Disadvantages
//!
//! - It's not very flexible. The connection ID is only valid for a certain
//! amount of time.
//! - It's not very accurate. The connection ID is valid for more than two
//! minutes.
use std::net::SocketAddr;
use std::panic::Location;

use aquatic_udp_protocol::ConnectionId;
use torrust_tracker_clock::time_extent::{Extent, TimeExtent};
use zerocopy::network_endian::I64;
use zerocopy::AsBytes;

use super::error::Error;

pub type Cookie = [u8; 8];

pub type SinceUnixEpochTimeExtent = TimeExtent;

pub const COOKIE_LIFETIME: TimeExtent = TimeExtent::from_sec(2, &60);

/// Converts a connection ID into a connection cookie.
#[must_use]
pub fn from_connection_id(connection_id: &ConnectionId) -> Cookie {
    let mut cookie = [0u8; 8];
    connection_id.write_to(&mut cookie);
    cookie
}

/// Converts a connection cookie into a connection ID.
#[must_use]
pub fn into_connection_id(connection_cookie: &Cookie) -> ConnectionId {
    ConnectionId(I64::new(i64::from_be_bytes(*connection_cookie)))
}

/// Generates a new connection cookie.
#[must_use]
pub fn make(remote_address: &SocketAddr) -> Cookie {
    let time_extent = cookie_builder::get_last_time_extent();

    //println!("remote_address: {remote_address:?}, time_extent: {time_extent:?}, cookie: {cookie:?}");
    cookie_builder::build(remote_address, &time_extent)
}

/// Checks if the supplied `connection_cookie` is valid.
///
/// # Panics
///
/// It would panic if the `COOKIE_LIFETIME` constant would be an unreasonably large number.
///
/// # Errors
///
/// Will return a `ServerError::InvalidConnectionId` if the supplied `connection_cookie` fails to verify.
pub fn check(remote_address: &SocketAddr, connection_cookie: &Cookie) -> Result<SinceUnixEpochTimeExtent, Error> {
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
    Err(Error::InvalidConnectionId {
        location: Location::caller(),
    })
}

mod cookie_builder {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::net::SocketAddr;

    use torrust_tracker_clock::time_extent::{Extent, Make, TimeExtent};

    use super::{Cookie, SinceUnixEpochTimeExtent, COOKIE_LIFETIME};
    use crate::shared::crypto::keys::seeds::{Current, Keeper};
    use crate::DefaultTimeExtentMaker;

    pub(super) fn get_last_time_extent() -> SinceUnixEpochTimeExtent {
        DefaultTimeExtentMaker::now(&COOKIE_LIFETIME.increment)
            .unwrap()
            .unwrap()
            .increase(COOKIE_LIFETIME.amount)
            .unwrap()
    }

    pub(super) fn build(remote_address: &SocketAddr, time_extent: &TimeExtent) -> Cookie {
        let seed = Current::get_seed();

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

    use torrust_tracker_clock::clock::stopped::Stopped as _;
    use torrust_tracker_clock::clock::{self};
    use torrust_tracker_clock::time_extent::{self, Extent};

    use super::cookie_builder::{self};
    use crate::servers::udp::v0::cookie::{check, make, Cookie, COOKIE_LIFETIME};

    // #![feature(const_socketaddr)]
    // const REMOTE_ADDRESS_IPV4_ZERO: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

    #[test]
    fn it_should_make_a_connection_cookie() {
        // Note: This constant may need to be updated in the future as the hash is not guaranteed to to be stable between versions.
        const ID_COOKIE_OLD: Cookie = [23, 204, 198, 29, 48, 180, 62, 19];
        const ID_COOKIE_NEW: Cookie = [41, 166, 45, 246, 249, 24, 108, 203];

        clock::Stopped::local_set_to_unix_epoch();

        let cookie = make(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));

        assert!(cookie == ID_COOKIE_OLD || cookie == ID_COOKIE_NEW);
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

        assert_eq!(cookie, cookie_2);
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

        assert_ne!(cookie, cookie_2);
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

        assert_ne!(cookie, cookie_2);
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

        assert_ne!(cookie, cookie_2);
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

        assert_ne!(cookie, cookie_2);
    }

    #[test]
    fn it_should_make_different_cookies_for_the_next_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make(&remote_address);

        clock::Stopped::local_add(&COOKIE_LIFETIME.increment).unwrap();

        let cookie_next = make(&remote_address);

        assert_ne!(cookie, cookie_next);
    }

    #[test]
    fn it_should_be_valid_for_this_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make(&remote_address);

        check(&remote_address, &cookie).unwrap();
    }

    #[test]
    fn it_should_be_valid_for_the_next_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make(&remote_address);

        clock::Stopped::local_add(&COOKIE_LIFETIME.increment).unwrap();

        check(&remote_address, &cookie).unwrap();
    }

    #[test]
    fn it_should_be_valid_for_the_last_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        clock::Stopped::local_set_to_unix_epoch();

        let cookie = make(&remote_address);

        clock::Stopped::local_set(&COOKIE_LIFETIME.total().unwrap().unwrap());

        check(&remote_address, &cookie).unwrap();
    }

    #[test]
    #[should_panic = "InvalidConnectionId"]
    fn it_should_be_not_valid_after_their_last_time_extent() {
        let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

        let cookie = make(&remote_address);

        clock::Stopped::local_set(&COOKIE_LIFETIME.total_next().unwrap().unwrap());

        check(&remote_address, &cookie).unwrap();
    }
}
