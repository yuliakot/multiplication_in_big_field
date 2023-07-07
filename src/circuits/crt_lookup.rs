use super::* ;
use std::collections::HashMap;
use halo2_base::utils::{fe_to_biguint, biguint_to_fe};

fn a_mod_p<F: ScalarField>(a: &F, p: &F) -> F{
    biguint_to_fe::<F>(& (fe_to_biguint(a) % fe_to_biguint(p)))
}

fn into_crt<F: ScalarField>(a: F, moduli: Vec<F>) -> HashMap<&'static F, F>{
    let res : HashMap<_, _> = moduli.iter().map(|x| (x, a_mod_p(&a, x))).collect();
    res
}

struct FLGateChip<F: ScalarField>(GateChip<F>);

impl<F: ScalarField> FLGateChip<F> {
    pub fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        ab: impl Into<QuantumCell<F>>,
        modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        let [a, b, ab] = [a, b, ab].map(|x| a_mod_p(*x.into().value(), modulus.into().value()));
        a.into()
    }

    pub fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        ab: impl Into<QuantumCell<F>>,
        modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        let [a, b, ab] = [a, b, ab].map(|x| a_mod_p(*x.into().value(), modulus.into().value()));
        a.into()
    }
}
