use std::time::Duration;

/// A polling-loop deadline expressed as a [`Duration`] measured from the moment
/// the loop starts.
///
/// Wraps a [`Duration`] representing the *maximum time* a polling loop may wait
/// before giving up. Keeping it distinct from [`PollInterval`] turns an
/// accidental swap into a compile error instead of a silent logic bug.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Deadline(Duration);

impl Deadline {
    /// Creates a new [`Deadline`] from a [`Duration`].
    pub(crate) const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) const fn as_duration(&self) -> Duration {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Deadline;

    #[test]
    fn it_should_round_trip_duration() {
        let duration = Duration::from_secs(42);
        let deadline = Deadline::new(duration);

        assert_eq!(deadline.as_duration(), duration);
    }
}
