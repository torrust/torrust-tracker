use std::net::{AddrParseError, IpAddr};
use std::panic::Location;
use std::str::FromStr;

use thiserror::Error;
use torrust_tracker_located_error::{Located, LocatedError};

#[derive(Error, Debug)]
pub enum XForwardedForParseError {
    #[error("Empty X-Forwarded-For header value, {location}")]
    EmptyValue { location: &'static Location<'static> },

    #[error("Invalid IP in X-Forwarded-For header: {source}")]
    InvalidIp { source: LocatedError<'static, AddrParseError> },
}

impl From<AddrParseError> for XForwardedForParseError {
    #[track_caller]
    fn from(err: AddrParseError) -> Self {
        Self::InvalidIp {
            source: Located(err).into(),
        }
    }
}

/// It extracts the last IP address from the `X-Forwarded-For` http header value.
///
/// # Errors
///
/// Will return and error if the last IP in the `X-Forwarded-For` header is not a valid IP
pub fn maybe_rightmost_forwarded_ip(x_forwarded_for_value: &str) -> Result<IpAddr, XForwardedForParseError> {
    let mut x_forwarded_for_raw = x_forwarded_for_value.to_string();

    // Remove whitespace chars
    x_forwarded_for_raw.retain(|c| !c.is_whitespace());

    // Get all forwarded IP's in a vec
    let x_forwarded_ips: Vec<&str> = x_forwarded_for_raw.split(',').collect();

    match x_forwarded_ips.last() {
        Some(last_ip) => match IpAddr::from_str(last_ip) {
            Ok(ip) => Ok(ip),
            Err(err) => Err(err.into()),
        },
        None => Err(XForwardedForParseError::EmptyValue {
            location: Location::caller(),
        }),
    }
}

#[cfg(test)]
mod tests {

    use std::net::IpAddr;
    use std::str::FromStr;

    use super::maybe_rightmost_forwarded_ip;

    #[test]
    fn the_last_forwarded_ip_can_be_parsed_from_the_the_corresponding_http_header() {
        assert!(maybe_rightmost_forwarded_ip("").is_err());

        assert!(maybe_rightmost_forwarded_ip("INVALID IP").is_err());

        assert_eq!(
            maybe_rightmost_forwarded_ip("2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap(),
            IpAddr::from_str("2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap()
        );

        assert_eq!(
            maybe_rightmost_forwarded_ip("203.0.113.195").unwrap(),
            IpAddr::from_str("203.0.113.195").unwrap()
        );

        assert_eq!(
            maybe_rightmost_forwarded_ip("203.0.113.195, 2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap(),
            IpAddr::from_str("2001:db8:85a3:8d3:1319:8a2e:370:7348").unwrap()
        );

        assert_eq!(
            maybe_rightmost_forwarded_ip("203.0.113.195,2001:db8:85a3:8d3:1319:8a2e:370:7348,150.172.238.178").unwrap(),
            IpAddr::from_str("150.172.238.178").unwrap()
        );
    }
}
