use super::* ;
use std::collections::HashMap;

pub fn mod_r_mul<F: ScalarField>(chip: &GateChip<F>, ctx: &mut Context<F>, inputs: &[F; 3], p: impl Into<QuantumCell<F>>) -> AssignedValue<F> {
    let [a, b, q]: [_; 3] = ctx.assign_witnesses(*inputs).try_into().unwrap();

    let a_times_b = chip.mul(ctx, a, b);
    let p_times_q = chip.mul(ctx, p, q);
    let res = chip.sub(ctx, Existing(a_times_b), Existing(p_times_q));
    res
}   