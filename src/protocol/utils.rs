/// Serializer for `std::time::Instant` type.
/// Before serializing, it converts the instant to time elapse since that instant in milliseconds.
///
/// You can use it like this:
///
/// ```text
/// #[serde(serialize_with = "ser_instant")]
/// pub updated: std::time::Instant,
/// ```
///
pub fn ser_instant<S: serde::Serializer>(inst: &std::time::Instant, ser: S) -> Result<S::Ok, S::Error> {
    ser.serialize_u64(inst.elapsed().as_millis() as u64)
}

#[cfg(test)]
mod tests {
    use std::{time::Instant};
    use serde::Serialize;

    #[warn(unused_imports)]
    use super::ser_instant;

    #[derive(PartialEq, Eq, Debug, Clone, Serialize)]
    struct S {
        #[serde(serialize_with = "ser_instant")]
        pub time: Instant,
    }

    #[test]
    fn instant_types_can_be_serialized_as_elapsed_time_since_that_instant_in_milliseconds() {

        use std::{thread, time};

        let t1 = Instant::now();

        let s = S { time: t1 };

        // Sleep 10 milliseconds
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);

        let json_serialized_value = serde_json::to_string(&s).unwrap();

        // Json contains time duration since t1 instant in milliseconds
        assert_eq!(json_serialized_value, r#"{"time":10}"#);
    }
}
