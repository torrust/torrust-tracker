// https://stackoverflow.com/questions/74336993/getting-line-numbers-with-when-using-boxdyn-stderrorerror

use std::error::Error;
use std::panic::Location;
use std::sync::Arc;

pub struct Located<E>(pub E);

#[derive(Debug)]
pub struct LocatedError<'a, E>
where
    E: Error + ?Sized,
{
    source: Arc<E>,
    location: Box<Location<'a>>,
}

impl<'a, E> std::fmt::Display for LocatedError<'a, E>
where
    E: Error + ?Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.source, self.location)
    }
}

impl<'a, E> Error for LocatedError<'a, E>
where
    E: Error + ?Sized + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl<'a, E> Clone for LocatedError<'a, E>
where
    E: Error + ?Sized,
{
    fn clone(&self) -> Self {
        LocatedError {
            source: self.source.clone(),
            location: self.location.clone(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a, E> Into<LocatedError<'a, E>> for Located<E>
where
    E: Error,
    Arc<E>: Clone,
{
    #[track_caller]
    fn into(self) -> LocatedError<'a, E> {
        let e = LocatedError {
            source: Arc::new(self.0),
            location: Box::new(*std::panic::Location::caller()),
        };
        log::debug!("{e}");
        e
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<LocatedError<'a, dyn std::error::Error>> for Arc<dyn std::error::Error> {
    #[track_caller]
    fn into(self) -> LocatedError<'a, dyn std::error::Error> {
        LocatedError {
            source: self,
            location: Box::new(*std::panic::Location::caller()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::Location;

    use super::LocatedError;
    use crate::located_error::Located;

    #[derive(thiserror::Error, Debug)]
    enum TestError {
        #[error("Test")]
        Test,
    }

    #[track_caller]
    fn get_caller_location() -> Location<'static> {
        *Location::caller()
    }

    #[test]
    fn error_should_include_location() {
        let e = TestError::Test;

        let b: LocatedError<TestError> = Located(e).into();
        let l = get_caller_location();

        assert_eq!(b.location.file(), l.file());
    }
}
