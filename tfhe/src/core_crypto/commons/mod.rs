#![allow(dead_code, deprecated)] // For the time being

#[allow(unused_macros)]
macro_rules! assert_delta {
    ($A:expr, $B:expr, $d:expr) => {
        for (x, y) in $A.iter().zip($B) {
            assert!((*x as i64 - y as i64).abs() <= $d, "{} != {} ", *x, y);
        }
    };
}

#[allow(unused_macros)]
macro_rules! assert_delta_scalar {
    ($A:expr, $B:expr, $d:expr) => {
        assert!(
            ($A as i64 - $B as i64).abs() <= $d,
            "{} != {} +- {}",
            $A,
            $B,
            $d
        );
    };
}

#[allow(unused_macros)]
macro_rules! assert_delta_scalar_float {
    ($A:expr, $B:expr, $d:expr) => {
        assert!(($A - $B).abs() <= $d, "{} != {} +- {}", $A, $B, $d);
    };
}

#[allow(unused_macros)]
macro_rules! modular_distance {
    ($A:expr, $B:expr) => {
        ($A.wrapping_sub($B)).min($B.wrapping_sub($A))
    };
}

pub mod crypto;
pub mod math;
pub mod numeric;
pub mod utils;

#[doc(hidden)]
#[cfg(test)]
pub mod test_tools {
    use rand::Rng;

    use crate::core_crypto::commons::crypto::secret::generators::{
        EncryptionRandomGenerator, SecretRandomGenerator,
    };
    use crate::core_crypto::commons::math::random::{RandomGenerable, RandomGenerator, Uniform};
    use crate::core_crypto::commons::math::tensor::{AsRefSlice, AsRefTensor};
    use crate::core_crypto::commons::math::torus::UnsignedTorus;
    use crate::core_crypto::commons::numeric::UnsignedInteger;
    use crate::core_crypto::prelude::{
        CiphertextCount, DecompositionBaseLog, DecompositionLevelCount, DispersionParameter,
        GlweDimension, LweDimension, PlaintextCount, PolynomialSize,
    };
    use concrete_csprng::generators::SoftwareRandomGenerator;
    use concrete_csprng::seeders::{Seed, Seeder};

    fn modular_distance<T: UnsignedInteger>(first: T, other: T) -> T {
        let d0 = first.wrapping_sub(other);
        let d1 = other.wrapping_sub(first);
        std::cmp::min(d0, d1)
    }

    fn torus_modular_distance<T: UnsignedInteger>(first: T, other: T) -> f64 {
        let d0 = first.wrapping_sub(other);
        let d1 = other.wrapping_sub(first);
        if d0 < d1 {
            let d: f64 = d0.cast_into();
            d / 2_f64.powi(T::BITS as i32)
        } else {
            let d: f64 = d1.cast_into();
            -d / 2_f64.powi(T::BITS as i32)
        }
    }

    pub fn new_random_generator() -> RandomGenerator<SoftwareRandomGenerator> {
        RandomGenerator::new(random_seed())
    }

    pub fn new_secret_random_generator() -> SecretRandomGenerator<SoftwareRandomGenerator> {
        SecretRandomGenerator::new(random_seed())
    }

    pub fn new_encryption_random_generator() -> EncryptionRandomGenerator<SoftwareRandomGenerator> {
        EncryptionRandomGenerator::new(random_seed(), &mut UnsafeRandSeeder)
    }

    pub fn random_seed() -> Seed {
        Seed(rand::thread_rng().gen())
    }

    pub struct UnsafeRandSeeder;

    impl Seeder for UnsafeRandSeeder {
        fn seed(&mut self) -> Seed {
            Seed(rand::thread_rng().gen())
        }

        fn is_available() -> bool {
            true
        }
    }

    pub fn assert_delta_std_dev<First, Second, Element>(
        first: &First,
        second: &Second,
        dist: impl DispersionParameter,
    ) where
        First: AsRefTensor<Element = Element>,
        Second: AsRefTensor<Element = Element>,
        Element: UnsignedTorus,
    {
        for (x, y) in first.as_tensor().iter().zip(second.as_tensor().iter()) {
            println!("{:?}, {:?}", *x, *y);
            println!("{}", dist.get_standard_dev());
            let distance: f64 = modular_distance(*x, *y).cast_into();
            let torus_distance = distance / 2_f64.powi(Element::BITS as i32);
            assert!(
                torus_distance <= 5. * dist.get_standard_dev(),
                "{} != {} ",
                x,
                y
            );
        }
    }

