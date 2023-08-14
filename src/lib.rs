#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod multiplication_gates;
pub mod crt_int;
pub mod moduli_precomuted;
pub mod range_checks;


use itertools::izip;
use moduli_precomuted::Modulus;
use num_bigint::BigUint;
use num_integer::Integer;
use std::convert::TryInto;


use multiplication_gates::{mod_p_verifications::*, crt_lookup::FLGateChip};
use halo2_base::{
    halo2_proofs::halo2curves::bn256::Fr,
    utils::{ScalarField, biguint_to_fe},
    gates::{
        builder::GateCircuitBuilder,
        builder::GateThreadBuilder,
        GateChip,
        GateInstructions,
        RangeChip,
        RangeInstructions
    },
    AssignedValue,
    Context,
    QuantumCell::{self, Constant, Existing, Witness, WitnessFraction},
};
use halo2_proofs_axiom::dev::MockProver;

use crate::{crt_int::{CRTint, biguint_into_crtint}, 
            multiplication_gates::crt_lookup,
            moduli_precomuted::fe_to_modulus,
            range_checks::{check_big_less_than_p, check_big_less_than_p_minus_one}
        };

fn into_crtint_batch<F:ScalarField>(
    (a,b, a_times_b, q,p_times_q, r): 
    (&BigUint,&BigUint,&BigUint,&BigUint,&BigUint,&BigUint,),
     moduli: &Vec<F>,
     p: &BigUint) -> [CRTint<F>; 6]
 {
    [biguint_into_crtint(a, moduli, p), 
    biguint_into_crtint(b, moduli, p),
    biguint_into_crtint(a_times_b, moduli, p),
    biguint_into_crtint(q, moduli, p),  
    biguint_into_crtint(p_times_q, moduli, p), 
    biguint_into_crtint(r, moduli, p)
     ]
 }

 fn zip_residues_batch<F:ScalarField>(
    inputs: [&CRTint<F>; 6],
    crt_p: &CRTint<F>,
    moduli: &Vec<F>) -> Vec<([F; 6], F, F)>
 {
    let v: [Vec<F>; 6] = inputs.map(|x| x.residues.clone());
    let zipped_v: Vec::<[F; 6]> = 
        (0..v[0].len())
            .map(|i| TryInto::<[F; 6]>::try_into(
                    v.iter().map(|inner| inner[i].clone()).collect::<Vec::<F>>())
                    .unwrap())
            .collect();
    (0..v[0].len())
        .map(|i| (zipped_v[i], crt_p.residues[i], moduli[i]))
        .collect()
 }

fn load_bits_batch<F: ScalarField>(
    ctx: &mut Context<F>,
    inputs: [&CRTint<F>; 6],) -> [[AssignedValue<F>; 2]; 6]
{
    let out: [AssignedValue<F>; 6];
    inputs.map(|x| ctx.assign_witnesses(x.limbs_as_fe).try_into().unwrap())
}
 

pub fn crt_mul<F: ScalarField>(
    chip: RangeChip<F>,
    ctx: &mut Context<F>,
    a: &BigUint,
    b: &BigUint,
    crt_p: &CRTint<F>,
    moduli: &Vec<F>) -> AssignedValue<F>
    {

        //Step 1: finding q, r, a_times_b = a*b = p*q + r; p_times_q
        let p = crt_p.value.clone();
        let a_times_b = a*b;
        let (q, r) = a_times_b.div_rem(&p);
        let p_times_q = &a_times_b - &r;
        

        println!("\na = {:?}, \nb = {:?}, \np = {:?}, \nq = {:?}, \nr = {:?}", a, b, p, q, r);

        let [a,b, a_times_b, q,p_times_q, r] = 
            into_crtint_batch((&a,&b, &a_times_b, &q,&p_times_q, &r), moduli, &p);
                        
        //Steps 2-3: range checks:

        let [bits_a, bits_b, bits_ab, bits_q, bits_pq, bits_r] = load_bits_batch(ctx, [&a, &b, &a_times_b, &q, &p_times_q, &r]);
        let bits_p = ctx.assign_witnesses(crt_p.limbs_as_fe).try_into().unwrap();
        check_big_less_than_p(&chip, ctx, bits_a, bits_p);
        check_big_less_than_p(&chip, ctx, bits_b, bits_p);
        check_big_less_than_p_minus_one(&chip, ctx, bits_q, bits_p);
        check_big_less_than_p(&chip, ctx, bits_r, bits_p);

        //TODO: Check bits - to - CRT

        //Step 4: native operations
        let p_mod_n = ctx.load_constant(biguint_to_fe(&p));
        
        let mod_n_inputs = [a.residue_mod_n,
                                    b.residue_mod_n, 
                                    q.residue_mod_n, 
                                    r.residue_mod_n];

        mod_r_mul(&chip.gate(), ctx, &mod_n_inputs, p_mod_n);

        //Steps 5-6: CRT operations

        let zipped_inputs = 
        zip_residues_batch([&a,&b, &a_times_b, &q,&p_times_q, &r], &crt_p, moduli);

        for (curr_inputs, curr_p, curr_modulus) in zipped_inputs.iter()
        {
            let assigned_inputs = ctx.assign_witnesses(*curr_inputs);
            let assigned_p = ctx.load_constant(*curr_p);
            let assigned_inputs: [AssignedValue<F>; 6] = assigned_inputs.try_into().unwrap();
            let curr_modulus = fe_to_modulus(ctx, curr_modulus);
            
            chip.gate().crt_lookup_division_with_remainder(ctx, assigned_inputs, assigned_p, &curr_modulus);
        }   
        ctx.get(0)

    }
