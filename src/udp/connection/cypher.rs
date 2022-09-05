use crypto::{blowfish::Blowfish, symmetriccipher::{BlockEncryptor, BlockDecryptor}};

use super::secret::Secret;

pub trait Cypher {
    fn encrypt(&self, decrypted_bytes: &[u8; 8]) -> [u8; 8];

    fn decrypt(&self, encrypted_bytes: &[u8; 8]) -> [u8; 8];
}

pub struct BlowfishCypher {
    blowfish: Blowfish
}

impl BlowfishCypher {
    pub fn new(secret: Secret) -> Self {
        let blowfish = Blowfish::new(&secret.to_bytes());
        BlowfishCypher {
            blowfish
        }
    }
}

impl Cypher for BlowfishCypher {
    fn encrypt(&self, decrypted_bytes: &[u8; 8]) -> [u8; 8] {
        let mut encrypted_bytes = [0u8; 8];

        self.blowfish.encrypt_block(decrypted_bytes, &mut encrypted_bytes);

        encrypted_bytes
    }

    fn decrypt(&self, encrypted_bytes: &[u8; 8]) -> [u8; 8] {
        let mut decrypted_bytes = [0u8; 8];

        self.blowfish.decrypt_block(encrypted_bytes, &mut decrypted_bytes);

        decrypted_bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{secret::Secret, cypher::{BlowfishCypher, Cypher}};


    #[test]
    fn it_should_encrypt_and_decrypt_a_byte_array() {
        let secret = Secret::new([0u8;32]);

        let cypher = BlowfishCypher::new(secret);

        let text = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];

        let encrypted_text = cypher.encrypt(&text);

        let decrypted_text = cypher.decrypt(&encrypted_text);

        assert_eq!(decrypted_text, text);
    }
}