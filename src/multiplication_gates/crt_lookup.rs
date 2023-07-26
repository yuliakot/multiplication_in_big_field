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
        modulus: F,) -> AssignedValue<F>
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
        modulus: F,) -> AssignedValue<F>
        {
            unimplemented!()
        }

    fn crt_lookup_division_with_remainder(
        &self,
        ctx: &mut Context<F>,
        inputs : [impl Into<QuantumCell<F>> + Copy; 6],
        p: impl Into<QuantumCell<F>>,
        modulus: F,) -> AssignedValue<F>;
}

impl<F: ScalarField> FLGateChip<F> for GateChip<F>{
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
        modulus: F,) -> AssignedValue<F>
    {
        self.crt_lookup_mul(ctx, a.into(), b.into(), a_times_b.into(), modulus);
        self.crt_lookup_mul(ctx, p.into(), q.into(), p_times_q.into(), modulus);
        self.crt_lookup_add(ctx, p_times_q.into().clone(), r.into(), a_times_b.into().clone(), modulus);

        //ctx.get(0)
        unimplemented!()
    }
}
