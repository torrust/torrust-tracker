pub mod keys {

    pub mod seeds {
        use self::detail::DEFAULT_SEED;
        use crate::ephemeral_instance_keys::{Seed, RANDOM_SEED};

        pub trait SeedKeeper {
            type Seed: Sized + Default + AsMut<[u8]>;
            fn get_seed() -> &'static Self::Seed;
        }

        pub struct InstanceSeed;
        pub struct DefaultSeed;

        impl SeedKeeper for InstanceSeed {
            type Seed = Seed;

            fn get_seed() -> &'static Self::Seed {
                &RANDOM_SEED
            }
        }

        impl SeedKeeper for DefaultSeed {
            type Seed = Seed;

            #[allow(clippy::needless_borrow)]
            fn get_seed() -> &'static Self::Seed {
                &DEFAULT_SEED
            }
        }

        #[cfg(test)]
        mod tests {
            use super::detail::ZEROED_TEST_SEED;
            use super::{DefaultSeed, InstanceSeed, SeedKeeper};
            use crate::ephemeral_instance_keys::Seed;

            pub struct ZeroedTestSeed;

            impl SeedKeeper for ZeroedTestSeed {
                type Seed = Seed;

                #[allow(clippy::needless_borrow)]
                fn get_seed() -> &'static Self::Seed {
                    &ZEROED_TEST_SEED
                }
            }

            #[test]
            fn the_default_seed_and_the_zeroed_seed_should_be_the_same_when_testing() {
                assert_eq!(DefaultSeed::get_seed(), ZeroedTestSeed::get_seed())
            }

            #[test]
            fn the_default_seed_and_the_instance_seed_should_be_different_when_testing() {
                assert_ne!(DefaultSeed::get_seed(), InstanceSeed::get_seed())
            }
        }

        mod detail {
            use crate::ephemeral_instance_keys::Seed;

            #[allow(dead_code)]
            pub const ZEROED_TEST_SEED: &Seed = &[0u8; 32];

            #[cfg(test)]
            pub use ZEROED_TEST_SEED as DEFAULT_SEED;

            #[cfg(not(test))]
            pub use crate::ephemeral_instance_keys::RANDOM_SEED as DEFAULT_SEED;

            #[cfg(test)]
            mod tests {
                use std::convert::TryInto;

                use crate::ephemeral_instance_keys::RANDOM_SEED;
                use crate::protocol::crypto::keys::seeds::detail::ZEROED_TEST_SEED;
                use crate::protocol::crypto::keys::seeds::DEFAULT_SEED;

                #[test]
                fn it_should_have_a_zero_test_seed() {
                    assert_eq!(*ZEROED_TEST_SEED, [0u8; 32])
                }

                #[test]
                fn it_should_default_to_zeroed_seed_when_testing() {
                    assert_eq!(*DEFAULT_SEED, *ZEROED_TEST_SEED)
                }

                #[test]
                fn it_should_have_a_large_random_seed() {
                    assert!(u128::from_ne_bytes((*RANDOM_SEED)[..16].try_into().unwrap()) > u64::MAX as u128);
                    assert!(u128::from_ne_bytes((*RANDOM_SEED)[16..].try_into().unwrap()) > u64::MAX as u128);
                }
            }
        }
    }

    pub mod block_ciphers {
        use cipher::generic_array::GenericArray;
        use cipher::BlockSizeUser;

        pub(super) use crate::block_ciphers::ephemeral_instance::BLOCK_CIPHER_BLOWFISH as INSTANCE_BLOCK_CIPHER;
        #[allow(unused_imports)]
        pub(super) use crate::block_ciphers::testing::TEST_BLOCK_CIPHER_BLOWFISH as TEST_BLOCK_CIPHER;
        use crate::block_ciphers::Cipher;

        pub trait BlockCipherKeeper {
            type BlockCipher: cipher::BlockCipher;
            fn get_block_cipher() -> &'static Self::BlockCipher;
        }

        pub type CipherArray = GenericArray<u8, <Cipher as BlockSizeUser>::BlockSize>;

        pub struct DefaultBlockCipher;
        pub struct InstanceBlockCipher;

        impl BlockCipherKeeper for DefaultBlockCipher {
            type BlockCipher = Cipher;
            fn get_block_cipher() -> &'static Self::BlockCipher {
                &self::detail::DEFAULT_BLOCK_CIPHER
            }
        }

        impl BlockCipherKeeper for InstanceBlockCipher {
            type BlockCipher = Cipher;
            fn get_block_cipher() -> &'static Self::BlockCipher {
                &INSTANCE_BLOCK_CIPHER
            }
        }

        #[cfg(test)]
        mod tests {
            use cipher::BlockEncrypt;

            use super::{BlockCipherKeeper, CipherArray, DefaultBlockCipher, InstanceBlockCipher, TEST_BLOCK_CIPHER};
            use crate::block_ciphers::Cipher;

            pub struct TestBlockCipher;

            impl BlockCipherKeeper for TestBlockCipher {
                type BlockCipher = Cipher;

                fn get_block_cipher() -> &'static Self::BlockCipher {
                    &TEST_BLOCK_CIPHER
                }
            }

            #[test]
            fn when_testing_the_default_and_test_block_ciphers_should_be_the_same() {
                let mut array = CipherArray::from([0u8; 8]);
                let mut array2 = CipherArray::from([0u8; 8]);

                DefaultBlockCipher::get_block_cipher().encrypt_block(&mut array);
                TestBlockCipher::get_block_cipher().encrypt_block(&mut array2);

                assert_eq!(array, array2)
            }

            #[test]
            fn when_testing_the_default_and_instance_block_ciphers_should_be_the_different() {
                let mut array = CipherArray::from([0u8; 8]);
                let mut array2 = CipherArray::from([0u8; 8]);

                DefaultBlockCipher::get_block_cipher().encrypt_block(&mut array);
                InstanceBlockCipher::get_block_cipher().encrypt_block(&mut array2);

                assert_ne!(array, array2)
            }
        }

        mod detail {
            #[cfg(not(test))]
            pub(super) use super::INSTANCE_BLOCK_CIPHER as DEFAULT_BLOCK_CIPHER;
            #[cfg(test)]
            pub(super) use super::TEST_BLOCK_CIPHER as DEFAULT_BLOCK_CIPHER;
        }
    }
}
