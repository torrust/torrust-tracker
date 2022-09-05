//! A secret for encryption.

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Secret([u8; 32]);

impl Secret {

    pub fn new(bytes: [u8; 32]) -> Self {
        Secret(bytes)
    }

    pub fn to_bytes(self) -> [u8; 32] {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_converted_into_a_generic_byte_array() {

        let byte_array_32 = Secret::new([0; 32]);

        assert_eq!(byte_array_32.to_bytes(), [0u8; 32]);
    }
}