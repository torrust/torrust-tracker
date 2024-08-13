const DEFAULT_MAX_RECURSION: usize = 50;
const DEFAULT_CHECK_KEY_SORT: bool = false;
const DEFAULT_ENFORCE_FULL_DECODE: bool = true;

/// Stores decoding options for modifying decode behavior.
#[derive(Copy, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct BDecodeOpt {
    max_recursion: usize,
    check_key_sort: bool,
    enforce_full_decode: bool,
}

impl BDecodeOpt {
    /// Create a new `BDecodeOpt` object.
    #[must_use]
    pub fn new(max_recursion: usize, check_key_sort: bool, enforce_full_decode: bool) -> BDecodeOpt {
        BDecodeOpt {
            max_recursion,
            check_key_sort,
            enforce_full_decode,
        }
    }

    /// Maximum limit allowed when decoding bencode.
    #[must_use]
    pub fn max_recursion(&self) -> usize {
        self.max_recursion
    }

    /// Whether or not an error should be thrown for out of order dictionary keys.
    #[must_use]
    pub fn check_key_sort(&self) -> bool {
        self.check_key_sort
    }

    /// Whether or not we enforce that the decoded bencode must make up all of the input
    /// bytes or not.
    ///
    /// It may be useful to disable this if for example, the input bencode is prepended to
    /// some payload and you would like to disassociate it. In this case, to find where the
    /// rest of the payload starts that wasn't decoded, get the bencode buffer, and call `len()`.
    #[must_use]
    pub fn enforce_full_decode(&self) -> bool {
        self.enforce_full_decode
    }
}

impl Default for BDecodeOpt {
    fn default() -> BDecodeOpt {
        BDecodeOpt::new(DEFAULT_MAX_RECURSION, DEFAULT_CHECK_KEY_SORT, DEFAULT_ENFORCE_FULL_DECODE)
    }
}
