use crate::gadgets::{Gadget, Primitive, PrimitiveType};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::sha256::sha256;

pub struct Sha256;

impl<E: Engine> Gadget<E> for Sha256 {
    type Input = Vec<Primitive<E>>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let mut bits = Vec::new();
        for (i, bit_scalar) in input.into_iter().enumerate() {
            let allocated_bit = AllocatedBit::alloc(
                cs.namespace(|| format!("AllocatedBit {}", i)),
                bit_scalar.value.map(|fr| !fr.is_zero()),
            )?;

            bits.push(allocated_bit.into());
        }

        let digest_bits = sha256(cs.namespace(|| "sha256"), &bits)?;

        assert_eq!(digest_bits.len(), 256);

        let digest = digest_bits
            .into_iter()
            .map(|bit| Primitive {
                value: bit.get_value_field::<E>(),
                variable: bit
                    .get_variable()
                    .expect("sha256 must allocate")
                    .get_variable(),
                data_type: Some(PrimitiveType::BOOLEAN),
            })
            .collect();

        Ok(digest)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
