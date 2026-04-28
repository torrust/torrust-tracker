use std::time::Duration;

/// The sleep duration between successive retries in a polling loop.
///
/// Wraps a [`Duration`]. Distinct from [`Deadline`] so that the two cannot
/// be accidentally swapped at a call site.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PollInterval(Duration);

impl PollInterval {
    /// Creates a new [`PollInterval`] from a [`Duration`].
    pub(crate) fn new(duration: Duration) -> Self {
        Self(duration)
    }

    /// Returns the underlying [`Duration`].
    pub(crate) fn as_duration(&self) -> Duration {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::PollInterval;

    #[test]
    fn it_should_round_trip_duration() {
        let duration = Duration::from_millis(750);
        let interval = PollInterval::new(duration);

        assert_eq!(interval.as_duration(), duration);
    }
}
