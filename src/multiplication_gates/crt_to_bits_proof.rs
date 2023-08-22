use std::str::SplitAsciiWhitespace;

use halo2_proofs_axiom::dev::metadata::Gate;
use num_bigint::BigUint;

use crate::{moduli_precomuted::Modulus, range_checks::check_big_less_than_p};

use super::* ;
use super::crt_lookup::CQLookupGateChip;
use halo2_base::{
    halo2_proofs::halo2curves::bn256::Fr,
    utils::{ScalarField, fe_to_biguint, biguint_to_fe},
    gates::{
        GateChip,
        GateInstructions,
        RangeChip, 
        RangeInstructions
    },
    AssignedValue,
    Context,
    QuantumCell::{self, Constant, Existing, Witness},
};
use num_traits::identities::One;
use num_integer::Integer;

pub fn pow_of_two() -> BigUint{
    BigUint::from(std::u128::MAX) + BigUint::one()
}


pub trait BITStoCRT<F: ScalarField> {
    fn find_pow_two_residue(
        &self,
        ctx: &mut Context<F>,
        modulus: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;

    fn find_small_residue(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        modulus: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;

    fn bits_to_residue_proof(
        &self,
        ctx: &mut Context<F>,
        limbs: [impl Into<QuantumCell<F>> + Copy;2],
        modulus: impl Into<QuantumCell<F>> + Copy,
        residue: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;
        
    fn bits_to_crt_proof(
        &self, 
        ctx: &mut Context<F>, 
        input: [impl Into<QuantumCell<F>> + Copy;2],
        crt_residues: &Vec<impl Into<QuantumCell<F>> + Copy>, 
        moduli: Vec<AssignedValue<F>>,
        bits: usize,  
    )-> AssignedValue<F>;
}

impl<F:ScalarField> BITStoCRT<F> for RangeChip<F> {

// we prove the bits-to-crt transformation

    fn bits_to_crt_proof(
        &self, 
        ctx: &mut Context<F>, 
        limbs: [impl Into<QuantumCell<F>> + Copy;2],
        crt_residues: &Vec<impl Into<QuantumCell<F>> + Copy>, 
        moduli: Vec<AssignedValue<F>>,
        bits: usize,
    ) -> AssignedValue<F>
    {
        let numbits = 128;
        let result = ctx.load_constant(F::one());
        let mod_res = crt_residues.iter().zip(moduli);
        for (residue, modulus) in mod_res{
            self.check_less_than(ctx, *residue, modulus, bits);
            let curr_residue = self.bits_to_residue_proof(ctx, limbs, *residue, modulus, bits);
            let result = self.gate().and(ctx, curr_residue, result);
        }
        result


    }

//https://github.com/axiom-crypto/halo2-lib/blob/f2eacb1f7fdbb760213cf8037a1bd1a10672133f/halo2-base/src/gates/range.rs#L311    
    fn find_pow_two_residue(
        &self,
        ctx: &mut Context<F>,
        modulus: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>
    {
        let modulus_bigint = fe_to_biguint(modulus.into().value());
        let pow_of_two = pow_of_two();
        let (div, pow_of_two_rem) = &pow_of_two.div_rem(&modulus_bigint);
        ctx.assign_region(
            [Witness(biguint_to_fe(&pow_of_two_rem)), 
            Constant(biguint_to_fe(&modulus_bigint)), 
            Witness(biguint_to_fe(div)), 
            Constant(biguint_to_fe(&pow_of_two))], [0]);
    
        let pow_of_two_rem = ctx.get(-4);

        // Constrain that remainder is less than divisor (i.e. `r < b`).
        self.check_less_than(ctx, pow_of_two_rem, modulus, bits);
        pow_of_two_rem
    }
    
    fn find_small_residue(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        modulus: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>    {
        let modulus_bigint = fe_to_biguint(modulus.into().value());
        let a_biguint = fe_to_biguint(a.into().value());
        let (div, a_rem) = &a_biguint.div_rem(&modulus_bigint);
        ctx.assign_region(
            [Witness(biguint_to_fe(&a_rem)), 
            Constant(biguint_to_fe(&modulus_bigint)), 
            Witness(biguint_to_fe(div)), 
            a.into()], [0]);
    
        let a_rem = ctx.get(-4);

        // Constrain that remainder is less than divisor (i.e. `r < b`).
        self.check_less_than(ctx, a_rem, modulus, bits);
        a_rem
    }

    
fn bits_to_residue_proof(
        &self,
        ctx: &mut Context<F>,
        [a0, a1]: [impl Into<QuantumCell<F>> + Copy;2],
        modulus: impl Into<QuantumCell<F>> + Copy,
        residue: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>
{
    let modulus_biguint = fe_to_biguint(modulus.into().value());
    let pow_of_two = pow_of_two();
    let (_, a1_times_pow_of_two) = (fe_to_biguint(a1.into().value()) * (&pow_of_two)).div_rem(&modulus_biguint);
    let a1_times_pow_of_two = ctx.load_witness(biguint_to_fe(&a1_times_pow_of_two));
    let pow_of_two = self.find_pow_two_residue(ctx, modulus, bits);
    let [a0, a1] = [self.find_small_residue(ctx, a0, modulus, bits), self.find_small_residue(ctx, a1, modulus, bits)];
    let check_multiplication = self.gate().crt_lookup_mul(ctx, a1, pow_of_two, a1_times_pow_of_two, &modulus_biguint);
    let check_addition = self.gate().crt_lookup_add(ctx, a1_times_pow_of_two, a0, residue, &modulus_biguint, modulus);
    self.gate().and(ctx, check_addition, check_multiplication)
}
}