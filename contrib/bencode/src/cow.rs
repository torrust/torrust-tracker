use std::borrow::Cow;

/// Trait for macros to convert owned/borrowed types to `Cow`.
///
/// This is needed because `&str` and `String` do not have `From`
/// implements into `Cow<_, [u8]>`. One solution is to just call `AsRef<[u8]>`
/// before converting. However, then when a user specifies an owned type,
/// we will implicitly borrow that; this trait prevents that so that macro
/// behavior is intuitive, so that owned types stay owned.
pub trait BCowConvert<'a> {
    fn convert(self) -> Cow<'a, [u8]>;
}

// TODO: Enable when specialization lands.
/*
impl<'a, T> BCowConvert<'a> for T where T: AsRef<[u8]> + 'a {
    fn convert(self) -> Cow<'a, [u8]> {
        self.into()
    }
}*/

impl<'a> BCowConvert<'a> for &'a [u8] {
    fn convert(self) -> Cow<'a, [u8]> {
        self.into()
    }
}

impl<'a> BCowConvert<'a> for &'a str {
    fn convert(self) -> Cow<'a, [u8]> {
        self.as_bytes().into()
    }
}

impl BCowConvert<'static> for String {
    fn convert(self) -> Cow<'static, [u8]> {
        self.into_bytes().into()
    }
}

impl BCowConvert<'static> for Vec<u8> {
    fn convert(self) -> Cow<'static, [u8]> {
        self.into()
    }
}
