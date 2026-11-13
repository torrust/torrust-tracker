use derive_more::Constructor;
use serde::Deserialize;

/// A struct to keep information about the page when results are being paginated
// `derive_more::Constructor` generates `field: field` initializers on this MSRV-compatible version.
// Nightly Clippy diagnoses that proc-macro expansion; remove this allowance once derive_more emits
// field-init shorthand.
#[allow(
    clippy::redundant_field_names,
    reason = "derive_more::Constructor emits field initializers on this MSRV-compatible version"
)]
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Constructor)]
pub struct Pagination {
    /// The number of results to skip: `0` starts at the first result
    pub offset: u32,
    /// Page size. The number of results per page
    pub limit: u32,
}

impl Pagination {
    #[must_use]
    pub const fn new_with_options(offset_option: Option<u32>, limit_option: Option<u32>) -> Self {
        let offset = match offset_option {
            Some(offset) => offset,
            None => Self::default_offset(),
        };
        let limit = match limit_option {
            Some(offset) => offset,
            None => Self::default_limit(),
        };

        Self { offset, limit }
    }

    #[must_use]
    pub const fn default_offset() -> u32 {
        0
    }

    #[must_use]
    pub const fn default_limit() -> u32 {
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

#[cfg(test)]
mod tests {
    use crate::pagination::Pagination;

    #[test]
    fn it_should_default_to_the_first_result_and_a_page_of_4000() {
        let pagination = Pagination::default();

        assert_eq!(pagination, Pagination::new(0, 4000));
    }

    #[test]
    fn it_should_default_each_value_that_is_missing_and_keep_each_value_that_is_given() {
        let cases = [
            (None, None, Pagination::new(0, 4000)),
            (Some(5), None, Pagination::new(5, 4000)),
            (None, Some(10), Pagination::new(0, 10)),
            (Some(5), Some(10), Pagination::new(5, 10)),
            (Some(0), Some(0), Pagination::new(0, 0)),
        ];

        for (offset, limit, expected) in cases {
            assert_eq!(
                Pagination::new_with_options(offset, limit),
                expected,
                "offset {offset:?} and limit {limit:?}"
            );
        }
    }

    #[test]
    fn it_should_be_deserialized_from_an_offset_and_a_limit() {
        let pagination: Pagination =
            serde_json::from_str(r#"{"offset": 5, "limit": 10}"#).expect("the pagination should deserialize");

        assert_eq!(pagination, Pagination::new(5, 10));
    }
}
