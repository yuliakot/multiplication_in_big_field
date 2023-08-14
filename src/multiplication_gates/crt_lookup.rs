use halo2_base::{gates::flex_gate::FlexGateConfig, utils::fe_to_bigint};
use halo2_proofs_axiom::dev::metadata::Gate;
use halo2_base::utils::fe_to_biguint ;
use num_bigint::BigUint;

use crate::moduli_precomuted::Modulus;

use super::* ;

pub trait FLGateChip<F: ScalarField>{
    // returns true if a * b = a_times_b mod modulus
    fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        a_times_b: impl Into<QuantumCell<F>>,
        modulus: &Modulus<F>,) -> AssignedValue<F>
    {
            //unimplemented!()
            //returns TRUE always 
            //for testing purposes
            ctx.assign_region_last([Witness(F::one())], [])
            //ctx.get(0)
    }


    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b:  impl Into<QuantumCell<F>>,
        a_plus_b:  impl Into<QuantumCell<F>> + Copy,
        modulus: &Modulus<F>,
    ) -> AssignedValue<F>;

    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        inputs : [impl Into<QuantumCell<F>> + Copy; 6],
        p: impl Into<QuantumCell<F>>,
        modulus: &Modulus<F>,
    ) -> AssignedValue<F>;
}

impl<F: ScalarField> FLGateChip<F> for GateChip<F>{

    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        a_plus_b: impl Into<QuantumCell<F>> + Copy,
        modulus: &Modulus<F>,
    ) -> AssignedValue<F>
    {
        //assert!(fe_to_bigint(a.value()) < fe_to_bigint(&modulus.value));
        //assert!(fe_to_bigint(b.value()) < fe_to_bigint(&modulus.value));


        let true_a_plus_b = self.add(ctx, a, b);
        let diff = self.sub(ctx, Existing(true_a_plus_b), a_plus_b.clone());
        let diff_is_zero = self.is_equal(ctx, Existing(diff), Constant(F::zero()));
        let diff_is_modulus = self.is_equal(ctx, Existing(diff), modulus.assigned);
        println!("diff is 0 {:?}, \ndiff is modulus {:?}",
        {if *diff_is_zero.value() == F::one() {true} else {false}}, 
        {if *diff_is_modulus.value() == F::one(){true} else{false}});
        self.or(ctx, Existing(diff_is_zero), Existing(diff_is_modulus))
        
    }


    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        [a,
        b, 
        a_times_b, 
        q, 
        p_times_q, 
        r] : [impl Into<QuantumCell<F>> + Copy; 6],
        p: impl Into<QuantumCell<F>>,
        modulus: &Modulus<F>,) -> AssignedValue<F>
    {
        let res1 = self.crt_lookup_mul(ctx, a, b, a_times_b, &modulus);
        let res2 = self.crt_lookup_mul(ctx, p, q, p_times_q, &modulus);
        let res3 = self.crt_lookup_add(ctx, p_times_q.clone(), r, a_times_b.clone(), &modulus);
        let res4 = self.and(ctx, res1, res2);
        let res5 = self.and(ctx, res4, res3);

        //println!("\nmodulus = {:?}, \na = {:?}, \nb = {:?}, \np = {:?}, \nq = {:?}, \nr = {:?}",  fe_to_bigint(&modulus), a.into().value(),  (&b.value),  (&p.value),  (&q.value),  (&r.value));
        
        res5
    }
}