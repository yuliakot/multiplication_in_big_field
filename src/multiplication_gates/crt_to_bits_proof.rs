use std::str::SplitAsciiWhitespace;

use halo2_proofs_axiom::dev::metadata::Gate;
use num_bigint::BigUint;

use crate::range_checks::check_big_less_than_p;

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
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;

    fn find_small_residue(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;

    fn bits_to_residue_find(
        &self,
        ctx: &mut Context<F>,
        limbs: [impl Into<QuantumCell<F>> + Copy;2],
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>;

    
    fn bits_to_residue_constrain(
        &self,
        ctx: &mut Context<F>,
        limbs: [impl Into<QuantumCell<F>> + Copy;2],
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        residue: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    );
        
    fn bits_to_crt_check(
        &self, 
        ctx: &mut Context<F>, 
        input: [impl Into<QuantumCell<F>> + Copy;2],
        crt_residues: &Vec<impl Into<QuantumCell<F>> + Copy>, 
        moduli: Vec<AssignedValue<F>>,
        bits: usize,  
    );
}

impl<F:ScalarField> BITStoCRT<F> for RangeChip<F> {

// we prove the bits-to-crt transformation

    fn bits_to_crt_check(
        &self, 
        ctx: &mut Context<F>, 
        limbs: [impl Into<QuantumCell<F>> + Copy;2],
        crt_residues: &Vec<impl Into<QuantumCell<F>> + Copy>, 
        moduli: Vec<AssignedValue<F>>,
        bits: usize,
    ){
        let result = ctx.load_constant(F::one());
        let mod_res = crt_residues.iter().zip(moduli);
        for (residue, modulus) in mod_res{
            println!("{:?}, {:?}", Into::<QuantumCell<F>>::into(*residue).value(), modulus.value());
            self.check_less_than(ctx, Into::<QuantumCell<F>>::into(*residue), modulus, bits);
            let curr_residue = self.bits_to_residue_find(ctx, limbs, modulus, bits);
            //constraints computed remainder == provided remainder
            ctx.assign_region([curr_residue.into(), Constant(F::zero()), Constant(F::zero()), (*residue).into()], [0])
        }

    }

//https://github.com/axiom-crypto/halo2-lib/blob/f2eacb1f7fdbb760213cf8037a1bd1a10672133f/halo2-base/src/gates/range.rs#L311    
    fn find_pow_two_residue(
        &self,
        ctx: &mut Context<F>,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>
    {
        let modulus = fe_to_biguint(modulus_assigned.into().value());
        let pow_of_two = pow_of_two();
        let (div, pow_of_two_rem) = &pow_of_two.div_rem(&modulus);
        let pow_of_two_rem = ctx.load_witness(biguint_to_fe(&pow_of_two));

        // Constrain that remainder is less than divisor (i.e. `r < b`).
        pow_of_two_rem
    }
    
    fn find_small_residue(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ) -> AssignedValue<F>    {
        let modulus = fe_to_biguint(modulus_assigned.into().value());
        let a_biguint = fe_to_biguint(a.into().value());
        let (div, a_rem) = &a_biguint.div_rem(&modulus);
        ctx.assign_region(
            [Witness(biguint_to_fe(&a_rem)), 
            Constant(biguint_to_fe(&modulus)), 
            Witness(biguint_to_fe(div)), 
            a.into()], [0]);
    
        let a_rem = ctx.get(-4);

        // Constrain that remainder is less than divisor (i.e. `r < b`).
        println!("{:?}, {:?}", a_rem.value(), modulus_assigned.into().value());
        self.check_less_than(ctx, a_rem, modulus_assigned, 14);
        a_rem
    }

        
    fn bits_to_residue_find(
            &self,
            ctx: &mut Context<F>,
            [a0, a1]: [impl Into<QuantumCell<F>> + Copy;2],
            modulus_assigned: impl Into<QuantumCell<F>> + Copy,
            bits: usize,
        ) -> AssignedValue<F>
    {
        let modulus_biguint = fe_to_biguint(modulus_assigned.into().value());
        let pow_of_two = pow_of_two();
        let pow_of_two = self.find_pow_two_residue(ctx, modulus_assigned, bits);
        let [a0, a1] = [self.find_small_residue(ctx, a0, modulus_assigned, bits), self.find_small_residue(ctx, a1, modulus_assigned, bits)];
        let a1_times_pow_of_two = self.gate().crt_lookup_mul(ctx, a1, pow_of_two, modulus_assigned);
        self.gate().crt_lookup_add(ctx, a1_times_pow_of_two, a0, modulus_assigned)
    }

    fn bits_to_residue_constrain(
        &self,
        ctx: &mut Context<F>,
        [a0, a1]: [impl Into<QuantumCell<F>> + Copy;2],
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
        residue: impl Into<QuantumCell<F>> + Copy,
        bits: usize,
    ){
        let modulus_biguint = fe_to_biguint(modulus_assigned.into().value());
        let pow_of_two = pow_of_two();
        let pow_of_two = self.find_pow_two_residue(ctx, modulus_assigned, bits);
        let [a0, a1] = [self.find_small_residue(ctx, a0, modulus_assigned, bits), self.find_small_residue(ctx, a1, modulus_assigned, bits)];
        let a1_times_pow_of_two = self.gate().crt_lookup_mul(ctx, a1, pow_of_two, modulus_assigned);
        self.gate().crt_lookup_add_constrain(ctx, a1_times_pow_of_two, a0, residue, modulus_assigned)

    }

}