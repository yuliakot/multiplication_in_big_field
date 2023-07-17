#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod multiplication_gates;

use itertools::izip;

use multiplication_gates::{mod_p_verifications::*, crt_lookup::FLGateChip};
use halo2_base::{
    halo2_proofs::halo2curves::bn256::Fr,
    utils::ScalarField,
    gates::{
        builder::GateCircuitBuilder,
        builder::GateThreadBuilder,
        GateChip,
        GateInstructions,
    },
    AssignedValue,
    Context,
    QuantumCell::{self, Constant, Existing, Witness, WitnessFraction},
};
use halo2_proofs_axiom::dev::MockProver;
use halo2_base::utils::{fe_to_biguint, biguint_to_fe};

fn a_mod_q<F: ScalarField>(a: &F, p: &F) -> F{
    biguint_to_fe::<F>(& (fe_to_biguint(a) % fe_to_biguint(p)))
}

pub fn into_crt<F: ScalarField>(a: &F, moduli: &Vec<F>) -> Vec<F>{
    let res = moduli.iter().map(|x | a_mod_q(a, x)).collect();
    res
}

pub fn crt_mul<F: ScalarField>(
    chip: &GateChip<F>,
    ctx: &mut Context<F>,
    [a, b, p, q, r]: [F; 5], 
    moduli: &Vec<F>)
    {
        
        mod_r_mul(chip, ctx, &[a, b, p, q, r]);


        let crt_inputs: Vec<(F, F, F, F, F, F, F)> = izip!(into_crt(&a, moduli), 
                                                            into_crt(&b, moduli),
                                                            into_crt(&(a*b), moduli), 
                                                            into_crt(&p, moduli), 
                                                            into_crt(&q, moduli),  
                                                            into_crt(&(p*q), moduli), 
                                                            into_crt(&r, moduli)
                                                        ).collect();

        for (input, &modulus) in crt_inputs.iter().zip(moduli){
            let (a, b, a_times_b, p, q, p_times_q, r) = *input;
            let v = ctx.assign_witnesses(vec![a, b, a_times_b, p, q, p_times_q, r]);
            let mut v = v.iter();
            let (a, b, a_times_b, p, q, p_times_q, r) 
            = (
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap(),
                v.next().unwrap()
            );

            
            chip.crt_lookup_division_with_remainder(ctx, [*a, *b, *a_times_b, *p, *q, *p_times_q, *r], modulus);
            
        }

    }
