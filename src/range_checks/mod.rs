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



pub fn check_big_less_than_p<F:ScalarField>
    (
        chip: &RangeChip<F>, 
        ctx: &mut Context<F>, 
        [a0_assigned, a1_assigned]: [impl Into<QuantumCell<F>> + Copy; 2],
        [p0_assigned, p1_assigned]: [impl Into<QuantumCell<F>> + Copy; 2],
    )
    {
        let num_bits = 12;
        chip.check_less_than(ctx, Constant(F::zero()), a1_assigned, num_bits);
        chip.check_less_than(ctx, Constant(F::zero()), a0_assigned, num_bits);
        // need to check that                 
        // either a1 < p1 or a1 == p1, and then we need another range check
        

        let p1_plus_one_assigned = chip.gate().add(ctx, p1_assigned, Constant(F::one()));

        //a_1 \le p_1
        chip.check_less_than(ctx, a1_assigned, p1_plus_one_assigned, num_bits);

        //if a_1 == p_1, we want a_0 < p_O
        let a1_is_p1 = chip.gate.is_equal(ctx, a1_assigned.clone(), p1_assigned.clone());
        let need_to_check_small_bits = chip.gate().mul(ctx, Existing(a1_is_p1), a0_assigned);
        chip.check_less_than(ctx, need_to_check_small_bits, p0_assigned, num_bits);

    }


    pub fn check_big_less_than_p_minus_one<F:ScalarField>
    (
        chip: &RangeChip<F>, 
        ctx: &mut Context<F>, 
        [a0_assigned, a1_assigned]: [impl Into<QuantumCell<F>> + Copy; 2],
        [p0_assigned, p1_assigned]: [impl Into<QuantumCell<F>> + Copy; 2],
        )
    {
        let num_bits = 128;
        chip.check_less_than(ctx, Constant(F::zero()), a1_assigned, num_bits);
        chip.check_less_than(ctx, Constant(F::zero()), a0_assigned, num_bits);
        // here i am assuming only 2 limbs in both a and p
        let p1_plus_one_assigned = chip.gate().add(ctx, p1_assigned, Constant(F::one()));

        chip.check_less_than(ctx, a1_assigned, p1_plus_one_assigned, num_bits);

        let a1_is_p1 = chip.gate.is_equal(ctx, a1_assigned, p1_assigned);
        let need_to_check_small_bits = chip.gate().mul(ctx, a1_is_p1, a0_assigned);
        let p0_minus_one = chip.gate().sub(ctx, p0_assigned, Constant(F::one()));
        chip.check_less_than(ctx, need_to_check_small_bits, p0_minus_one, num_bits);

    }

#[cfg(test)]
pub mod tests;