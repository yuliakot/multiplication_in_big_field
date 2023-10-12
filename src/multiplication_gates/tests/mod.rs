use super::*;

use ark_std::fs::File;
use test_case::test_case;
use num_bigint::BigUint;
use halo2_base::utils::{biguint_to_fe, fe_to_biguint};
use ark_std::env::set_var;


use super::mod_r_verifications::mod_r_mul;
use super::crt_to_bits_proof::BITStoCRT;
use super::crt_lookup::CQLookupGateChip;
use num_integer::Integer;

use crate::crt_int::limb_bits_to_crt;
use crate::utils::NUMBER_OF_TABLES;

use halo2_base::halo2_proofs::dev::MockProver;

fn read_inputs_crt_mod_p_mul(i: i32) -> [u64; 3]{
    let path = format!("src/multiplication_gates/tests/tests_input{i}.in");
    serde_json::from_reader(
        File::open(path).unwrap_or_else(|e| panic!("Something went wrong parcing the inputs: {}", e)),
    )
    .unwrap()
}

#[test_case(1)]//, b"1*2 != 3*p + 4")]
#[test_case(2)]//, b"5*6 == 4*p + 2")]

fn test_crt_mod_p_mul(i: i32){
    let k = 6;
    let mut builder = GateThreadBuilder::new(false);
    let chip = GateChip::default();
    let ctx = builder.main(0);

    let p = Fr::from(7);
    let p = ctx.load_constant(p);

    let inputs = read_inputs_crt_mod_p_mul(i).map(Fr::from);

    let expected_result = inputs[0]*inputs[1] - inputs[2]*p.value();
    
    let res = mod_r_mul(&chip, ctx, &inputs, p);

    builder.config(k, Some(9));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    assert_eq!(res.value(), &expected_result);

}


#[test_case([5, 5].map(&Fr::from), &BigUint::from(7u64); "5 + 5 == 3 mod 7")]
#[test_case([5, 3].map(&Fr::from), &BigUint::from(7u64))]
#[test_case([1, 5].map(&Fr::from), &BigUint::from(6u64))]
#[test_case([200, 200].map(&Fr::from), &BigUint::from(391u64))]

fn test_crt_other_moduli_add(inputs: [Fr; 2], modulus: &BigUint){
    let k = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip: GateChip<Fr> = GateChip::default();
    let ctx = builder.main(0);

    let expected_result = (fe_to_biguint(&inputs[0])+fe_to_biguint(&inputs[1])).div_rem(modulus).1;
    
    let assigned_inputs = ctx.assign_witnesses(inputs);

    let [a, b]: [AssignedValue<Fr> ; 2] = assigned_inputs.try_into().unwrap();
    
    let modulus_assigned = ctx.load_constant(biguint_to_fe(&modulus));

    let res = chip.crt_lookup_add(ctx, a, b, modulus_assigned);
    builder.config(k, Some(11));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();

    assert_eq!(res.value(), &biguint_to_fe(&expected_result));
}

#[test_case([5, 5].map(&Fr::from), &BigUint::from(7u64); "5 + 5 == 3 mod 7")]
#[test_case([5, 3].map(&Fr::from), &BigUint::from(7u64))]
#[test_case([1, 5].map(&Fr::from), &BigUint::from(6u64))]
#[test_case([200, 301].map(&Fr::from), &BigUint::from(391u64))]
fn test_crt_other_moduli_add_constrain(inputs: [Fr; 2], modulus: &BigUint){
    let k = 10;
    let mut builder = GateThreadBuilder::new(false);
    let chip: GateChip<Fr> = GateChip::default();
    let ctx = builder.main(0);

    let expected_result = (fe_to_biguint(&inputs[0])+fe_to_biguint(&inputs[1])).div_rem(modulus).1;
    let assigned_expected_result = ctx.load_witness(biguint_to_fe(&expected_result));
    
    let assigned_inputs = ctx.assign_witnesses(inputs);

    let [a, b]: [AssignedValue<Fr> ; 2] = assigned_inputs.try_into().unwrap();
    
    let modulus_assigned = ctx.load_constant(biguint_to_fe(&modulus));

    let res = chip.crt_lookup_add_constrain(ctx, a, b, assigned_expected_result, modulus_assigned);
    builder.config(k, Some(11));
    
    let circuit = GateCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
}


#[test_case([5, 5].map(&Fr::from), [2, 3, 5, 7, 11].map(&Fr::from); "5*2^{128} + 5")]
#[test_case([5, 3].map(&Fr::from), [3, 5, 7, 11, 13].map(&Fr::from))]
#[test_case([1, 5].map(&Fr::from), [5, 7, 11, 13, 17].map(&Fr::from))]
#[test_case([200, 301].map(&Fr::from), [11, 13, 17, 21, 23].map(&Fr::from))]

fn test_crt_to_bits(inputs: [Fr; 2], moduli: [Fr; 5]){
    let k = 12;
    let mut builder = GateThreadBuilder::new(false);
    let lookup_bits = 10;
    let chip: RangeChip<Fr> = RangeChip::default(lookup_bits);
    set_var("LOOKUP_BITS", lookup_bits.to_string());
    let ctx = builder.main(0);

    let residues = limb_bits_to_crt([&inputs[0], &inputs[1]], &moduli.to_vec()).residues;
    
    let assigned_inputs: [AssignedValue<Fr>; 2] = ctx.assign_witnesses(inputs).try_into().unwrap();
    let assigned_moduli: Vec<AssignedValue<Fr>> = moduli.map(|x| ctx.load_constant(x)).to_vec();
    let assigned_residues: Vec<AssignedValue<Fr>> = residues.iter().map(|x| ctx.load_constant(*x)).collect();

    let twelve = ctx.load_witness(Fr::from(18));
    let m = ctx.load_witness(Fr::from(21));

    let mut cells_to_lookup = vec![];
    for _ in 0..NUMBER_OF_TABLES{
        cells_to_lookup.push(vec![]);
    }    
    let mut cells_to_lookup = cells_to_lookup.try_into().unwrap();

    chip.bits_to_crt_check(ctx, &mut cells_to_lookup, assigned_inputs, &assigned_residues,assigned_moduli, 9);

    
    builder.config(k, Some(9));
    
    let circuit = RangeCircuitBuilder::mock(builder);

    MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
}