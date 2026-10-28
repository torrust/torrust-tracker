//! Block until a reviewer event happens on a pull request, with no model calls and no token of its own.
//!
//! The poller reads the pull request through the GitHub CLI and returns one [`Event`] as soon as a
//! watched reviewer reviews or comments, the head commit changes, or the pull request closes. If
//! nothing happens within the wait, it returns [`Event::Timeout`].

pub mod error;
pub mod event;
pub mod gh;
pub mod snapshot;
pub mod timestamp;
pub mod wait;

pub use error::Error;
pub use event::Event;
pub use gh::{Gh, GhCli};
pub use snapshot::PullRequest;
pub use wait::{Clock, Config, Since, SystemClock, wait};
