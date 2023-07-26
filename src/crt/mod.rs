use halo2_base::utils::{ScalarField, fe_to_biguint, biguint_to_fe};
use num_bigint::BigUint;

pub struct CRTint<F: ScalarField>{
    pub residues: Vec<F>,
    pub value: BigUint,
    pub residue_mod_n: F,
}

fn a_mod_q<F: ScalarField>(a: &BigUint, p: F) -> F{
    biguint_to_fe::<F>(&(a % fe_to_biguint(&p)))
}

fn into_crt<F: ScalarField>(a: &BigUint, moduli: &Vec<F>) -> Vec<F>{
    let res = moduli.iter().map(|x | a_mod_q(a, *x)).collect();
    res
}

pub fn into_crtint<F:ScalarField>(a: &BigUint, moduli: &Vec<F>, p: &BigUint) -> CRTint<F>
{
    let residues: Vec<F> = into_crt(a, moduli);
    let residue_mod_n =  biguint_to_fe::<F>(a);
    CRTint{
        residues: residues,
        value: a.clone(),
        residue_mod_n: residue_mod_n,
    }
}
