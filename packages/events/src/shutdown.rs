/// The explicit terminal state of a cooperatively cancellable event listener.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Completion {
    /// The listener's input closed independently of cancellation.
    Completed,

    /// The listener observed its cancellation token.
    Cancelled,
}
