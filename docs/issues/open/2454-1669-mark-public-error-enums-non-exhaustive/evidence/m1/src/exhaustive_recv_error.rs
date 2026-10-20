use torrust_tracker_events::receiver::RecvError;

fn keep_listening(error: &RecvError) -> bool {
    match error {
        RecvError::Closed => false,
        RecvError::Lagged(_) => true,
    }
}

fn main() {
    let _ = keep_listening;
}
