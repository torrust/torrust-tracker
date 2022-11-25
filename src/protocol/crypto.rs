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
                assert_eq!(DefaultSeed::get_seed(), ZeroedTestSeed::get_seed());
            }

            #[test]
            fn the_default_seed_and_the_instance_seed_should_be_different_when_testing() {
                assert_ne!(DefaultSeed::get_seed(), InstanceSeed::get_seed());
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
                    assert_eq!(*ZEROED_TEST_SEED, [0u8; 32]);
                }

                #[test]
                fn it_should_default_to_zeroed_seed_when_testing() {
                    assert_eq!(*DEFAULT_SEED, *ZEROED_TEST_SEED);
                }

                #[test]
                fn it_should_have_a_large_random_seed() {
                    assert!(u128::from_ne_bytes((*RANDOM_SEED)[..16].try_into().unwrap()) > u128::from(u64::MAX));
                    assert!(u128::from_ne_bytes((*RANDOM_SEED)[16..].try_into().unwrap()) > u128::from(u64::MAX));
                }
            }
        }
    }
}
