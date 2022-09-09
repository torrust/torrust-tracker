use super::secret::Secret;
use std::convert::TryInto;
use blowfish::{BlowfishLE, cipher::{KeyInit, BlockEncrypt, BlockDecrypt}, Blowfish};
use byteorder::LittleEndian;
use cipher::generic_array::GenericArray;
use cipher::BlockSizeUser;

pub trait Cypher {
    fn encrypt(&self, decrypted_bytes: &[u8; 8]) -> [u8; 8];

    fn decrypt(&self, encrypted_bytes: &[u8; 8]) -> [u8; 8];
}

pub struct BlowfishCypher {
    blowfish: BlowfishLE
}

impl BlowfishCypher {
    pub fn new(secret: Secret) -> Self {
        Self {
            blowfish: BlowfishLE::new_from_slice(&secret.into_bytes()).unwrap()
        }
    }
}

type BlowfishArray = GenericArray<u8, <Blowfish<LittleEndian> as BlockSizeUser>::BlockSize>;

impl Cypher for BlowfishCypher {
    fn encrypt(&self, decrypted_bytes: &[u8; 8]) -> [u8; 8] {
        let mut encrypted_bytes: BlowfishArray = BlowfishArray::from(*decrypted_bytes);

        self.blowfish.encrypt_block(&mut encrypted_bytes);

        encrypted_bytes.try_into().unwrap()
    }

    fn decrypt(&self, encrypted_bytes: &[u8; 8]) -> [u8; 8] {
        let mut decrypted_bytes: BlowfishArray = BlowfishArray::from(*encrypted_bytes);

        self.blowfish.decrypt_block(&mut decrypted_bytes);

        decrypted_bytes.try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::udp::connection::{secret::Secret, cypher::{BlowfishCypher, Cypher}};

    #[test]
    fn it_should_encrypt_and_decrypt_a_byte_array() {
        let secret = Secret::from_bytes([0u8;32]);

        let cypher = BlowfishCypher::new(secret);

        let text = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];

        let encrypted_text = cypher.encrypt(&text);

        let decrypted_text = cypher.decrypt(&encrypted_text);

        assert_eq!(decrypted_text, [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8]);
    }
}