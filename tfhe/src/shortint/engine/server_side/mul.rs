use crate::shortint::ciphertext::Degree;
use crate::shortint::engine::{EngineResult, ShortintEngine};
use crate::shortint::{Ciphertext, ServerKey};

impl ShortintEngine {
    pub(crate) fn unchecked_mul_lsb(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_mul_lsb_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn unchecked_mul_lsb_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;

        //message 1 is shifted to the carry bits
        self.unchecked_scalar_mul_assign(ct_left, modulus as u8)?;

        //message 2 is placed in the message bits
        self.unchecked_add_assign(ct_left, ct_right)?;

        //Modulus of the msg in the msg bits
        let res_modulus = ct_left.message_modulus.0 as u64;

        //generate the accumulator for the multiplication
        let acc = self.generate_accumulator(server_key, |x| {
            ((x / modulus) * (x % modulus)) % res_modulus
        })?;

        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;
        ct_left.degree = Degree(ct_left.message_modulus.0 - 1);
        Ok(())
    }

    pub(crate) fn unchecked_mul_msb(
        &mut self,
        server_key: &ServerKey,
        ct_left: &Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.unchecked_mul_msb_assign(server_key, &mut result, ct_right)?;

        Ok(result)
    }

    pub(crate) fn unchecked_mul_msb_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &Ciphertext,
    ) -> EngineResult<()> {
        let modulus = (ct_right.degree.0 + 1) as u64;
        let deg = (ct_left.degree.0 * ct_right.degree.0) / ct_right.message_modulus.0;

        // Message 1 is shifted to the carry bits
        self.unchecked_scalar_mul_assign(ct_left, modulus as u8)?;

        // Message 2 is placed in the message bits
        self.unchecked_add_assign(ct_left, ct_right)?;

        // Modulus of the msg in the msg bits
        let res_modulus = server_key.message_modulus.0 as u64;

        // Generate the accumulator for the multiplication
        let acc = self.generate_accumulator(server_key, |x| {
            ((x / modulus) * (x % modulus)) / res_modulus
        })?;

        self.programmable_bootstrap_keyswitch_assign(server_key, ct_left, &acc)?;

        ct_left.degree = Degree(deg);
        Ok(())
    }

    pub(crate) fn unchecked_mul_lsb_small_carry_modulus(
        &mut self,
        server_key: &ServerKey,
        ct1: &mut Ciphertext,
        ct2: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        //ct1 + ct2
        let mut ct_tmp_left = self.unchecked_add(ct1, ct2)?;

        //ct1-ct2
        let (mut ct_tmp_right, z) = self.unchecked_sub_with_z(server_key, ct1, ct2)?;

        //Modulus of the msg in the msg bits
        let modulus = ct1.message_modulus.0 as u64;

        let acc_add = self.generate_accumulator(server_key, |x| ((x * x) / 4) % modulus)?;
        let acc_sub =
            self.generate_accumulator(server_key, |x| (((x - z) * (x - z)) / 4) % modulus)?;

        self.programmable_bootstrap_keyswitch_assign(server_key, &mut ct_tmp_left, &acc_add)?;
        self.programmable_bootstrap_keyswitch_assign(server_key, &mut ct_tmp_right, &acc_sub)?;

        //Last subtraction might fill one bit of carry
        self.unchecked_sub(server_key, &ct_tmp_left, &ct_tmp_right)
    }

    pub(crate) fn unchecked_mul_lsb_small_carry_modulus_assign(
        &mut self,
        server_key: &ServerKey,
        ct1: &mut Ciphertext,
        ct2: &mut Ciphertext,
    ) -> EngineResult<()> {
        *ct1 = self.unchecked_mul_lsb_small_carry_modulus(server_key, ct1, ct2)?;
        Ok(())
    }

    pub(crate) fn smart_mul_lsb_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        //Choice of the multiplication algorithm depending on the parameters
        if ct_left.message_modulus.0 > ct_left.carry_modulus.0 {
            //If the ciphertext cannot be added together without exceeding the capacity of a
            // ciphertext
            if !server_key.is_mul_small_carry_possible(ct_left, ct_right) {
                self.message_extract_assign(server_key, ct_left)?;
                self.message_extract_assign(server_key, ct_right)?;
            }
            self.unchecked_mul_lsb_small_carry_modulus_assign(server_key, ct_left, ct_right)?;
        } else {
            //If the ciphertext cannot be added together without exceeding the capacity of a
            // ciphertext
            if !server_key.is_mul_possible(ct_left, ct_right) {
                if server_key.message_modulus.0 * (ct_right.degree.0 + 1)
                    < (ct_right.carry_modulus.0 * ct_right.message_modulus.0 - 1)
                {
                    self.message_extract_assign(server_key, ct_left)?;
                } else if (server_key.message_modulus.0 + 1) + (ct_left.degree.0 + 1)
                    < (ct_right.carry_modulus.0 * ct_right.message_modulus.0 - 1)
                {
                    self.message_extract_assign(server_key, ct_right)?;
                } else {
                    self.message_extract_assign(server_key, ct_left)?;
                    self.message_extract_assign(server_key, ct_right)?;
                }
            }
            self.unchecked_mul_lsb_assign(server_key, ct_left, ct_right)?;
        }
        Ok(())
    }

    pub(crate) fn smart_mul_lsb(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_mul_lsb_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }

    pub(crate) fn smart_mul_msb_assign(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<()> {
        if !server_key.is_mul_possible(ct_left, ct_right) {
            self.message_extract_assign(server_key, ct_left)?;
            self.message_extract_assign(server_key, ct_right)?;
        }
        self.unchecked_mul_msb_assign(server_key, ct_left, ct_right)?;
        Ok(())
    }

    pub(crate) fn smart_mul_msb(
        &mut self,
        server_key: &ServerKey,
        ct_left: &mut Ciphertext,
        ct_right: &mut Ciphertext,
    ) -> EngineResult<Ciphertext> {
        let mut result = ct_left.clone();
        self.smart_mul_msb_assign(server_key, &mut result, ct_right)?;
        Ok(result)
    }
}
