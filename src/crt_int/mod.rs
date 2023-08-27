use halo2_base::utils::{ScalarField, fe_to_biguint, biguint_to_fe};
use num_bigint::BigUint;
use num_integer::Integer;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub struct CRTint<F: ScalarField>{
    pub residues: Vec<F>,
    pub value: BigUint,
    pub residue_mod_n: F,
    pub limbs_as_fe: [F; 2],
    pub limb_bits: usize,
}

fn a_mod_q<F: ScalarField>(a: &BigUint, p: F) -> F{
    biguint_to_fe::<F>(&(a % fe_to_biguint(&p)))
}

fn find_residues_fe<F: ScalarField>(a: &BigUint, moduli: &Vec<F>) -> Vec<F>{
    let res = moduli.iter().map(|x | a_mod_q(a, *x)).collect();
    res
}

fn find_residues_bui<F: ScalarField>(a: &BigUint, moduli: &Vec<BigUint>) -> Vec<F>{
    let res = moduli.iter().map(|x| biguint_to_fe(&a.div_rem(x).1)).collect();
    res
}

fn biguint_to_limbs<F: ScalarField>(a: &BigUint) -> [F; 2]{
    let mut e = a.iter_u64_digits();
    let mut limb0 = e.next().unwrap_or(0) as u128;
    let limb0_5 = e.next().unwrap_or(0) as u128;
    
    limb0 |= ((limb0_5 & ((1u128 << 64) - 1u128)) as u128) << 64u32;


    let mut limb1 = e.next().unwrap_or(0) as u128;
    let limb1_5 = e.next().unwrap_or(0) as u128;
    
    limb1 |= ((limb1_5 & ((1u128 << 64) - 1u128)) as u128) << 64u32;

    let limb0 = F::from_u128(limb0);
    let limb1 = F::from_u128(limb1);
    
    [limb0, limb1]
}

// Borrowed from https://github.com/axiom-crypto/halo2-lib/blob/f2eacb1f7fdbb760213cf8037a1bd1a10672133f/halo2-base/src/utils.rs#L106
// Returns the modulus of [ScalarField].
pub fn modulus<F: ScalarField>() -> BigUint {
    fe_to_biguint(&-F::one()) + 1u64
}

pub fn biguint_into_crtint_fe_modulus<F:ScalarField>(a: &BigUint, moduli: &Vec<F>) -> CRTint<F>
{
    let n = modulus::<F>();
    let residues: Vec<F> = find_residues_fe(a, moduli);
    let residue_mod_n =  biguint_to_fe::<F>(&(a % n));
    
    let limbs = biguint_to_limbs::<F>(a);
    CRTint{
        residues: residues,
        value: a.clone(),
        residue_mod_n: residue_mod_n,
        limbs_as_fe: limbs,
        limb_bits: 128
    }
}

pub fn biguint_into_crtint_bui_modulus<F:ScalarField>(a: &BigUint, moduli: &Vec<BigUint>) -> CRTint<F>
{
    let n = modulus::<F>();
    let residues: Vec<F> = find_residues_bui(a, moduli);
    let residue_mod_n =  biguint_to_fe::<F>(&(a % n));
    
    let limbs = biguint_to_limbs::<F>(a);
    CRTint{
        residues: residues,
        value: a.clone(),
        residue_mod_n: residue_mod_n,
        limbs_as_fe: limbs,
        limb_bits: 128
    }
}


pub fn fe_into_crtint<F:ScalarField>(a: &F, moduli: &Vec<F>) -> CRTint<F>
{
    let a= &fe_to_biguint(a);
    biguint_into_crtint_fe_modulus(a, moduli)

}
pub fn fe_into_crtint_bui_modulus<F:ScalarField>(a: &F, moduli: &Vec<BigUint>) -> CRTint<F>
{
    let a= &fe_to_biguint(a);
    biguint_into_crtint_bui_modulus(a, moduli)

}


fn pow_of_two() -> BigUint{
    use num_traits::One;
    BigUint::from(std::u128::MAX) + BigUint::one()
}

pub fn limb_bits_to_crt<F:ScalarField>([a0, a1]: [&F; 2], moduli: &Vec<F>) -> CRTint<F>
{
    let (a0, a1) = (fe_to_biguint(a0), fe_to_biguint(a1));
    let a = a0 + pow_of_two()*a1;
    biguint_into_crtint_fe_modulus(&a, moduli)

}