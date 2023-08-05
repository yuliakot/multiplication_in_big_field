// Here we compute 2^128 mod m_i

use halo2_base::utils::{ScalarField, fe_to_biguint, biguint_to_fe};
use halo2_base::{AssignedValue, Context};
use num_bigint::BigUint;
use num_integer::Integer;

#[cfg(test)]
pub mod tests;

const NUM_BITS: usize = 14;

fn pow_of_two() -> BigUint{
    BigUint::from(std::u128::MAX) + BigUint::from(1u32)
}

#[derive(Clone)]
pub struct Modulus<F: ScalarField>{
    pub value: F,
    pub assigned: AssignedValue<F>,
    pub residue_of_a_limb: F,
    pub bits: usize,
}

pub fn find_residue<F: ScalarField>(modulus: &BigUint)->F{
    biguint_to_fe::<F>(&pow_of_two().div_rem(modulus).1)
}


pub fn biguint_to_modulus<F: ScalarField>(ctx: &mut Context<F>, modulus: &BigUint)-> Modulus<F>{
    let residue_of_a_limb = find_residue(modulus);
    let value = biguint_to_fe::<F>(modulus);
    let assigned = ctx.load_constant(biguint_to_fe::<F>(modulus));
    Modulus{
        value: value,
        assigned: assigned,
        residue_of_a_limb: residue_of_a_limb,
        bits : NUM_BITS,
    }
}

pub fn fe_to_modulus<F: ScalarField>(ctx: &mut Context<F>, modulus: &F)-> Modulus<F>{
    let modulus = fe_to_biguint(modulus);
    biguint_to_modulus(ctx, &modulus)
}