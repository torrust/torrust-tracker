//! The drain every event listener runs after it is cancelled.
//!
//! A cancelled listener must not drop the events already queued for it. [`drain`]
//! takes them with [`Receiver::try_recv`] and hands each one to the listener's
//! handler, until the queue is empty or the drain timeout expires.
//!
//! The timeout is an *admission* budget: it is checked only between events. An event
//! that was taken is always handled to completion, so the timeout is not a hard bound
//! on a handler. The shared shutdown deadline, which aborts the listener task, stays
//! the final bound. After each handled event the drain yields to the runtime, so a long drain does
//! not starve the other tasks that are shutting down.
use std::future::Future;
use std::time::Duration;

use tokio::task::yield_now;
use tokio::time::Instant;

use crate::receiver::{Receiver, TryRecvError};

/// The drain timeout the application gives every listener until it becomes configuration (SI-20).
///
/// It bounds how long a cancelled listener keeps *admitting* queued events, not how long a handler
/// may run. It is deliberately below the 10-second shared shutdown deadline: a full buffer of 65,536
/// events takes about 12 to 14 s to persist on tmpfs and about 158 to 188 s on an `NVMe` disk with
/// SQLite (issue #2414, `manual-verification-evidence.md` V6), so no timeout that fits the deadline
/// drains a full buffer, and a larger backlog is reported by the drain's warning.
pub const DEFAULT_DRAIN_TIMEOUT: Duration = Duration::from_secs(5);

/// Why a [`drain`] stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrainOutcome {
    /// The receiver had no more unread events.
    Emptied,

    /// The receiver had read every buffered event and no sender is left.
    Closed,

    /// The drain timeout expired while events were still unread.
    TimedOut,
}

impl DrainOutcome {
    /// A stable name for logs and metrics, independent of the `Debug` representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Emptied => "emptied",
            Self::Closed => "closed",
            Self::TimedOut => "timed_out",
        }
    }
}

/// What a [`drain`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrainReport {
    pub outcome: DrainOutcome,

    /// Events handed to the handler, each handled to completion.
    pub processed: u64,

    /// Events the receiver reported as skipped (`Lagged`) during the drain.
    pub lost_to_lag: u64,

    /// [`Receiver::len`] when the timeout expired, and `0` otherwise.
    ///
    /// It is an unread-position count, not a count of recoverable events: it includes
    /// events that sends overwrote and the receiver has not yet reported as lagged. Do
    /// not add it to `lost_to_lag` as if the two were disjoint totals; a lag that the
    /// drain observed has already advanced the receiver, so it is not in `len()`.
    pub unread_when_timed_out: usize,
}

/// Handles every event already queued for `receiver`, within an admission `timeout`.
///
/// The timeout starts when this function is called and is checked before each event
/// is taken. An event that was taken is awaited without racing it against the
/// timeout. `Lagged` results are counted and the drain continues.
pub async fn drain<R, H, Fut>(receiver: &mut R, timeout: Duration, mut handle: H) -> DrainReport
where
    R: Receiver + ?Sized,
    H: FnMut(R::Event) -> Fut,
    Fut: Future<Output = ()>,
{
    // A timeout too large for the clock to represent never expires.
    let deadline = Instant::now().checked_add(timeout);
    let mut processed = 0;
    let mut lost_to_lag = 0;

    loop {
        if deadline.is_some_and(|deadline| Instant::now() >= deadline) {
            let unread = receiver.len();

            return DrainReport {
                outcome: if unread == 0 {
                    DrainOutcome::Emptied
                } else {
                    DrainOutcome::TimedOut
                },
                processed,
                lost_to_lag,
                unread_when_timed_out: unread,
            };
        }

        match receiver.try_recv() {
            Ok(event) => {
                handle(event).await;
                processed += 1;

                // A handler that never waits would otherwise hold the worker thread until the timeout
                // expires, starving the other tasks that are shutting down.
                yield_now().await;
            }
            Err(TryRecvError::Lagged(skipped)) => lost_to_lag += skipped,
            Err(TryRecvError::Empty) => return report(DrainOutcome::Emptied, processed, lost_to_lag),
            Err(TryRecvError::Closed) => return report(DrainOutcome::Closed, processed, lost_to_lag),
        }
    }
}

