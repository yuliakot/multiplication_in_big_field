use super::* ;
use std::collections::HashMap;

pub fn mod_r_mul<F: ScalarField>(chip: &GateChip<F>, ctx: &mut Context<F>, inputs: &[F; 4], p: impl Into<QuantumCell<F>>) -> AssignedValue<F> {
    let [a, b, q, r]: [_; 4] = ctx.assign_witnesses(*inputs).try_into().unwrap();

    let a_times_b = chip.mul(ctx, a, b);
    let p_times_q_plus_r = chip.mul_add(ctx, p, q, r);
    let res = chip.is_equal(ctx, Existing(p_times_q_plus_r), Existing(a_times_b));
    res
}   