#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod multiplication_gates;
pub mod crt_int;
pub mod range_checks;


use crt_int::biguint_into_crtint_bui_modulus;
use itertools::izip;
use num_bigint::BigUint;
use num_integer::Integer;
use std::convert::TryInto;


use multiplication_gates::{mod_r_verifications::*, crt_lookup::CQLookupGateChip};
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

use crate::{crt_int::{CRTint, biguint_into_crtint_fe_modulus}, 
            multiplication_gates::{crt_lookup, crt_to_bits_proof::BITStoCRT},
            range_checks::{check_big_less_than_p, check_big_less_than_p_minus_one}
        };

 fn zip_residues_batch<F:ScalarField>(
    inputs: [&CRTint<F>; 3],
    crt_p: &CRTint<F>,
    moduli: &Vec<BigUint>) -> Vec<([F; 3], F, BigUint)>
 {
    let v: [Vec<F>; 3] = inputs.map(|x| x.residues.clone());
    let zipped_v: Vec::<[F; 3]> = 
        (0..v[0].len())
            .map(|i| TryInto::<[F; 3]>::try_into(
                    v.iter().map(|inner| inner[i].clone()).collect::<Vec::<F>>())
                    .unwrap())
            .collect();
    (0..v[0].len())
        .map(|i| (zipped_v[i], crt_p.residues[i], moduli[i].clone()))
        .collect()
 }


pub fn crt_mul<F: ScalarField>(
    chip: &RangeChip<F>,
    ctx: &mut Context<F>,
    a: &BigUint,
    b: &BigUint,
    crt_p: &CRTint<F>,
    moduli: &Vec<BigUint>) -> [AssignedValue<F>; 2]
    {

        //Step 1: finding q, r, a_times_b = a*b = p*q + r; p_times_q
        let p = crt_p.value.clone();
        let a_times_b = a*b;
        let (q, r) = a_times_b.div_rem(&p);

        //Step2
        
        let [a,b, q, r] = 
            [&a,&b, &q, &r].map(|x| biguint_into_crtint_bui_modulus(x, moduli));
                        
        //Steps 3-4: range checks:

        let [bits_a, bits_b, bits_q, bits_r]: [[AssignedValue<F>; 2]; 4] = 
            [&a, &b, &q, &r].map(|x| ctx.assign_witnesses(x.limbs_as_fe).try_into().unwrap());
        let bits_p: [AssignedValue<F>; 2] = ctx.assign_witnesses(crt_p.limbs_as_fe).try_into().unwrap();
        check_big_less_than_p(&chip, ctx, bits_a, bits_p);
        check_big_less_than_p(&chip, ctx, bits_b, bits_p);
        check_big_less_than_p_minus_one(&chip, ctx, bits_q, bits_p);
        check_big_less_than_p(&chip, ctx, bits_r, bits_p);

        //Step 5: native operations
        let p_mod_n = ctx.load_constant(crt_p.residue_mod_n);
        
        let mod_n_inputs = [a.residue_mod_n,
                                    b.residue_mod_n, 
                                    q.residue_mod_n,
                                    ];

        mod_r_mul(&chip.gate(), ctx, &mod_n_inputs, p_mod_n);

        //Steps 6: CRT operations

        let zipped_inputs = 
        zip_residues_batch([&a,&b, &q], &crt_p, moduli);

        for (curr_inputs, curr_p, curr_modulus) in zipped_inputs.iter()
        {
           let assigned_inputs = ctx.assign_witnesses(*curr_inputs);
           let assigned_p = ctx.load_constant(*curr_p);
           let assigned_inputs: [AssignedValue<F>; 3] = assigned_inputs.try_into().unwrap();
           let curr_modulus_assigned = ctx.load_constant(biguint_to_fe(&curr_modulus));
            
           let r_residue = chip.gate().crt_lookup_division_with_remainder(ctx, assigned_inputs, assigned_p, curr_modulus_assigned);

           // CRT proof: for each modulus and each number, we check that we picked the right remainder
           for (&limbs, residue) in vec![bits_a, bits_b, bits_q].iter().zip(assigned_inputs){
               chip.bits_to_residue_constrain(ctx, limbs, curr_modulus_assigned, residue,14);
           }
           chip.bits_to_residue_constrain(ctx, bits_r, curr_modulus_assigned, r_residue,14);
        }
        bits_r
    }
