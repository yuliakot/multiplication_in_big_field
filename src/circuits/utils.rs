use super::* ;

// fn crt_lookup_mul<F: ScalarField>(ctx: &mut Context<F>, modulus: F, inputs: [F; 7]) -> AssignedValue<F>{
//     let [a, b, ab, p, q, pq, c, pqc]: [_; 7] = ctx.assign_witnesses(inputs).try_into().unwrap();
//     let chip = GateChip::default();

//     let res_ab = chip.crt_lookup_mul(ctx, modulus, a, b, ab);
//     let res_pq = chip.crt_lookup_mul(ctx, modulus, p, q, pq);
//     let res_pqc = chip.crt_lookup_add(ctx, modulus, pq, c, pqc);
// }

pub fn mod_p_mul<F: ScalarField>(ctx: &mut Context<F>, inputs: [F; 5]) -> AssignedValue<F> {
    let [a, b, c, p, q]: [_; 5] = ctx.assign_witnesses(inputs).try_into().unwrap();
    let chip = GateChip::default();

    let ab = chip.mul(ctx, a, b);
    let pq = chip.mul(ctx, p, q);
    let pqc = chip.add(ctx, pq, c);
    let res = chip.is_equal(ctx, pqc, ab);
    res
}

// fn crt_mul<F: ScalarField>(ctx: &mut Context<F>, inputs: [F; 5]) -> AssignedValue<F> {
//     (mod_p_mul_inputs, crt_form_inputs) = to_crt(inputs);

//     let mut res = mod_p_mul(ctx, mod_p_mul_inputs);

//     for crt_input in crt_form_inputs{
//         res = 
//     }

//     res
// }

