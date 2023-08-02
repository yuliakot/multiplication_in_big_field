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
}

fn a_mod_q<F: ScalarField>(a: &BigUint, p: F) -> F{
    biguint_to_fe::<F>(&(a % fe_to_biguint(&p)))
}

fn into_crt<F: ScalarField>(a: &BigUint, moduli: &Vec<F>) -> Vec<F>{
    let res = moduli.iter().map(|x | a_mod_q(a, *x)).collect();
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

pub fn biguint_into_crtint<F:ScalarField>(a: &BigUint, moduli: &Vec<F>, p: &BigUint) -> CRTint<F>
{
    let residues: Vec<F> = into_crt(a, moduli);
    let residue_mod_n =  biguint_to_fe::<F>(a);
    let limbs = biguint_to_limbs::<F>(a);
    CRTint{
        residues: residues,
        value: a.clone(),
        residue_mod_n: residue_mod_n,
        limbs_as_fe: limbs,
    }
}


pub fn fe_into_crtint<F:ScalarField>(a: &F, moduli: &Vec<F>, p: &F) -> CRTint<F>
{
    let (a, p) = (&fe_to_biguint(a), &fe_to_biguint(p));
    biguint_into_crtint(a, moduli, p)

}