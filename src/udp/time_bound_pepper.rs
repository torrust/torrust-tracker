use super::byte_array_32::ByteArray32;

pub fn generate_time_bound_pepper(server_secret: &ByteArray32, current_timestamp: u64) -> ByteArray32 {

    // todo: use a TimeBoundPepper struct as proposed by @da2ce7, storing also the current timestamp
    // used to generate the TimeBoundPepper
    //
    // #[derive(Default)]
    // struct TimeBoundPepper {
    //    created: SystemTime,
    //    pepper: [u8; 32],
    // }
    //
    // impl Default for TimeBoundPepper {
    //     fn default() -> Self { SystemTime::now(), pepper: hash(SERVER_SECRET || self.created ) };
    // }
    //
    // impl TimeBoundPepper {
    //   pub fn new() ->  Default::default();
    //
    //   pub fn get_pepper(&mut self, expires: std::time::Duration) -> [u8; 32] {
    //     if (created.elapsed().unwrap() >= expires) self = self.new();
    //     return self.pepper;
    // }

    // Time_Bound_Pepper = Hash(Static_Secret || Unix_Time_Minutes / 2)
    // (32-bytes), cached, expires every two minutes.

    let unix_time_minutes: u64 = current_timestamp / 60;

    // Server Secret | Unix_Time_Minutes / 2
    let server_secret_or_two_minute_counter = *server_secret | ByteArray32::from(unix_time_minutes / 2);

    // Hash(Server Secret | Unix_Time_Minutes / 2)
    let time_bound_pepper = blake3::hash(&server_secret_or_two_minute_counter.as_generic_byte_array());

    ByteArray32::new(*time_bound_pepper.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_server_secret_for_testing() -> ByteArray32 {
        ByteArray32::new([0u8;32])
    }

    #[test]
    fn time_bound_pepper_should_be_the_same_during_each_two_minute_window_since_unix_epoch() {

        let server_secret = generate_server_secret_for_testing();

        // | Date                  | Timestamp | Unix Epoch in minutes | Connection IDs |
        // |----------------------------------------------------------------------------|
        // | 1/1/1970, 12:00:00 AM | 0         | minute 0              | X              |
        // | 1/1/1970, 12:01:00 AM | 60        | minute 1              | X              |
        // | 1/1/1970, 12:02:00 AM | 120       | minute 2              | Y = X          |
        // | 1/1/1970, 12:03:00 AM | 180       | minute 3              | Y = X          |
        // | 1/1/1970, 12:04:00 AM | 240       | minute 4              | Z != X         |
        // | 1/1/1970, 12:05:00 AM | 300       | minute 5              | Z != X         |

        let initial_timestamp = 0u64;

        let time_bound_pepper = generate_time_bound_pepper(&server_secret, initial_timestamp);

        assert_eq!(time_bound_pepper, ByteArray32::new([42, 218, 131, 193, 129, 154, 83, 114, 218, 225, 35, 143, 193, 222, 209, 35, 200, 16, 79, 218, 161, 88, 98, 170, 238, 105, 66, 138, 24, 32, 252, 218]));

        let time_bound_pepper_after_two_minutes = generate_time_bound_pepper(&server_secret, initial_timestamp + 120 - 1);

        assert_eq!(time_bound_pepper_after_two_minutes, time_bound_pepper);
    }

    #[test]
    fn time_bound_pepper_should_change_after_a_two_minute_window_starting_time_windows_at_unix_epoch() {

        let server_secret = generate_server_secret_for_testing();

        // | Date                  | Timestamp | Unix Epoch in minutes | Connection IDs |
        // |----------------------------------------------------------------------------|
        // | 1/1/1970, 12:00:00 AM | 0         | minute 0              | X              |
        // | 1/1/1970, 12:01:00 AM | 60        | minute 1              | X              |
        // | 1/1/1970, 12:02:00 AM | 120       | minute 2              | Y = X          |
        // | 1/1/1970, 12:03:00 AM | 180       | minute 3              | Y = X          |
        // | 1/1/1970, 12:04:00 AM | 240       | minute 4              | Z != X         |
        // | 1/1/1970, 12:05:00 AM | 300       | minute 5              | Z != X         |

        let initial_timestamp = 0u64;

        let time_bound_pepper = generate_time_bound_pepper(&server_secret, initial_timestamp);

        assert_eq!(time_bound_pepper, ByteArray32::new([42, 218, 131, 193, 129, 154, 83, 114, 218, 225, 35, 143, 193, 222, 209, 35, 200, 16, 79, 218, 161, 88, 98, 170, 238, 105, 66, 138, 24, 32, 252, 218]));

        let time_bound_pepper_after_two_minutes = generate_time_bound_pepper(&server_secret, initial_timestamp + 120 - 1);

        assert_eq!(time_bound_pepper_after_two_minutes, time_bound_pepper);
    }

}