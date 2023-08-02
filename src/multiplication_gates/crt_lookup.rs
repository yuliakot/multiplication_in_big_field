use halo2_base::{gates::flex_gate::FlexGateConfig, utils::fe_to_bigint};
use halo2_proofs_axiom::dev::metadata::Gate;
use halo2_base::utils::fe_to_biguint ;
use num_bigint::BigUint;

use super::* ;

pub trait FLGateChip<F: ScalarField>{
    fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        a_times_b: impl Into<QuantumCell<F>>,
        modulus: F,) -> AssignedValue<F>
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
        a: AssignedValue<F>,
        b: AssignedValue<F>,
        a_plus_b: AssignedValue<F>,
        modulus: F,) -> AssignedValue<F>;

    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        inputs : [AssignedValue<F>; 6],
        p: AssignedValue<F>,
        modulus: F,) -> AssignedValue<F>;
}

impl<F: ScalarField> FLGateChip<F> for GateChip<F>{
    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: AssignedValue<F>,
        b: AssignedValue<F>,
        a_plus_b: AssignedValue<F>,
        modulus: F,) -> AssignedValue<F>
    {
        let true_a_plus_b = self.add(ctx, a, b);
        let diff = self.sub(ctx, true_a_plus_b, a_plus_b);
        let diff_is_zero = self.is_equal(ctx, diff, Constant(F::zero()));
        let diff_is_modulus = self.is_equal(ctx, diff, Constant(modulus));
        self.or(ctx, diff_is_zero, diff_is_modulus)
    }


    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        [a,
        b, 
        a_times_b, 
        q, 
        p_times_q, 
        r] : [AssignedValue<F>; 6],
        p: AssignedValue<F>,
        modulus: F,) -> AssignedValue<F>
    {
        let res1 = self.crt_lookup_mul(ctx, a, b, a_times_b, modulus);
        let res2 = self.crt_lookup_mul(ctx, p, q, p_times_q, modulus);
        let res3 = self.crt_lookup_add(ctx, p_times_q, r, a_times_b, modulus);
        let res4 = self.and(ctx, res1, res2);
        let res5 = self.and(ctx, res4, res3);

        println!("\nmodulus = {:?}, \na = {:?}, \nb = {:?}, \np = {:?}, \nq = {:?}, \nr = {:?}",  fe_to_bigint(&modulus), &a.value,  (&b.value),  (&p.value),  (&q.value),  (&r.value));
        
        res5
    }
}
