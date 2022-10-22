use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

use aquatic_udp_protocol::ConnectionId;

use crate::protocol::clock::time_extent::{DefaultTimeExtentMaker, Extent, MakeTimeExtent, TimeExtent};
use crate::protocol::clock::{DefaultClock, Time, TimeNow};
use crate::protocol::crypto::keys::seeds::{DefaultSeed, SeedKeeper};
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

pub trait ConnectionCookie {
    fn make_connection_cookie(remote_address: &SocketAddr) -> Cookie;
    fn check_connection_cookie(
        remote_address: &SocketAddr,
        connection_cookie: &Cookie,
    ) -> Result<SinceUnixEpochTimeExtent, ServerError>;
}

pub struct HashedConnectionCookie;

impl HashedConnectionCookie {
    fn get_last_time_extent() -> SinceUnixEpochTimeExtent {
        DefaultTimeExtentMaker::now(&COOKIE_LIFETIME.increment)
            .unwrap()
            .unwrap()
            .increase(COOKIE_LIFETIME.amount)
            .unwrap()
    }

    fn build(remote_address: &SocketAddr, time_extent: &TimeExtent) -> Cookie {
        let seed = DefaultSeed::get_seed();

        let mut hasher = DefaultHasher::new();

        remote_address.hash(&mut hasher);
        time_extent.hash(&mut hasher);
        seed.hash(&mut hasher);

        hasher.finish().to_le_bytes()
    }
}

impl ConnectionCookie for HashedConnectionCookie {
    fn make_connection_cookie(remote_address: &SocketAddr) -> Cookie {
        let time_extent = HashedConnectionCookie::get_last_time_extent();

        HashedConnectionCookie::build(remote_address, &time_extent)
    }

    fn check_connection_cookie(
        remote_address: &SocketAddr,
        connection_cookie: &Cookie,
    ) -> Result<SinceUnixEpochTimeExtent, ServerError> {
        // we loop backwards testing each time_extent until we find one that matches.
        // (or the lifetime of time_extents is exhausted)
        for offset in 0..=COOKIE_LIFETIME.amount {
            let checking_time_extent = HashedConnectionCookie::get_last_time_extent().decrease(offset).unwrap();

            let checking_cookie = HashedConnectionCookie::build(remote_address, &checking_time_extent);
            //println!("remote_address: {remote_address:?}, time_extent: {checking_time_extent:?}, cookie: {checking_cookie:?}");

            if *connection_cookie == checking_cookie {
                return Ok(checking_time_extent);
            }
        }
        Err(ServerError::InvalidConnectionId)
    }
}

pub struct WitnessConnectionCookie;

impl ConnectionCookie for WitnessConnectionCookie {
    fn make_connection_cookie(remote_address: &SocketAddr) -> Cookie {
        // get the time that the cookie will expire.
        let expiry_time = DefaultClock::add(&COOKIE_LIFETIME.total().unwrap().unwrap())
            .unwrap()
            .as_secs_f32()
            .to_le_bytes();

        WitnessConnectionCookie::build(remote_address, expiry_time)
    }

    fn check_connection_cookie(
        remote_address: &SocketAddr,
        connection_cookie: &Cookie,
    ) -> Result<SinceUnixEpochTimeExtent, ServerError> {
        let expiry_time = WitnessConnectionCookie::extract_time(connection_cookie);
        let time_clock = DefaultClock::now().as_secs_f32();

        println!("expiry_time: {expiry_time:?}, time_clock: {time_clock:?}");

        // lets check if the cookie has expired.
        if expiry_time < time_clock {
            return Err(ServerError::ExpiredConnectionId);
        }

        if *connection_cookie != WitnessConnectionCookie::build(remote_address, expiry_time.to_le_bytes()) {
            return Err(ServerError::BadWitnessConnectionId);
        }

        Ok(TimeExtent::from_sec(1, &(expiry_time.round() as u64)))
    }
}

