// Here we compute 2^128 mod m_i

use halo2_base::utils::{ScalarField, fe_to_biguint, biguint_to_fe};
use num_bigint::BigUint;
use num_integer::Integer;

#[cfg(test)]
pub mod tests;

fn pow_of_two() -> BigUint{
    BigUint::from(std::u128::MAX) + BigUint::from(1u32)
}

pub struct Modulus<F: ScalarField>{
    pub value: F,
    pub residue_of_a_limb: F,
}

pub fn find_residue<F: ScalarField>(modulus: &BigUint)->F{
    biguint_to_fe::<F>(&pow_of_two().div_rem(modulus).1)
}


pub fn biguint_to_modulus<F: ScalarField>(modulus: &BigUint)-> Modulus::<F>{
    let residue_of_a_limb = find_residue(modulus);
    let modulus = biguint_to_fe::<F>(modulus);
    Modulus{
        value: modulus,
        residue_of_a_limb: residue_of_a_limb,
    }
}