use crate::shortint::engine::{EngineResult, ShortintEngine};
use crate::shortint::{Ciphertext, ServerKey};

impl ShortintEngine {
    pub(crate) fn unchecked_greater(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_greater_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_greater_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            (((x % large_mod / modulus) % modulus_msg) > (x % modulus_msg)) as u64
        })?;

        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_greater(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_greater_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_greater_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }

        self.unchecked_greater_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn unchecked_greater_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_greater_or_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_greater_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            (((x % large_mod / modulus) % modulus_msg) >= (x % modulus_msg)) as u64
        })?;

        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_greater_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_greater_or_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_greater_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_greater_or_equal_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn unchecked_less(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_less_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_less_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            (((x % large_mod / modulus) % modulus_msg) < (x % modulus_msg)) as u64
        })?;

        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_less(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_less_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_less_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_less_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn unchecked_less_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_less_or_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_less_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            (((x % large_mod / modulus) % modulus_msg) <= (x % modulus_msg)) as u64
        })?;

        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_less_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_less_or_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_less_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_less_or_equal_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn unchecked_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            ((((x % large_mod) / modulus) % modulus_msg) == (x % modulus_msg)) as u64
        })?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_equal_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn smart_scalar_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_equal_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let modulus = ct_left.message_modulus.0 as u64;
        let acc =
            self.generate_accumulator(server_key, |x| (x % modulus == scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn unchecked_not_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_not_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    fn unchecked_not_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let modulus_msg = ct_left.message_modulus.0 as u64;
        let large_mod = modulus * modulus_msg;
        self.unchecked_functional_bivariate_pbs_assign(server_key, ct_left, ct_right, |x| {
            ((((x % large_mod) / modulus) % modulus_msg) != (x % modulus_msg)) as u64
        })?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_not_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_not_equal_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_not_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_functional_bivariate_pbs_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_not_equal_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn smart_scalar_not_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_not_equal_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_not_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let modulus = ct_left.message_modulus.0 as u64;
        let acc =
            self.generate_accumulator(server_key, |x| (x % modulus != scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_scalar_greater_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_greater_or_equal_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_greater_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let acc = self.generate_accumulator(server_key, |x| (x >= scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_scalar_less_or_equal(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_less_or_equal_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_less_or_equal_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let acc = self.generate_accumulator(server_key, |x| (x <= scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_scalar_greater(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_greater_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_greater_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let acc = self.generate_accumulator(server_key, |x| (x > scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }

    pub(crate) fn smart_scalar_less(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        scalar: u8,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_scalar_less_assign(server_key, &mut result, scalar)?;
        Ok(result)
    }

    fn smart_scalar_less_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        scalar: u8,
    ) -> EngineResult<()> {
        let acc = self.generate_accumulator(server_key, |x| (x < scalar as u64) as u64)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree.0 = 1;
        Ok(())
    }
}
