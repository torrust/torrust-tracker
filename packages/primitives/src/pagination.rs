use derive_more::Constructor;
use serde::Deserialize;

/// A struct to keep information about the page when results are being paginated
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Constructor)]
pub struct Pagination {
    /// The page number, starting at 0
    pub offset: u32,
    /// Page size. The number of results per page
    pub limit: u32,
}

impl Pagination {
    #[must_use]
    pub fn new_with_options(offset_option: Option<u32>, limit_option: Option<u32>) -> Self {
        let offset = match offset_option {
            Some(offset) => offset,
            None => Pagination::default_offset(),
        };
        let limit = match limit_option {
            Some(offset) => offset,
            None => Pagination::default_limit(),
        };

        Self { offset, limit }
    }

    #[must_use]
    pub fn default_offset() -> u32 {
        0
    }

    #[must_use]
    pub fn default_limit() -> u32 {
        4000
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: Self::default_offset(),
            limit: Self::default_limit(),
        }
    }
}
