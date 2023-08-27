use halo2_base::{gates::flex_gate::FlexGateConfig, utils::{fe_to_bigint, biguint_to_fe}};
use halo2_proofs_axiom::dev::metadata::Gate;
use halo2_base::utils::fe_to_biguint ;
use num_bigint::BigUint;
use num_integer::Integer;



use super::* ;

pub trait CQLookupGateChip<F: ScalarField>{
    // returns modulus - a
    fn crt_neg(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,) -> AssignedValue<F>;


    // returns a_times_b mod modulus where if a * b = a_times_b mod modulus
    fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        b: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,) -> AssignedValue<F>
    {
            //unimplemented!()
            //for now doesn't constrain anything 
            //for testing purposes   
            let modulus = fe_to_biguint(modulus_assigned.into().value());
            let a_bui = fe_to_biguint(a.into().value());
            let b_bui = fe_to_biguint(b.into().value());
            let a_times_b = &(a_bui*b_bui).div_rem(&modulus).1;
            ctx.load_witness(biguint_to_fe(a_times_b))
        }



    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        b:  impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
    ) -> AssignedValue<F>;

    
    fn crt_lookup_add_constrain(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        b: impl Into<QuantumCell<F>> + Copy,
        a_plus_b: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
    );


    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        inputs : [impl Into<QuantumCell<F>> + Copy; 3],
        p: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
    ) -> AssignedValue<F>;
}

impl<F: ScalarField> CQLookupGateChip<F> for GateChip<F>{
        // returns modulus-a
        fn crt_neg(
            &self,
            ctx: &mut Context<F>,
            a: impl Into<QuantumCell<F>> + Copy,
            modulus_assigned: impl Into<QuantumCell<F>> + Copy,) -> AssignedValue<F>
        {
            //unimplemented!()
            //for now doesn't constrain anything 
            //for testing purposes
            self.sub(ctx, modulus_assigned, a)
        }

    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        b: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
    ) -> AssignedValue<F>
    {
        // we range check when we verify the crt representations
        let modulus = fe_to_biguint(modulus_assigned.into().value());

        let a = a.into();
        let b = b.into();
        
        let a_bui = fe_to_biguint(a.value());
        let b_bui = fe_to_biguint(b.value());
        let out_value = if &a_bui + &b_bui < modulus
            {a_bui + b_bui}
        else{a_bui + b_bui - modulus};

        let out = ctx.load_witness(biguint_to_fe(&out_value));
        let true_a_plus_b = self.add(ctx, a, b);

        let diff = self.sub(ctx, Existing(true_a_plus_b), out.clone());
        let diff_minus_p = self.sub(ctx, diff, modulus_assigned);
        let zero = self.mul(ctx, diff, diff_minus_p);
        self.assert_is_const(ctx, &zero, &F::zero());
        
        out
        
    }


    fn crt_lookup_add_constrain(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>> + Copy,
        b: impl Into<QuantumCell<F>> + Copy,
        a_plus_b: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,
    )
    {
        // we range check when we verify the crt representations
        let compute_a_plus_b = self.crt_lookup_add(ctx, a, b, modulus_assigned);
        ctx.assign_region([Existing(compute_a_plus_b), Constant(F::zero()), Constant(F::zero()), a_plus_b.into()], []);
    }



    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        [a,
        b, 
        q] : [impl Into<QuantumCell<F>> + Copy; 3],
        p: impl Into<QuantumCell<F>> + Copy,
        modulus_assigned: impl Into<QuantumCell<F>> + Copy,) -> AssignedValue<F>
    {
        let moduls = fe_to_biguint(modulus_assigned.into().value());

        let a_times_b = self.crt_lookup_mul(ctx, a, b, modulus_assigned);
        let p_times_q = self.crt_lookup_mul(ctx, p, q, modulus_assigned);
        let p_times_q_neg = self.crt_neg(ctx, p_times_q, modulus_assigned);
        self.crt_lookup_add(ctx, a_times_b, p_times_q_neg, modulus_assigned)
    }
}