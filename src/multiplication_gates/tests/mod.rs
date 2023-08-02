use super::*;

use serde::{Deserialize, Serialize};
use ark_std::fs::File;
use test_case::test_case;

use super::mod_p_verifications::mod_r_mul;


fn read_inputs_crt_mod_p_mul(i: i32) -> [u64; 4]{
    let path = format!("src/multiplication_gates/tests/tests_input{i}.in");
    serde_json::from_reader(
        File::open(path).unwrap_or_else(|e| panic!("Something went wrong parcing the inputs: {}", e)),
    )
    .unwrap()
}

#[test_case(1 => Fr::from(0))]//, b"1*2 != 3*p + 4")]
#[test_case(2 => Fr::from(1))]//, b"5*6 == 4*p + 2")]

fn test_crt_mod_p_mul(i: i32) -> Fr{
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let chip = GateChip::default();
    let ctx = builder.main(0);

    let p = Fr::from(7);
    let p = ctx.load_constant(p);

    let inputs = read_inputs_crt_mod_p_mul(i).map(Fr::from);
    
    let res = mod_r_mul(&chip, ctx,  &inputs, p);

    builder.config(k, Some(9));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    *res.value()
}


use super::crt_lookup::FLGateChip;


#[test_case(1 => Fr::from(1))]//, b"1*2 != 3*p + 4")]
#[test_case(2 => Fr::from(1))]//, b"5*6 == 4*p + 2")]

fn test_crt_other_moduli_mul(i: i32) -> Fr{
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let chip: GateChip<Fr> = GateChip::default();
    let ctx = builder.main(0);

    let p = Fr::from(7);
    //let p = ctx.load_constant(p);

    let inputs = read_inputs_crt_mod_p_mul(i).map(Fr::from);
    //let inputs = [];
    let assigned_inputs = ctx.assign_witnesses(inputs);
    
    //let res = chip.crt_lookup_add(ctx, &assigned_inputs[1], &assigned_inputs[2], &assigned_inputs[3], p);

    builder.config(k, Some(9));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    Fr::from(3)

    //*res.value()
}

