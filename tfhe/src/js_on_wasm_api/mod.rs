#[cfg(feature = "shortint-client-js-wasm-api")]
pub mod shortint;
#[cfg(feature = "shortint-client-js-wasm-api")]
pub use shortint::*;

#[cfg(feature = "boolean-client-js-wasm-api")]
pub mod boolean;
#[cfg(feature = "boolean-client-js-wasm-api")]
pub use boolean::*;

pub(self) mod js_wasm_seeder {
    use crate::core_crypto::commons::math::random::Seed;
    use crate::core_crypto::prelude::Seeder;

    const SEED_BYTES_COUNT: usize = 16;

    pub struct ConstantSeeder {
        seed: Seed,
    }

    impl ConstantSeeder {
        pub fn new(seed: Seed) -> Self {
            Self { seed }
        }
    }

    impl Seeder for ConstantSeeder {
        fn seed(&mut self) -> Seed {
            self.seed
        }

        fn is_available() -> bool
        where
            Self: Sized,
        {
            true
        }
    }
}
