use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum BencodeParseError {
    #[error("Incomplete Number Of Bytes At {pos}")]
    BytesEmpty { pos: usize },

    #[error("Invalid Byte Found At {pos}")]
    InvalidByte { pos: usize },

    #[error("Invalid Integer Found With No Delimiter At {pos}")]
    InvalidIntNoDelimiter { pos: usize },

    #[error("Invalid Integer Found As Negative Zero At {pos}")]
    InvalidIntNegativeZero { pos: usize },

    #[error("Invalid Integer Found With Zero Padding At {pos}")]
    InvalidIntZeroPadding { pos: usize },

    #[error("Invalid Integer Found To Fail Parsing At {pos}")]
    InvalidIntParseError { pos: usize },

    #[error("Invalid Dictionary Key Ordering Found At {pos} For Key {key:?}")]
    InvalidKeyOrdering { pos: usize, key: Vec<u8> },

    #[error("Invalid Dictionary Key Found At {pos} For Key {key:?}")]
    InvalidKeyDuplicates { pos: usize, key: Vec<u8> },

    #[error("Invalid Byte Length Found As Negative At {pos}")]
    InvalidLengthNegative { pos: usize },

    #[error("Invalid Byte Length Found To Overflow Buffer Length At {pos}")]
    InvalidLengthOverflow { pos: usize },

    #[error("Invalid Recursion Limit Exceeded At {pos} For Limit {max}")]
    InvalidRecursionExceeded { pos: usize, max: usize },
}

pub type BencodeParseResult<T> = Result<T, BencodeParseError>;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub enum BencodeConvertError {
    #[error("Missing Key In Bencode For {key:?}")]
    MissingKey { key: Vec<u8> },

    #[error("Wrong Type In Bencode For {key:?} Expected Type {expected_type}")]
    WrongType { key: Vec<u8>, expected_type: String },
}

pub type BencodeConvertResult<T> = Result<T, BencodeConvertError>;
