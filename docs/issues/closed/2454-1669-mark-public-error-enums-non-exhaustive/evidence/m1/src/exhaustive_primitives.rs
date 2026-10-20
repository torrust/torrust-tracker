use torrust_tracker_primitives::peer::IdConversionError;

fn describe(error: &IdConversionError) -> &'static str {
    match error {
        IdConversionError::NotEnoughBytes { .. } => "not enough bytes",
        IdConversionError::TooManyBytes { .. } => "too many bytes",
    }
}

fn main() {
    let _ = describe;
}