/// Logs what a drain did, with the same fields for every listener.
///
/// A drain that lost events (`TimedOut`, or `lost_to_lag > 0`) logs a warning that names the
/// listener and the counts; any other drain logs an info line. It is a macro and not a function
/// because `tracing` needs a constant `target`, and each listener's package has its own.
///
/// ```ignore
/// torrust_tracker_events::log_drain_report!(MY_LOG_TARGET, "my listener", report);
/// ```
#[macro_export]
macro_rules! log_drain_report {
    ($target:expr, $listener:expr, $report:expr) => {{
        let report: $crate::drain::DrainReport = $report;

        if report.outcome == $crate::drain::DrainOutcome::TimedOut || report.lost_to_lag > 0 {
            tracing::warn!(
                target: $target,
                listener = $listener,
                outcome = report.outcome.as_str(),
                processed = report.processed,
                lost_to_lag = report.lost_to_lag,
                unread_when_timed_out = report.unread_when_timed_out,
                "Event listener drain lost events after cancellation"
            );
        } else {
            tracing::info!(
                target: $target,
                listener = $listener,
                outcome = report.outcome.as_str(),
                processed = report.processed,
                lost_to_lag = report.lost_to_lag,
                "Event listener drained its queue after cancellation"
            );
        }
    }};
}

