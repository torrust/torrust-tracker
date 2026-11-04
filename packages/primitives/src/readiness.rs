//! Application readiness.
// issue: #2697
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Whether the application should receive new traffic.
///
/// A new process is ready. The application marks it not ready when it begins
/// to shut down, before any component is asked to stop, and it never becomes
/// ready again: a restart creates a new lifecycle with a new `Readiness`.
///
/// Clones share one state, so the application keeps one handle to mark the
/// transition and gives another to every reader. Reading and marking are
/// lock-free and safe from any thread.
#[derive(Debug, Clone)]
pub struct Readiness {
    ready: Arc<AtomicBool>,
}

impl Default for Readiness {
    fn default() -> Self {
        Self {
            ready: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl Readiness {
    /// Whether the application is ready for new traffic.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Marks the application as not ready, for every clone.
    ///
    /// There is no way back: calling it again has no further effect.
    pub fn mark_not_ready(&self) {
        self.ready.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::Readiness;

    #[test]
    fn it_should_be_ready_by_default() {
        // Arrange
        let readiness = Readiness::default();

        // Act
        let ready = readiness.is_ready();

        // Assert
        assert!(ready);
    }

    #[test]
    fn it_should_not_be_ready_after_it_is_marked_not_ready() {
        // Arrange
        let readiness = Readiness::default();

        // Act
        readiness.mark_not_ready();

        // Assert
        assert!(!readiness.is_ready());
    }

    #[test]
    fn it_should_stay_not_ready_when_it_is_marked_not_ready_again() {
        // Arrange
        let readiness = Readiness::default();
        readiness.mark_not_ready();

        // Act
        readiness.mark_not_ready();

        // Assert
        assert!(!readiness.is_ready());
    }

    #[test]
    fn it_should_share_its_state_between_clones() {
        // Arrange
        let readiness = Readiness::default();
        let reader = readiness.clone();

        // Act
        readiness.mark_not_ready();

        // Assert
        assert!(!reader.is_ready());
    }

    #[test]
    fn it_should_be_not_ready_for_every_concurrent_reader_after_it_is_marked() {
        // Arrange
        let readiness = Readiness::default();
        let readers: Vec<_> = (0..8)
            .map(|_| {
                let reader = readiness.clone();
                std::thread::spawn(move || {
                    while reader.is_ready() {
                        std::thread::yield_now();
                    }
                })
            })
            .collect();

        // Act
        readiness.mark_not_ready();

        // Assert
        for reader in readers {
            reader.join().expect("a reader should observe the not-ready state and finish");
        }
    }
}
