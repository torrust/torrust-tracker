use super::clock::DurationSinceUnixEpoch;

pub fn ser_unix_time_value<S: serde::Serializer>(unix_time_value: &DurationSinceUnixEpoch, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(unix_time_value.as_millis() as u64)
}
