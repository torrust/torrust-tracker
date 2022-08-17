//! A secret hash value bound to time.
//! It changes every two minutes starting at Unix Epoch.
//!
//! | Date                  | Timestamp | Unix Epoch in minutes | TimeBoundPepper |
//! |-----------------------|-----------|-----------------------|-----------------|
//! | 1/1/1970, 12:00:00 AM | 0         | minute 0              | X               |
//! | 1/1/1970, 12:01:00 AM | 60        | minute 1              | X               |
//! | 1/1/1970, 12:02:00 AM | 120       | minute 2              | Y = X           |
//! | 1/1/1970, 12:03:00 AM | 180       | minute 3              | Y = X           |
//! | 1/1/1970, 12:04:00 AM | 240       | minute 4              | Z != X          |
//! | 1/1/1970, 12:05:00 AM | 300       | minute 5              | Z != X          |

use super::byte_array_32::ByteArray32;

pub type Timestamp = u64;

#[derive(PartialEq, Debug)]
pub struct TimeBoundPepper {
    pepper: ByteArray32,
}

impl TimeBoundPepper {
    pub fn new(server_secret: &ByteArray32, current_timestamp: Timestamp) -> Self {
        Self {
            pepper: Self::generate_pepper(server_secret, current_timestamp)
        }
    }

    /// Time_Bound_Pepper = Hash(Server_Secret || Unix_Time_Minutes / 2)
    fn generate_pepper(server_secret: &ByteArray32, current_timestamp: Timestamp) -> ByteArray32 {

        let unix_time_minutes: u64 = current_timestamp / 60;

        // Server Secret | Unix_Time_Minutes / 2
        let server_secret_or_two_minute_counter = *server_secret | ByteArray32::from(unix_time_minutes / 2);

        // Hash(Server Secret | Unix_Time_Minutes / 2)
        let time_bound_pepper = blake3::hash(&server_secret_or_two_minute_counter.as_generic_byte_array());

        ByteArray32::new(*time_bound_pepper.as_bytes())
    }

    pub fn get_pepper(&self) -> &ByteArray32 {
        &self.pepper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_server_secret_for_testing() -> ByteArray32 {
        ByteArray32::new([0u8;32])
    }

    #[test]
    fn it_should_be_the_same_during_each_two_minute_window_since_unix_epoch() {

        let server_secret = generate_server_secret_for_testing();

        let initial_timestamp = 0u64;

        let time_bound_pepper = TimeBoundPepper::new(&server_secret, initial_timestamp);
        let time_bound_pepper_after_two_minutes = TimeBoundPepper::new(&server_secret, initial_timestamp + 120 - 1);

        assert_eq!(time_bound_pepper_after_two_minutes, time_bound_pepper);
    }

    #[test]
    fn it_should_change_after_a_two_minute_window_starting_time_windows_at_unix_epoch() {

        let server_secret = generate_server_secret_for_testing();

        let initial_timestamp = 0u64;

        let time_bound_pepper = TimeBoundPepper::new(&server_secret, initial_timestamp);
        let time_bound_pepper_after_two_minutes = TimeBoundPepper::new(&server_secret, initial_timestamp + 120 - 1);

        assert_eq!(time_bound_pepper_after_two_minutes, time_bound_pepper);
    }

}