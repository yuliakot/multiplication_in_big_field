use super::*;

use serde::{Deserialize, Serialize};
use ark_std::fs::File;
use test_case::test_case;
use num_bigint::BigUint;
use halo2_base::utils::biguint_to_fe;


use super::mod_p_verifications::mod_r_mul;
use crate::moduli_precomuted::{biguint_to_modulus, fe_to_modulus, Modulus};

use halo2_base::halo2_proofs::dev::MockProver;

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

use super::crt_lookup::CQLookupGateChip;

//#[should_panic(expected = "")]
//#[test_case([5, 10, 1].map(&Fr::from) => Fr::from(0); "5 + 10 == 1 mod 7 but panic")]
//#[test_case([5, 5, 10].map(&Fr::from) => Fr::from(0); "5 + 5 != 10 mod 7")]
//#[test_case([5, 1, 6].map(&Fr::from) => Fr::from(1); "5 + 1 == 6 mod 7")]
#[test_case([5, 5, 3].map(&Fr::from), BigUint::from(7u64) => Fr::from(1); "5 + 5 == 3 mod 7")]

fn test_crt_other_moduli_add(inputs: [Fr; 3], modulus: BigUint) -> Fr{
    let k = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip: GateChip<Fr> = GateChip::default();
    let ctx = builder.main(0);
    
    let assigned_inputs = ctx.assign_witnesses(inputs);

    let [a, b, a_plus_b]: [AssignedValue<Fr> ; 3] = assigned_inputs.try_into().unwrap();
    
    let modulus_assigned = ctx.load_constant(biguint_to_fe(&modulus));

    let res = chip.crt_lookup_add(ctx, a, b, a_plus_b, &modulus, modulus_assigned);
    builder.config(k, Some(11));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    *res.value()
}