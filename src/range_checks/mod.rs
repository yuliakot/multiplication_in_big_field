#![allow(unused_imports)]
#![allow(unused_variables)]

use itertools::izip;
use num_bigint::BigUint;

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


use crate::{crt_int::{CRTint, biguint_into_crtint}, 
            multiplication_gates::crt_lookup,
            moduli_precomuted::fe_to_modulus};



// borrowed from https://github.com/axiom-crypto/halo2-lib/blob/402dac423da4259f97788827848ee0d9443454d2/halo2-base/src/gates/range.rs#L457
// fn compute_limb_bases<F: ScalarField>(limb_bits: &u32, num_bases: &usize) -> Vec<QuantumCell<F>>
// {
//     let limb_base = F::from(1u64 << limb_bits);
//     let mut running_base = limb_base;
//     let mut limb_bases = Vec::with_capacity(num_bases + 1);
//     limb_bases.extend([Constant(F::one()), Constant(running_base)]);
//     for _ in 2..= *num_bases {
//         running_base *= &limb_base;
//         limb_bases.push(Constant(running_base));
//     }
//     limb_bases
// }

//fn verify_limbs<F: ScalarField>
//    (
//        chip: RangeChip<F>, 
//        ctx: &mut Context<F>, 
//        a: AssignedValue<F>, 
//        a_limbs: &Vec<AssignedValue<F>>,
//        limb_bits: &usize,
//    )
    // {
    //     let limb_bases = compute_limb_bases(limb_bits, a_limbs.len());
    //     let a_from_bits = chip.gate().inner_product(ctx, a_limbs, limb_bases);
    //     ctx.constrain_equal(&a, &a_from_bits);
    // }

fn load_limbs<F: ScalarField>
    (
        chip: RangeChip<F>, 
        ctx: &mut Context<F>, 
        a: &CRTint<F>,
    ) -> Vec<AssignedValue<F>>
    {
        ctx.assign_witnesses(a.limbs_as_fe)
    }


pub fn check_big_less_than_p<F:ScalarField>
    (
        chip: RangeChip<F>, 
        ctx: &mut Context<F>, 
        a: &CRTint<F>,
        p: &CRTint<F>,
    )
    {
        // here i am assuming only 2 limbs in both a and p
        assert_eq!(a.limbs_as_fe.len(), p.limbs_as_fe.len());
        assert_eq!(2, p.limbs_as_fe.len());
        let num_bits = 128;
        
        let ([a0, a1], [p0, p1]) = (a.limbs_as_fe, p.limbs_as_fe);

        let ([a0_assigned, a1_assigned], [p0_assigned, p1_assigned]) = 
                        ([ctx.load_witness(a0), ctx.load_witness(a1)], [ctx.load_constant(p0), ctx.load_constant(p1)]);

        // need to check that                 
        // either a1 < p1 or a1 == p1, and then we need another range check

//        chip.check_less_than(ctx, a0, , num_bits)
        todo!();


    }

pub fn check_254_bits<F:ScalarField>
    (
        chip: RangeChip<F>, 
        ctx: &mut Context<F>, 
        a: &CRTint<F>, 
    )
    {
        let [a0, a1] = a.limbs_as_fe;
        let [a0_assigned, a1_assigned] = [ctx.load_witness(a0), ctx.load_witness(a1)];






    }

    

pub fn check_big_range<F:ScalarField>
    (
        chip: RangeChip<F>, 
        ctx: &mut Context<F>, 
        a: &CRTint<F>, 
        bits: &usize
    )
    {
        let mut bits = *bits;
        let mut a_limbs = a.limbs_as_fe.iter();
        //let mut assigned_limbs : Vec::<AssignedValue<F>> = vec![];

        while let Some(curr_limb) = a_limbs.next(){
            let assigned_curr_limb = ctx.load_witness(*curr_limb);
            //assigned_limbs.push(assigned_curr_limb);
            chip.range_check(ctx, assigned_curr_limb, a.limb_bits.min(bits));
            bits = 0.max(a.limb_bits - bits);
        }
    }


