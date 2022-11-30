use derive_more::{Display, Error};

#[derive(Debug, Display, PartialEq, Eq, Error)]
#[allow(dead_code)]
pub enum Error {
    #[display(fmt = "Query returned no rows.")]
    QueryReturnedNoRows,
    #[display(fmt = "Invalid query.")]
    InvalidQuery,
    #[display(fmt = "Database error.")]
    DatabaseError,
}

impl From<r2d2_sqlite::rusqlite::Error> for Error {
    fn from(e: r2d2_sqlite::rusqlite::Error) -> Self {
        match e {
            r2d2_sqlite::rusqlite::Error::QueryReturnedNoRows => Error::QueryReturnedNoRows,
            _ => Error::InvalidQuery,
        }
    }
}
