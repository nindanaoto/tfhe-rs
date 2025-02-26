use crate::core_crypto::commons::math::decomposition::{
    DecompositionLevel, DecompositionTerm, DecompositionTermTensor,
};
use crate::core_crypto::commons::math::tensor::Tensor;
use crate::core_crypto::commons::numeric::UnsignedInteger;
use crate::core_crypto::commons::utils::{zip, zip_args};
use crate::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};

/// An iterator-like object that yields the terms of the signed decomposition of a tensor of values.
///
/// # Note
///
/// On each call to [`TensorSignedDecompositionIter::next_term`], this structure yields a new
/// [`DecompositionTermTensor`], backed by a `Vec` owned by the structure. This vec is mutated at
/// each call of the `next_term` method, and as such the term must be dropped before `next_term` is
/// called again.
///
/// Such a pattern can not be implemented with iterators yet (without GATs), which is why this
/// iterator must be explicitly called.
///
/// # Warning
///
/// This iterator yields the decomposition in reverse order. That means that the highest level
/// will be yielded first.
pub struct TensorSignedDecompositionIter<Scalar>
where
    Scalar: UnsignedInteger,
{
    // The base log of the decomposition
    base_log: usize,
    // The number of levels of the decomposition
    level_count: usize,
    // The current level
    current_level: usize,
    // A mask which allows to compute the mod B of a value. For B=2^4, this guy is of the form:
    // ...0001111
    mod_b_mask: Scalar,
    // The values being decomposed
    inputs: Vec<Scalar>,
    // The internal states of each decomposition
    states: Vec<Scalar>,
    // In order to avoid allocating a new Vec every time we yield a decomposition term, we store
    // a Vec inside the structure and yield slices pointing to it.
    outputs: Vec<Scalar>,
    // A flag which stores whether the iterator is a fresh one (for the recompose method).
    fresh: bool,
}

impl<Scalar> TensorSignedDecompositionIter<Scalar>
where
    Scalar: UnsignedInteger,
{
    // Creates a new tensor decomposition iterator.
    pub(crate) fn new(
        input: Tensor<Vec<Scalar>>,
        base_log: DecompositionBaseLog,
        level: DecompositionLevelCount,
    ) -> TensorSignedDecompositionIter<Scalar> {
        let len = input.len();
        TensorSignedDecompositionIter {
            base_log: base_log.0,
            level_count: level.0,
            current_level: level.0,
            mod_b_mask: (Scalar::ONE << base_log.0) - Scalar::ONE,
            inputs: input.clone().into_container(),
            outputs: vec![Scalar::ZERO; len],
            states: input
                .iter()
                .map(|i| *i >> (Scalar::BITS - base_log.0 * level.0))
                .collect(),
            fresh: true,
        }
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.fresh
    }

    /// Returns the logarithm in base two of the base of this decomposition.
    ///
    /// If the decomposition uses a base $B=2^b$, this returns $b$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::commons::math::tensor::Tensor;
    /// use tfhe::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let decomposable = Tensor::allocate(1_340_987_234_u32, 2);
    /// let decomp = decomposer.decompose_tensor(&decomposable);
    /// assert_eq!(decomp.base_log(), DecompositionBaseLog(4));
    /// ```
    pub fn base_log(&self) -> DecompositionBaseLog {
        DecompositionBaseLog(self.base_log)
    }

    /// Returns the number of levels of this decomposition.
    ///
    /// If the decomposition uses $l$ levels, this returns $l$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::commons::math::tensor::Tensor;
    /// use tfhe::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let decomposable = Tensor::allocate(1_340_987_234_u32, 2);
    /// let decomp = decomposer.decompose_tensor(&decomposable);
    /// assert_eq!(decomp.level_count(), DecompositionLevelCount(3));
    /// ```
    pub fn level_count(&self) -> DecompositionLevelCount {
        DecompositionLevelCount(self.level_count)
    }

    /// Yield the next term of the decomposition, if any.
    ///
    /// # Note
    ///
    /// Because this function returns a borrowed tensor, owned by the iterator, the term must be
    /// dropped before `next_term` is called again.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::{DecompositionLevel, SignedDecomposer};
    /// use tfhe::core_crypto::commons::math::tensor::Tensor;
    /// use tfhe::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let decomposable = Tensor::allocate(1_340_987_234_u32, 1);
    /// let mut decomp = decomposer.decompose_tensor(&decomposable);
    /// let term = decomp.next_term().unwrap();
    /// assert_eq!(term.level(), DecompositionLevel(3));
    /// assert_eq!(*term.as_tensor().get_element(0), 4294967295);
    /// ```
    pub fn next_term(&mut self) -> Option<DecompositionTermTensor<'_, Scalar>> {
        // The iterator is not fresh anymore.
        self.fresh = false;
        // We check if the decomposition is over
        if self.current_level == 0 {
            return None;
        }
        // We iterate over the elements of the outputs and decompose
        for zip_args!(output_i, state_i) in zip!(self.outputs.iter_mut(), self.states.iter_mut()) {
            *output_i = decompose_one_level(self.base_log, state_i, self.mod_b_mask);
        }
        self.current_level -= 1;
        // We return the term tensor.
        Some(DecompositionTermTensor::new(
            DecompositionLevel(self.current_level + 1),
            DecompositionBaseLog(self.base_log),
            Tensor::from_container(self.outputs.as_slice()),
        ))
    }
}

