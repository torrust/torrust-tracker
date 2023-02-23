use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Returns a random alphanumeric string of a certain size.
pub fn string(size: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(size).map(char::from).collect()
}