impl WitnessConnectionCookie {
    fn build(remote_address: &SocketAddr, expiry: [u8; 4]) -> Cookie {
        let seed = DefaultSeed::get_seed();

        let mut hasher = DefaultHasher::new();

        remote_address.hash(&mut hasher);
        seed.hash(&mut hasher);
        expiry.hash(&mut hasher);

        let witness = hasher.finish().to_le_bytes();

        [
            witness[0], witness[1], witness[2], witness[3], expiry[0], expiry[1], expiry[2], expiry[3],
        ]
    }
    fn extract_time(cookie: &Cookie) -> f32 {
        f32::from_le_bytes([cookie[4], cookie[5], cookie[6], cookie[7]])
    }
}

#[cfg(test)]
mod tests {
    mod hashed_connection_cookie {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use crate::protocol::clock::time_extent::Extent;
        use crate::protocol::clock::{StoppedClock, StoppedTime};
        use crate::udp::connection_cookie::{
            ConnectionCookie, Cookie, HashedConnectionCookie as TestConnectionCookie, COOKIE_LIFETIME,
        };

        #[test]
        fn it_should_make_a_connection_cookie() {
            let cookie = TestConnectionCookie::make_connection_cookie(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));

            // Note: This constant may need to be updated in the future as the hash is not guaranteed to to be stable between versions.
            const ID_COOKIE: Cookie = [23, 204, 198, 29, 48, 180, 62, 19];

            assert_eq!(cookie, ID_COOKIE)
        }

        #[test]
        fn it_should_make_different_cookies_for_the_next_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

            let cookie_next = TestConnectionCookie::make_connection_cookie(&remote_address);