const fn report(outcome: DrainOutcome, processed: u64, lost_to_lag: u64) -> DrainReport {
    DrainReport {
        outcome,
        processed,
        lost_to_lag,
        unread_when_timed_out: 0,
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    use futures::future::BoxFuture;
    use tokio::sync::broadcast;
    use tokio::time::sleep;

    use super::{DrainOutcome, DrainReport, drain};
    use crate::receiver::{Receiver, RecvError, TryRecvError};

    /// A receiver that returns the scripted results in order and `Empty` after them.
    struct ScriptedReceiver {
        results: VecDeque<Result<u32, TryRecvError>>,
    }

    impl ScriptedReceiver {
        fn new(results: impl IntoIterator<Item = Result<u32, TryRecvError>>) -> Self {
            Self {
                results: results.into_iter().collect(),
            }
        }
    }

    impl Receiver for ScriptedReceiver {
        type Event = u32;

        fn recv(&mut self) -> BoxFuture<'_, Result<Self::Event, RecvError>> {
            unreachable!("a drain must not wait for events");
        }

        fn try_recv(&mut self) -> Result<Self::Event, TryRecvError> {
            self.results.pop_front().unwrap_or(Err(TryRecvError::Empty))
        }

        fn len(&self) -> usize {
            self.results.iter().filter(|result| result.is_ok()).count()
        }
    }

    #[tokio::test]
    async fn it_should_report_an_emptied_receiver_when_there_is_nothing_to_drain() {
        // Arrange
        // A receiver with no unread events, and a handler that records what it is given.
        let mut receiver = ScriptedReceiver::new([]);
        let handled = RefCell::new(Vec::new());

        // Act
        let report = drain(&mut receiver, Duration::from_secs(1), |event| {
            handled.borrow_mut().push(event);
            async {}
        })
        .await;

        // Assert
        // The drain ends as emptied, and the handler was never called.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::Emptied,
                processed: 0,
                lost_to_lag: 0,
                unread_when_timed_out: 0,
            }
        );
        assert!(handled.borrow().is_empty(), "the handler must not run when nothing is queued");
    }

    #[tokio::test]
    async fn it_should_handle_every_buffered_event_then_report_closed_when_the_senders_are_gone() {
        // Arrange
        // Three events are buffered in a real broadcast channel, and every sender is dropped
        // before the drain starts, as when the application has shut down its producers.
        let (sender, mut receiver) = broadcast::channel(8);
        for event in [10_u32, 20, 30] {
            sender.send(event).unwrap();
        }
        drop(sender);
        let handled = RefCell::new(Vec::new());

        // Act
        let report = drain(&mut receiver, Duration::from_secs(1), |event| {
            handled.borrow_mut().push(event);
            async {}
        })
        .await;

        // Assert
        // Closing the channel does not discard the buffered events: all three are handled, in
        // order, and only then does the drain see the channel closed.
        assert_eq!(
            *handled.borrow(),
            [10, 20, 30],
            "buffered events handled before the channel reports closed"
        );
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::Closed,
                processed: 3,
                lost_to_lag: 0,
                unread_when_timed_out: 0,
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_stop_admitting_events_and_report_how_many_are_left_when_the_timeout_expires() {
        // Arrange
        // Five events are queued and each takes 10 ms to handle, so with a 25 ms timeout the
        // drain admits events at 0, 10, and 20 ms and finds the timeout expired at 30 ms.
        // Tokio time is paused, and the handler's sleep advances it.
        let mut receiver = ScriptedReceiver::new([Ok(1), Ok(2), Ok(3), Ok(4), Ok(5)]);

        // Act
        let report = drain(&mut receiver, Duration::from_millis(25), |_event| {
            sleep(Duration::from_millis(10))
        })
        .await;

        // Assert
        // Three events were handled and two are left unread.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::TimedOut,
                processed: 3,
                lost_to_lag: 0,
                unread_when_timed_out: 2,
            }
        );
    }

    #[tokio::test]
    async fn it_should_count_lagged_events_as_lost_and_keep_draining() {
        // Arrange
        // The receiver reports 4 skipped events, then has two events to read.
        let mut receiver = ScriptedReceiver::new([Err(TryRecvError::Lagged(4)), Ok(1), Ok(2)]);

        // Act
        let report = drain(&mut receiver, Duration::from_secs(1), |_event| async {}).await;

        // Assert
        // The skipped events are reported as lost, and the two readable events are handled.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::Emptied,
                processed: 2,
                lost_to_lag: 4,
                unread_when_timed_out: 0,
            }
        );
    }

    #[tokio::test]
    async fn it_should_include_events_overwritten_before_the_first_lag_result_in_the_unread_count_at_timeout() {
        // Arrange
        // A real broadcast channel holds two events, and five are sent, so three were
        // overwritten and only two can still be read. The receiver has not yet been told
        // about the lag. The timeout is zero, so it expires before the first event is taken.
        let (sender, mut receiver) = broadcast::channel(2);
        for event in 1_u32..=5 {
            sender.send(event).unwrap();
        }

        // Act
        let report = drain(&mut receiver, Duration::ZERO, |_event| async {}).await;

        // Assert
        // The unread count is 5, the five positions sent and not received, although only two
        // events are recoverable. No lag was observed, so none is counted as lost.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::TimedOut,
                processed: 0,
                lost_to_lag: 0,
                unread_when_timed_out: 5,
            }
        );
    }

    #[tokio::test(start_paused = true)]
    async fn it_should_finish_a_slow_handler_that_crosses_the_timeout_before_stopping() {
        // Arrange
        // Three events are queued, the timeout is 50 ms, and the handler takes 100 ms, so the
        // timeout expires while the first event is being handled.
        let mut receiver = ScriptedReceiver::new([Ok(1), Ok(2), Ok(3)]);
        let completed = RefCell::new(Vec::new());
        let completed_ref = &completed;

        // Act
        let report = drain(&mut receiver, Duration::from_millis(50), move |event| async move {
            sleep(Duration::from_millis(100)).await;
            completed_ref.borrow_mut().push(event);
        })
        .await;

        // Assert
        // The first event was handled to completion, although the timeout expired during it.
        // The drain then stopped, leaving the other two unread.
        assert_eq!(
            *completed.borrow(),
            [1],
            "the timeout must not interrupt a handler that is running"
        );
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::TimedOut,
                processed: 1,
                lost_to_lag: 0,
                unread_when_timed_out: 2,
            }
        );
    }

    #[tokio::test]
    async fn it_should_report_an_emptied_receiver_and_not_a_timeout_when_nothing_is_left_unread_at_the_deadline() {
        // Arrange
        // A receiver with no unread events, and a timeout that has already expired.
        let mut receiver = ScriptedReceiver::new([]);

        // Act
        let report = drain(&mut receiver, Duration::ZERO, |_event| async {}).await;

        // Assert
        // The timeout expired, but nothing was left behind, so the drain is not reported as cut short.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::Emptied,
                processed: 0,
                lost_to_lag: 0,
                unread_when_timed_out: 0,
            }
        );
    }

    #[tokio::test]
    async fn it_should_drain_every_event_when_the_timeout_is_too_large_to_add_to_the_clock() {
        // Arrange
        // Three events are queued, and the timeout is the largest a `Duration` can hold, which is
        // more than the clock can add to the current time.
        let mut receiver = ScriptedReceiver::new([Ok(1), Ok(2), Ok(3)]);

        // Act
        let report = drain(&mut receiver, Duration::MAX, |_event| async {}).await;

        // Assert
        // The drain does not panic: a timeout that never expires drains the queue to the end.
        assert_eq!(
            report,
            DrainReport {
                outcome: DrainOutcome::Emptied,
                processed: 3,
                lost_to_lag: 0,
                unread_when_timed_out: 0,
            }
        );
    }

    #[tokio::test]
    async fn it_should_let_other_tasks_run_while_it_drains_a_ready_queue() {
        // Arrange
        // On the single-threaded test runtime, 100 events are ready, and another task is waiting to
        // run. A handler that never waits gives the drain no reason to stop on its own.
        let mut receiver = ScriptedReceiver::new((0..100).map(Ok));
        let other_task_ran = Arc::new(AtomicBool::new(false));
        let other_task_flag = other_task_ran.clone();
        let _other_task = tokio::spawn(async move { other_task_flag.store(true, Ordering::SeqCst) });
        let other_task_ran_before_the_last_event = RefCell::new(false);

        // Act
        let _report = drain(&mut receiver, Duration::from_secs(1), |event| {
            if event == 99 {
                *other_task_ran_before_the_last_event.borrow_mut() = other_task_ran.load(Ordering::SeqCst);
            }
            async {}
        })
        .await;

        // Assert
        // The other task ran while the drain was still handling events, so a long drain cannot hold a
        // worker thread until its timeout expires.
        assert!(
            *other_task_ran_before_the_last_event.borrow(),
            "another task must get to run before a drain of 100 ready events ends"
        );
    }
}
