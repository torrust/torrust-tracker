//! A secret for encryption.

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Secret([u8; 32]);

impl Secret {

    pub fn new() -> Self {
        let key = Self::generate_random_key();
        Self::from_bytes(key)
    }

    pub fn generate_random_key() -> [u8; 32] {
        let key: [u8; 32] = rand::Rng::gen(&mut rand::rngs::ThreadRng::default());
        key
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Secret(bytes)
    }

    pub fn into_bytes(self) -> [u8; 32] {
        self.0
    }
}

impl Default for Secret {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Secret> for [u8; 32] {
    fn from(secret: Secret) -> Self {
        secret.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_created_from_a_preexisting_byte_array_key() {
        let secret = Secret::from_bytes([0; 32]);
        assert_eq!(secret, Secret([0u8; 32]));
    }

    #[test]
    fn it_should_be_converted_into_a_byte_array_using_the_standard_trait() {
        let byte_array_32: [u8; 32] = Secret::from_bytes([0; 32]).into();
        assert_eq!(byte_array_32, [0u8; 32]);
    }

    #[test]
    fn it_should_be_converted_into_a_byte_array() {
        let byte_array_32_1 = Secret::from_bytes([0; 32]);
        assert_eq!(byte_array_32_1.into_bytes(), [0u8; 32]);
    }
}