    pub fn assert_noise_distribution<First, Second, Element>(
        first: &First,
        second: &Second,
        dist: impl DispersionParameter,
    ) where
        First: AsRefTensor<Element = Element>,
        Second: AsRefTensor<Element = Element>,
        Element: UnsignedTorus,
    {
        use crate::core_crypto::commons::math::tensor::Tensor;
        use rand::distributions::{Distribution, Normal};

        let std_dev = dist.get_standard_dev();
        let confidence = 0.95;
        let n_slots = first.as_tensor().len();

        // allocate 2 slices: one for the error samples obtained, the second for fresh samples
        // according to the std_dev computed
        let mut sdk_samples = Tensor::allocate(0.0_f64, n_slots);

        // recover the errors from each ciphertexts
        sdk_samples.fill_with_two(first.as_tensor(), second.as_tensor(), |a, b| {
            torus_modular_distance(*a, *b)
        });

        // fill the theoretical sample vector according to std_dev using the rand crate
        let mut theoretical_samples: Vec<f64> = Vec::with_capacity(n_slots);
        let normal = Normal::new(0.0, std_dev);
        for _i in 0..n_slots {
            theoretical_samples.push(normal.sample(&mut rand::thread_rng()));
        }

        // compute the kolmogorov smirnov test
        let result = kolmogorov_smirnov::test_f64(
            sdk_samples.as_slice(),
            theoretical_samples.as_slice(),
            confidence,
        );
        assert!(
            !result.is_rejected,
            "Not the same distribution with a probability of {}",
            result.reject_probability
        );
    }

    /// Returns a random plaintext count in [1;max].
    pub fn random_plaintext_count(max: usize) -> PlaintextCount {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        PlaintextCount((rng.gen::<usize>() % (max - 1)) + 1)
    }

    /// Returns a random ciphertext count in [1;max].
    pub fn random_ciphertext_count(max: usize) -> CiphertextCount {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        CiphertextCount((rng.gen::<usize>() % (max - 1)) + 1)
    }

    /// Returns a random LWE dimension in [1;max].
    pub fn random_lwe_dimension(max: usize) -> LweDimension {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        LweDimension((rng.gen::<usize>() % (max - 1)) + 1)
    }

    /// Returns a random GLWE dimension in [1;max].
    pub fn random_glwe_dimension(max: usize) -> GlweDimension {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        GlweDimension((rng.gen::<usize>() % (max - 1)) + 1)
    }

    /// Returns a random polynomial size in [2;max].
    pub fn random_polynomial_size(max: usize) -> PolynomialSize {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        PolynomialSize((rng.gen::<usize>() % (max - 2)) + 2)
    }

    /// Returns a random base log in [2;max].
    pub fn random_base_log(max: usize) -> DecompositionBaseLog {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        DecompositionBaseLog((rng.gen::<usize>() % (max - 2)) + 2)
    }

    /// Returns a random level count in [2;max].
    pub fn random_level_count(max: usize) -> DecompositionLevelCount {
        assert_ne!(max, 0, "Max cannot be 0");
        let mut rng = rand::thread_rng();
        DecompositionLevelCount((rng.gen::<usize>() % (max - 2)) + 2)
    }

    pub fn random_i32_between(range: std::ops::Range<i32>) -> i32 {
        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::from(range);
        let mut rng = rand::thread_rng();
        between.sample(&mut rng)
    }

    pub fn random_usize_between(range: std::ops::Range<usize>) -> usize {
        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::from(range);
        let mut rng = rand::thread_rng();
        between.sample(&mut rng)
    }

    pub fn any_usize() -> usize {
        random_usize_between(0..usize::MAX)
    }

    pub fn random_uint_between<T: UnsignedInteger + RandomGenerable<Uniform>>(
        range: std::ops::Range<T>,
    ) -> T {
        let mut generator = new_random_generator();
        let val: T = generator.random_uniform();
        val % (range.end - range.start) + range.start
    }

    pub fn any_uint<T: UnsignedInteger + RandomGenerable<Uniform>>() -> T {
        let mut generator = new_random_generator();
        generator.random_uniform()
    }
}
