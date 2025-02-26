use crate::core_crypto::prelude::*;
use crate::shortint::ciphertext::Degree;
use crate::shortint::engine::{EngineResult, ShortintEngine};
use crate::shortint::{Ciphertext, ServerKey};

impl ShortintEngine {
    pub(crate) fn unchecked_neg(
        &mut self,
        server_key: &ServerKey,
        ct: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct.clone();
        self.unchecked_neg_assign(server_key, &mut result)?;
        Ok(result)
    }

    pub(crate) fn unchecked_neg_with_z(
        &mut self,
        server_key: &ServerKey,
        ct: &Ciphertext,
    ) -> EngineResult<(Ciphertext, u64)> {
        let mut result = ct.clone();
        let z = self.unchecked_neg_assign_with_z(server_key, &mut result)?;
        Ok((result, z))
    }

    pub(crate) fn unchecked_neg_assign(
        &mut self,
        server_key: &ServerKey,
        ct: &mut Ciphertext,
    ) -> EngineResult<()> {
        let _z = self.unchecked_neg_assign_with_z(server_key, ct)?;
        Ok(())
    }

    pub(crate) fn unchecked_neg_assign_with_z(
        &mut self,
        _server_key: &ServerKey,
        ct: &mut Ciphertext,
    ) -> EngineResult<u64> {
        // z = ceil( degree / 2^p ) * 2^p
        let msg_mod = ct.message_modulus.0;
        let mut z = ((ct.degree.0 + msg_mod - 1) / msg_mod) as u64;
        z *= msg_mod as u64;

        // Value of the shift we multiply our messages by
        let delta =
            (1_u64 << 63) / (_server_key.message_modulus.0 * _server_key.carry_modulus.0) as u64;

        //Scaling + 1 on the padding bit
        let w = z * delta;

        // (0,Delta*z) - ct
        self.engine.fuse_opp_lwe_ciphertext(&mut ct.ct)?;

        let clear_w = self.engine.create_plaintext_from(&w)?;
        self.engine
            .fuse_add_lwe_ciphertext_plaintext(&mut ct.ct, &clear_w)?;

        // Update the degree
        ct.degree = Degree(z as usize);

        Ok(z)
    }

    pub(crate) fn smart_neg(
        &mut self,
        server_key: &ServerKey,
        ct: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        // If the ciphertext cannot be negated without exceeding the capacity of a ciphertext
        if !server_key.is_neg_possible(ct) {
            self.keyswitch_bootstrap_assign(server_key, ct)?;
        }
        self.unchecked_neg(server_key, ct)
    }

    pub(crate) fn smart_neg_assign(
        &mut self,
        server_key: &ServerKey,
        ct: &mut Ciphertext,
    ) -> EngineResult<()> {
        // If the ciphertext cannot be negated without exceeding the capacity of a ciphertext
        if !server_key.is_neg_possible(ct) {
            self.keyswitch_bootstrap_assign(server_key, ct)?;
        }
        self.unchecked_neg_assign(server_key, ct)
    }
}
