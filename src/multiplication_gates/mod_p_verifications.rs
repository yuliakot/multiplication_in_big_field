use super::* ;
use std::collections::HashMap;

pub fn mod_r_mul<F: ScalarField>(chip: &GateChip<F>, ctx: &mut Context<F>, inputs: &[F; 5]) -> AssignedValue<F> {
    let [a, b, c, p, q]: [_; 5] = ctx.assign_witnesses(*inputs).try_into().unwrap();

    let ab = chip.mul(ctx, a, b);
    let pq = chip.mul(ctx, p, q);
    let pqc = chip.add(ctx, pq, c);
    let res = chip.is_equal(ctx, pqc, ab);
    res
}   