            assert_ne!(cookie, cookie_next)
        }

        #[test]
        fn it_should_be_valid_for_this_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        fn it_should_be_valid_for_the_next_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        fn it_should_be_valid_for_the_last_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_set(&COOKIE_LIFETIME.total().unwrap().unwrap());

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        #[should_panic]
        fn it_should_be_not_valid_after_their_last_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_set(&COOKIE_LIFETIME.total_next().unwrap().unwrap());

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        mod detail {
            use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

            use crate::protocol::clock::time_extent;
            use crate::udp::connection_cookie::HashedConnectionCookie;

            #[test]
            fn it_should_build_the_same_connection_cookie_for_the_same_input_data() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let time_extent_zero = time_extent::ZERO;

                let cookie = HashedConnectionCookie::build(&remote_address, &time_extent_zero);
                let cookie_2 = HashedConnectionCookie::build(&remote_address, &time_extent_zero);

                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

                //remote_address: 127.0.0.1:8080, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [212, 9, 204, 223, 176, 190, 150, 153]
                //remote_address: 127.0.0.1:8080, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [212, 9, 204, 223, 176, 190, 150, 153]

                assert_eq!(cookie, cookie_2)
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_ip() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let remote_address_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 0);
                let time_extent_zero = time_extent::ZERO;

                let cookie = HashedConnectionCookie::build(&remote_address, &time_extent_zero);
                let cookie_2 = HashedConnectionCookie::build(&remote_address_2, &time_extent_zero);

                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
                println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

                //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
                //remote_address: 255.255.255.255:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [217, 87, 239, 178, 182, 126, 66, 166]

                assert_ne!(cookie, cookie_2)
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_ip_version() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let remote_address_2 = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0);
                let time_extent_zero = time_extent::ZERO;

                let cookie = HashedConnectionCookie::build(&remote_address, &time_extent_zero);
                let cookie_2 = HashedConnectionCookie::build(&remote_address_2, &time_extent_zero);

                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
                println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

                //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
                //remote_address: [::]:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [99, 119, 230, 177, 20, 220, 163, 187]

                assert_ne!(cookie, cookie_2)
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_socket() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let remote_address_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 1);
                let time_extent_zero = time_extent::ZERO;

                let cookie = HashedConnectionCookie::build(&remote_address, &time_extent_zero);
                let cookie_2 = HashedConnectionCookie::build(&remote_address_2, &time_extent_zero);

                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
                println!("remote_address: {remote_address_2:?}, time_extent: {time_extent_zero:?}, cookie: {cookie_2:?}");

                //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
                //remote_address: 0.0.0.0:1, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [38, 8, 0, 102, 92, 170, 220, 11]

                assert_ne!(cookie, cookie_2)
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_time_extents() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let time_extent_zero = time_extent::ZERO;
                let time_extent_max = time_extent::MAX;

                let cookie = HashedConnectionCookie::build(&remote_address, &time_extent_zero);
                let cookie_2 = HashedConnectionCookie::build(&remote_address, &time_extent_max);

                println!("remote_address: {remote_address:?}, time_extent: {time_extent_zero:?}, cookie: {cookie:?}");
                println!("remote_address: {remote_address:?}, time_extent: {time_extent_max:?}, cookie: {cookie_2:?}");

                //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 0ns, amount: 0 }, cookie: [151, 130, 30, 157, 190, 41, 179, 135]
                //remote_address: 0.0.0.0:0, time_extent: TimeExtent { increment: 18446744073709551615.999999999s, amount: 18446744073709551615 }, cookie: [87, 111, 109, 125, 182, 206, 3, 201]

                assert_ne!(cookie, cookie_2)
            }
        }
    }
    mod witness_connection_cookie {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        use crate::protocol::clock::time_extent::Extent;
        use crate::protocol::clock::{StoppedClock, StoppedTime};
        use crate::udp::connection_cookie::{
            ConnectionCookie, Cookie, WitnessConnectionCookie as TestConnectionCookie, COOKIE_LIFETIME,
        };

        #[test]
        fn it_should_make_a_connection_cookie() {
            let cookie = TestConnectionCookie::make_connection_cookie(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0));

            // Note: This constant may need to be updated in the future as the hash is not guaranteed to to be stable between versions.
            const ID_COOKIE: Cookie = [72, 245, 201, 136, 0, 0, 240, 66];

            assert_eq!(cookie, ID_COOKIE)
        }

        #[test]
        fn it_should_make_different_cookies_for_the_next_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

            let cookie_next = TestConnectionCookie::make_connection_cookie(&remote_address);

            assert_ne!(cookie, cookie_next)
        }

        #[test]
        fn it_should_be_valid_for_this_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        fn it_should_be_valid_for_the_next_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_add(&COOKIE_LIFETIME.increment).unwrap();

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        fn it_should_be_valid_for_the_last_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_set(&COOKIE_LIFETIME.total().unwrap().unwrap());

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        #[test]
        #[should_panic]
        fn it_should_be_not_valid_after_their_last_time_extent() {
            let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);

            let cookie = TestConnectionCookie::make_connection_cookie(&remote_address);

            StoppedClock::local_set(&COOKIE_LIFETIME.total_next().unwrap().unwrap());

            TestConnectionCookie::check_connection_cookie(&remote_address, &cookie).unwrap();
        }

        mod detail {
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};

            use crate::udp::connection_cookie::WitnessConnectionCookie;

            #[test]
            fn it_should_build_the_same_connection_cookie_for_the_same_input_data() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let expiry_zero = [0u8; 4];

                let cookie = WitnessConnectionCookie::build(&remote_address, expiry_zero);
                let cookie_2 = WitnessConnectionCookie::build(&remote_address, expiry_zero);

                // witness is the same, and the times are the same.
                assert_eq!(cookie[0..4], cookie_2[0..4]);
                assert_eq!(cookie[5..8], cookie_2[5..8]);
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_ip() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let remote_address_2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), 0);
                let expiry_zero = [0u8; 4];

                let cookie = WitnessConnectionCookie::build(&remote_address, expiry_zero);
                let cookie_2 = WitnessConnectionCookie::build(&remote_address_2, expiry_zero);

                // witness is different, but the times are the same.
                assert_ne!(cookie[0..4], cookie_2[0..4]);
                assert_eq!(cookie[5..8], cookie_2[5..8]);
            }

            #[test]
            fn it_should_build_the_different_connection_cookie_for_different_expires() {
                let remote_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
                let expiry_zero = [0u8; 4];
                let expiry_max = [255u8; 4];

                let cookie = WitnessConnectionCookie::build(&remote_address, expiry_zero);
                let cookie_2 = WitnessConnectionCookie::build(&remote_address, expiry_max);

                // witness is different, and the times are different.
                assert_ne!(cookie[0..4], cookie_2[0..4]);
                assert_ne!(cookie[5..8], cookie_2[5..8]);
            }
        }
    }
}