/// An iterator that yields the terms of the signed decomposition of an integer.
///
/// # Warning
///
/// This iterator yields the decomposition in reverse order. That means that the highest level
/// will be yielded first.
pub struct SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    // The value being decomposed
    input: T,
    // The base log of the decomposition
    base_log: usize,
    // The number of levels of the decomposition
    level_count: usize,
    // The internal state of the decomposition
    state: T,
    // The current level
    current_level: usize,
    // A mask which allows to compute the mod B of a value. For B=2^4, this guy is of the form:
    // ...0001111
    mod_b_mask: T,
    // A flag which store whether the iterator is a fresh one (for the recompose method)
    fresh: bool,
}

impl<T> SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    pub(crate) fn new(
        input: T,
        base_log: DecompositionBaseLog,
        level: DecompositionLevelCount,
    ) -> SignedDecompositionIter<T> {
        SignedDecompositionIter {
            input,
            base_log: base_log.0,
            level_count: level.0,
            state: input >> (T::BITS - base_log.0 * level.0),
            current_level: level.0,
            mod_b_mask: (T::ONE << base_log.0) - T::ONE,
            fresh: true,
        }
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.fresh
    }

    /// Returns the logarithm in base two of the base of this decomposition.
    ///
    /// If the decomposition uses a base $B=2^b$, this returns $b$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let val = 1_340_987_234_u32;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.base_log(), DecompositionBaseLog(4));
    /// ```
    pub fn base_log(&self) -> DecompositionBaseLog {
        DecompositionBaseLog(self.base_log)
    }

    /// Returns the number of levels of this decomposition.
    ///
    /// If the decomposition uses $l$ levels, this returns $l$.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::core_crypto::commons::math::decomposition::SignedDecomposer;
    /// use tfhe::core_crypto::prelude::{DecompositionBaseLog, DecompositionLevelCount};
    /// let decomposer =
    ///     SignedDecomposer::<u32>::new(DecompositionBaseLog(4), DecompositionLevelCount(3));
    /// let val = 1_340_987_234_u32;
    /// let decomp = decomposer.decompose(val);
    /// assert_eq!(decomp.level_count(), DecompositionLevelCount(3));
    /// ```
    pub fn level_count(&self) -> DecompositionLevelCount {
        DecompositionLevelCount(self.level_count)
    }
}

impl<T> Iterator for SignedDecompositionIter<T>
where
    T: UnsignedInteger,
{
    type Item = DecompositionTerm<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // The iterator is not fresh anymore
        self.fresh = false;
        // We check if the decomposition is over
        if self.current_level == 0 {
            return None;
        }
        // We decompose the current level
        let output = decompose_one_level(self.base_log, &mut self.state, self.mod_b_mask);
        self.current_level -= 1;
        // We return the output for this level
        Some(DecompositionTerm::new(
            DecompositionLevel(self.current_level + 1),
            DecompositionBaseLog(self.base_log),
            output,
        ))
    }
}

fn decompose_one_level<S: UnsignedInteger>(base_log: usize, state: &mut S, mod_b_mask: S) -> S {
    let res = *state & mod_b_mask;
    *state >>= base_log;
    let mut carry = (res.wrapping_sub(S::ONE) | *state) & res;
    carry >>= base_log - 1;
    *state += carry;
    res.wrapping_sub(carry << base_log)
}
