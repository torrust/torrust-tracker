#![allow(clippy::missing_errors_doc)]
use crate::access::bencode::{BRefAccess, BRefAccessExt};
use crate::access::dict::BDictAccess;
use crate::access::list::BListAccess;
use crate::BencodeConvertError;

/// Trait for extended casting of bencode objects and converting conversion errors into application specific errors.
pub trait BConvertExt: BConvert {
    /// See `BConvert::convert_bytes`.
    fn convert_bytes_ext<'a, B, E>(&self, bencode: B, error_key: E) -> Result<&'a [u8], Self::Error>
    where
        B: BRefAccessExt<'a>,
        E: AsRef<[u8]>,
    {
        bencode.bytes_ext().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "Bytes".to_owned(),
        }))
    }

    /// See `BConvert::convert_str`.
    fn convert_str_ext<'a, B, E>(&self, bencode: &B, error_key: E) -> Result<&'a str, Self::Error>
    where
        B: BRefAccessExt<'a>,
        E: AsRef<[u8]>,
    {
        bencode.str_ext().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "UTF-8 Bytes".to_owned(),
        }))
    }

    /// See `BConvert::lookup_and_convert_bytes`.
    fn lookup_and_convert_bytes_ext<'a, B, K1, K2>(
        &self,
        dictionary: &dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a [u8], Self::Error>
    where
        B: BRefAccessExt<'a>,
        K2: AsRef<[u8]>,
    {
        self.convert_bytes_ext(self.lookup(dictionary, &key)?, &key)
    }

    /// See `BConvert::lookup_and_convert_str`.
    fn lookup_and_convert_str_ext<'a, B, K1, K2>(
        &self,
        dictionary: &dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a str, Self::Error>
    where
        B: BRefAccessExt<'a>,
        K2: AsRef<[u8]>,
    {
        self.convert_str_ext(self.lookup(dictionary, &key)?, &key)
    }
}

/// Trait for casting bencode objects and converting conversion errors into application specific errors.
#[allow(clippy::module_name_repetitions)]
pub trait BConvert {
    type Error;

    /// Convert the given conversion error into the appropriate error type.
    fn handle_error(&self, error: BencodeConvertError) -> Self::Error;

    /// Attempt to convert the given bencode value into an integer.
    ///
    /// Error key is used to generate an appropriate error message should the operation return an error.
    fn convert_int<B, E>(&self, bencode: B, error_key: E) -> Result<i64, Self::Error>
    where
        B: BRefAccess,
        E: AsRef<[u8]>,
    {
        bencode.int().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "Integer".to_owned(),
        }))
    }

    /// Attempt to convert the given bencode value into bytes.
    ///
    /// Error key is used to generate an appropriate error message should the operation return an error.
    fn convert_bytes<'a, B, E>(&self, bencode: &'a B, error_key: E) -> Result<&'a [u8], Self::Error>
    where
        B: BRefAccess,
        E: AsRef<[u8]>,
    {
        bencode.bytes().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "Bytes".to_owned(),
        }))
    }

    /// Attempt to convert the given bencode value into a UTF-8 string.
    ///
    /// Error key is used to generate an appropriate error message should the operation return an error.
    fn convert_str<'a, B, E>(&self, bencode: &'a B, error_key: E) -> Result<&'a str, Self::Error>
    where
        B: BRefAccess,
        E: AsRef<[u8]>,
    {
        bencode.str().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "UTF-8 Bytes".to_owned(),
        }))
    }

    /// Attempt to convert the given bencode value into a list.
    ///
    /// Error key is used to generate an appropriate error message should the operation return an error.
    fn convert_list<'a, B, E>(&self, bencode: &'a B, error_key: E) -> Result<&'a dyn BListAccess<B::BType>, Self::Error>
    where
        B: BRefAccess,
        E: AsRef<[u8]>,
    {
        bencode.list().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "List".to_owned(),
        }))
    }

    /// Attempt to convert the given bencode value into a dictionary.
    ///
    /// Error key is used to generate an appropriate error message should the operation return an error.
    fn convert_dict<'a, B, E>(&self, bencode: &'a B, error_key: E) -> Result<&'a dyn BDictAccess<B::BKey, B::BType>, Self::Error>
    where
        B: BRefAccess,
        E: AsRef<[u8]>,
    {
        bencode.dict().ok_or(self.handle_error(BencodeConvertError::WrongType {
            key: error_key.as_ref().to_owned(),
            expected_type: "Dictionary".to_owned(),
        }))
    }

    /// Look up a value in a dictionary of bencoded values using the given key.
    fn lookup<'a, B, K1, K2>(&self, dictionary: &'a dyn BDictAccess<K1, B>, key: K2) -> Result<&'a B, Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        let key_ref = key.as_ref();

        match dictionary.lookup(key_ref) {
            Some(n) => Ok(n),
            None => Err(self.handle_error(BencodeConvertError::MissingKey { key: key_ref.to_owned() })),
        }
    }

    /// Combines a lookup operation on the given key with a conversion of the value, if found, to an integer.
    fn lookup_and_convert_int<B, K1, K2>(&self, dictionary: &dyn BDictAccess<K1, B>, key: K2) -> Result<i64, Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        self.convert_int(self.lookup(dictionary, &key)?, &key)
    }

    /// Combines a lookup operation on the given key with a conversion of the value, if found, to a series of bytes.
    fn lookup_and_convert_bytes<'a, B, K1, K2>(
        &self,
        dictionary: &'a dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a [u8], Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        self.convert_bytes(self.lookup(dictionary, &key)?, &key)
    }

    /// Combines a lookup operation on the given key with a conversion of the value, if found, to a UTF-8 string.
    fn lookup_and_convert_str<'a, B, K1, K2>(
        &self,
        dictionary: &'a dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a str, Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        self.convert_str(self.lookup(dictionary, &key)?, &key)
    }

    /// Combines a lookup operation on the given key with a conversion of the value, if found, to a list.
    fn lookup_and_convert_list<'a, B, K1, K2>(
        &self,
        dictionary: &'a dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a dyn BListAccess<B::BType>, Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        self.convert_list(self.lookup(dictionary, &key)?, &key)
    }

    /// Combines a lookup operation on the given key with a conversion of the value, if found, to a dictionary.
    fn lookup_and_convert_dict<'a, B, K1, K2>(
        &self,
        dictionary: &'a dyn BDictAccess<K1, B>,
        key: K2,
    ) -> Result<&'a dyn BDictAccess<B::BKey, B::BType>, Self::Error>
    where
        B: BRefAccess,
        K2: AsRef<[u8]>,
    {
        self.convert_dict(self.lookup(dictionary, &key)?, &key)
    }
}
