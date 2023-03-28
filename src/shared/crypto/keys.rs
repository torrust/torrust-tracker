//! This module contains logic related to cryptographic keys.
pub mod seeds {
    //! This module contains logic related to cryptographic seeds.
    //!
    //! Specifically, it contains the logic for storing the seed and providing
    //! it to other modules.
    //!
    //! A **seed** is a pseudo-random number that is used as a secret key for
    //! cryptographic operations.
    use self::detail::CURRENT_SEED;
    use crate::shared::crypto::ephemeral_instance_keys::{Seed, RANDOM_SEED};

    /// This trait is for structures that can keep and provide a seed.
    pub trait Keeper {
        type Seed: Sized + Default + AsMut<[u8]>;

        /// It returns a reference to the seed that is keeping.
        fn get_seed() -> &'static Self::Seed;
    }

    /// The seed keeper for the instance. When the application is running
    /// in production, this will be the seed keeper that is used.
    pub struct Instance;

    /// The seed keeper for the current execution. It's a facade at compilation
    /// time that will either be the instance seed keeper (with a randomly
    /// generated key for production) or the zeroed seed keeper.
    pub struct Current;

    impl Keeper for Instance {
        type Seed = Seed;

        fn get_seed() -> &'static Self::Seed {
            &RANDOM_SEED
        }
    }

    impl Keeper for Current {
        type Seed = Seed;

        #[allow(clippy::needless_borrow)]
        fn get_seed() -> &'static Self::Seed {
            &CURRENT_SEED
        }
    }

    #[cfg(test)]
    mod tests {
        use super::detail::ZEROED_TEST_SEED;
        use super::{Current, Instance, Keeper};
        use crate::shared::crypto::ephemeral_instance_keys::Seed;

        pub struct ZeroedTestSeed;

        impl Keeper for ZeroedTestSeed {
            type Seed = Seed;

            #[allow(clippy::needless_borrow)]
            fn get_seed() -> &'static Self::Seed {
                &ZEROED_TEST_SEED
            }
        }

        #[test]
        fn the_default_seed_and_the_zeroed_seed_should_be_the_same_when_testing() {
            assert_eq!(Current::get_seed(), ZeroedTestSeed::get_seed());
        }

        #[test]
        fn the_default_seed_and_the_instance_seed_should_be_different_when_testing() {
            assert_ne!(Current::get_seed(), Instance::get_seed());
        }
    }

    mod detail {
        use crate::shared::crypto::ephemeral_instance_keys::Seed;

        #[allow(dead_code)]
        pub const ZEROED_TEST_SEED: &Seed = &[0u8; 32];

        #[cfg(test)]
        pub use ZEROED_TEST_SEED as CURRENT_SEED;

        #[cfg(not(test))]
        pub use crate::shared::crypto::ephemeral_instance_keys::RANDOM_SEED as CURRENT_SEED;

        #[cfg(test)]
        mod tests {
            use std::convert::TryInto;

            use crate::shared::crypto::ephemeral_instance_keys::RANDOM_SEED;
            use crate::shared::crypto::keys::seeds::detail::ZEROED_TEST_SEED;
            use crate::shared::crypto::keys::seeds::CURRENT_SEED;

            #[test]
            fn it_should_have_a_zero_test_seed() {
                assert_eq!(*ZEROED_TEST_SEED, [0u8; 32]);
            }

            #[test]
            fn it_should_default_to_zeroed_seed_when_testing() {
                assert_eq!(*CURRENT_SEED, *ZEROED_TEST_SEED);
            }

            #[test]
            fn it_should_have_a_large_random_seed() {
                assert!(u128::from_ne_bytes((*RANDOM_SEED)[..16].try_into().unwrap()) > u128::from(u64::MAX));
                assert!(u128::from_ne_bytes((*RANDOM_SEED)[16..].try_into().unwrap()) > u128::from(u64::MAX));
            }
        }
    }
}
