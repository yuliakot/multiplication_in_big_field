use halo2_base::gates::flex_gate::FlexGateConfig;
use halo2_proofs_axiom::dev::metadata::Gate;

use super::* ;

pub trait FLGateChip<F: ScalarField>{
    fn crt_lookup_mul(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        a_times_b: impl Into<QuantumCell<F>>,
        modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        // should look something like 
        // let lookup_table = mul_lookup_tables[modulus];
        // vec![a, b, a_times_b]        
        //ctx.get(0)
        unimplemented!()
    }

    fn crt_lookup_add(
        &self,
        ctx: &mut Context<F>,
        a: impl Into<QuantumCell<F>>,
        b: impl Into<QuantumCell<F>>,
        a_plus_b: impl Into<QuantumCell<F>>,
        modulus: impl Into<QuantumCell<F>>,) -> AssignedValue<F>
    {
        
        unimplemented!()
        //ctx.get(0)
    }

    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        inputs : [impl Into<QuantumCell<F>>; 7],
        modulus: F,) -> AssignedValue<F>;
}

impl<F: ScalarField> FLGateChip<F> for GateChip<F>{
    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        [a,
        b, 
        a_times_b, 
        p, 
        q, 
        p_times_q, 
        r] : [impl Into<QuantumCell<F>>; 7],
        modulus: F,) -> AssignedValue<F>
    {
        //self.crt_lookup_mul(ctx, a, b, a_times_b, modulus);
        //self.crt_lookup_mul(ctx, p, q, p_times_q, modulus);
        //self.crt_lookup_add(ctx, p_times_q, r, a_times_b, modulus);
        //ctx.get(0)
        unimplemented!()
    }
}
